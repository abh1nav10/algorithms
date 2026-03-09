// Writing a BST using raw pointers! Making it MIRI safe and avoiding Box to prevent it from
// getting aliased which would be a lie to LLVM, making it vulnerable to optimizations that lead
// UB.

use std::cmp::Ordering;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub struct Node<T> {
    value: ManuallyDrop<T>,
    children: [Option<NonNull<Node<T>>>; 2],
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value: ManuallyDrop::new(value),
            children: std::array::from_fn(|_| None),
        }
    }
}

pub struct Tree<T> {
    root: Option<NonNull<Node<T>>>,
    length: usize,
}

impl<T> Drop for Tree<T> {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        if let Some(node) = self.root {
            stack.push(node);
        } else {
            return;
        }
        while let Some(node) = stack.pop() {
            let mut d = unsafe { Box::from_raw(node.as_ptr()) };
            if let Some(left) = d.children[0] {
                stack.push(left);
            }
            if let Some(right) = d.children[1] {
                stack.push(right);
            }
            unsafe { ManuallyDrop::drop(&mut d.value) };
            drop(d);
        }
    }
}

impl<T: Ord> Tree<T> {
    pub fn new(root: T) -> Self {
        let node = Box::into_raw(Box::new(Node::new(root)));
        Self {
            root: NonNull::new(node),
            length: 1,
        }
    }

    pub fn insert(&mut self, element: T) {
        let node = Box::into_raw(Box::new(Node::new(element)));
        let element = unsafe { &(*node).value };
        let mut current = &mut self.root;

        loop {
            if let Some(ptr) = current {
                let ptr = ptr.as_ptr();
                let n = unsafe { &mut (*ptr) };
                match element.cmp(&n.value) {
                    Ordering::Less | Ordering::Equal => {
                        current = &mut n.children[0];
                    }
                    Ordering::Greater => {
                        current = &mut n.children[1];
                    }
                }
            } else {
                *current = NonNull::new(node);
                self.length += 1;
                break;
            }
        }
    }

    pub fn delete(&mut self, element: &T) {
        let found;
        let mut parent: *mut Option<NonNull<Node<T>>> = std::ptr::null_mut();
        let mut current = &self.root;
        let ptr;
        loop {
            if let Some(node) = current {
                let p = node.as_ptr();
                let node = unsafe { &mut (*p) };
                match element.cmp(&node.value) {
                    Ordering::Less => {
                        // Raw pointers are untagged in Stacked Borrows. Therefore,
                        // at this point We use the mutable reference to create the raw
                        // pointer that accounts for its usage which pops every tag above it
                        // out of the stack which is fine.
                        parent = &mut node.children[0] as *mut _;
                        // We create a shared reference here, which pushes mutable references
                        // and even mutable raw pointers above it out of the stack which is fine
                        // because at this point there are None!
                        current = &node.children[0];
                    }
                    Ordering::Greater => {
                        parent = &mut node.children[1] as *mut _;
                        current = &node.children[1];
                    }
                    Ordering::Equal => {
                        ptr = p;
                        // We dereference again because we dont want to keep a mutable reference
                        // active for the duration of the call when there is no requirement to do
                        // so! Using p here will pop all tagged items in the stack above it but
                        // it won't pop the parent pointer that we had created which is all we
                        // need!
                        found = unsafe { &(*p) };
                        break;
                    }
                }
            } else {
                return;
            }
        }
        self.length -= 1;
        if found.children[0].is_some()
            && let Some(ref cr) = found.children[1]
        {
            let mut prev = std::ptr::null_mut();
            let mut current = cr.as_ptr();
            loop {
                let deref = unsafe { (*current).children[0] };
                if let Some(node) = deref {
                    prev = current;
                    current = node.as_ptr();
                } else {
                    break;
                }
            }
            if !prev.is_null() {
                unsafe {
                    let right = (*current).children[1];
                    // If right is None, we still want the following line of code
                    // to prevent it from pointing to dangling pointers.
                    (*prev).children[0] = right;

                    // The line of code that MIRI probably won't like!
                    // It does approve it because both pointers are pointing
                    // at different allocations!
                    std::mem::swap(&mut (*ptr).value, &mut (*current).value);

                    ManuallyDrop::drop(&mut (*current).value);

                    let _ = Box::from_raw(current);
                }
            } else {
                let cr = cr.as_ptr();
                let right = unsafe { &mut (*cr) };
                let ptr = unsafe { &mut (*ptr) };

                std::mem::swap(&mut ptr.value, &mut right.value);

                let right = right.children[1];
                ptr.children[1] = right;
                unsafe {
                    ManuallyDrop::drop(&mut (*cr).value);
                    let _ = Box::from_raw(cr);
                }
            }
        } else if found.children[0].is_some() {
            let child = found.children[0];
            if !parent.is_null() {
                unsafe {
                    (*parent) = child;
                }
            } else {
                self.root = child;
            }
            unsafe {
                ManuallyDrop::drop(&mut (*ptr).value);
                let _ = Box::from_raw(ptr);
            }
        } else if found.children[1].is_some() {
            let child = found.children[1];
            if !parent.is_null() {
                unsafe {
                    (*parent) = child;
                }
            } else {
                self.root = child;
            }
            unsafe {
                ManuallyDrop::drop(&mut (*ptr).value);
                let _ = Box::from_raw(ptr);
            }
        } else {
            if !parent.is_null() {
                unsafe {
                    (*parent) = None;
                }
            } else {
                // It is the root node!
                self.root = None;
            }
            unsafe {
                ManuallyDrop::drop(&mut (*ptr).value);
                let _ = Box::from_raw(ptr);
            }
        }
    }

    // A shared reference to ManuallyDrop cant be used to call any of its methods.
    // So returning it is fine!
    pub fn in_order_predecessor(&self, element: &T) -> Option<&ManuallyDrop<T>> {
        let mut found;
        let mut predecessor = None;
        let mut current = &self.root;
        loop {
            if let Some(node) = current {
                let node = unsafe { &(*node.as_ptr()) };
                match element.cmp(&node.value) {
                    Ordering::Less => {
                        current = &node.children[0];
                    }
                    Ordering::Greater => {
                        predecessor = Some(&node.value);
                        current = &node.children[1];
                    }
                    Ordering::Equal => {
                        found = node;
                        break;
                    }
                }
            } else {
                return None;
            }
        }
        if let Some(ref child) = found.children[0] {
            found = unsafe { &(*child.as_ptr()) };
        } else if predecessor.is_some() {
            return predecessor;
        } else {
            return None;
        }
        while let Some(ref child) = found.children[1] {
            found = unsafe { &(*child.as_ptr()) };
        }
        Some(&found.value)
    }

    pub fn in_order_successor(&self, element: &T) -> Option<&ManuallyDrop<T>> {
        let mut found;
        let mut successor = None;
        let mut current = &self.root;
        loop {
            if let Some(node) = current {
                let node = unsafe { &(*node.as_ptr()) };
                match element.cmp(&node.value) {
                    Ordering::Less => {
                        successor = Some(&node.value);
                        current = &node.children[0];
                    }
                    Ordering::Greater => {
                        current = &node.children[1];
                    }
                    Ordering::Equal => {
                        found = node;
                        break;
                    }
                }
            } else {
                return None;
            }
        }
        if let Some(ref child) = found.children[1] {
            found = unsafe { &(*child.as_ptr()) };
        } else if successor.is_some() {
            return successor;
        } else {
            return None;
        }
        while let Some(ref child) = found.children[0] {
            found = unsafe { &(*child.as_ptr()) };
        }
        Some(&found.value)
    }

    pub fn search(&self, element: &T) -> bool {
        let mut current = &self.root;
        loop {
            if let Some(node) = current {
                let node = unsafe { &(*node.as_ptr()) };
                match element.cmp(&node.value) {
                    Ordering::Less => {
                        current = &node.children[0];
                    }
                    Ordering::Greater => {
                        current = &node.children[1];
                    }
                    Ordering::Equal => break true,
                }
            } else {
                break false;
            }
        }
    }

    pub fn largest(&self) -> Option<&T> {
        let mut current = if let Some(ref root) = self.root {
            unsafe { &(*root.as_ptr()) }
        } else {
            return None;
        };
        while let Some(ref node) = current.children[1] {
            current = unsafe { &(*node.as_ptr()) };
        }
        Some(&current.value)
    }

    pub fn smallest(&self) -> Option<&T> {
        let mut current = if let Some(ref root) = self.root {
            unsafe { &(*root.as_ptr()) }
        } else {
            return None;
        };
        while let Some(ref node) = current.children[0] {
            current = unsafe { &(*node.as_ptr()) };
        }
        Some(&current.value)
    }

    pub fn traverse_in_order<F>(&self, mut closure: F)
    where
        F: FnMut(&T),
    {
        let mut stack = Vec::with_capacity(self.length);
        let mut current = &self.root;
        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                let ptr = unsafe { &(*node.as_ptr()) };
                stack.push(ptr);
                current = &ptr.children[0];
            }
            let popped = stack
                .pop()
                .expect("Can't be empty because of the checks above!");
            closure(&popped.value);
            current = &popped.children[1];
        }
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

pub enum IntoIter<T> {
    Empty,
    Full(Vec<NonNull<Node<T>>>),
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        match self {
            // Clippy suggests to return () instead of using the return keyword
            IntoIter::Empty => (),
            IntoIter::Full(n) => {
                while let Some(node) = n.pop() {
                    let mut d = unsafe { Box::from_raw(node.as_ptr()) };
                    if let Some(left) = d.children[0] {
                        n.push(left);
                    }
                    if let Some(right) = d.children[1] {
                        n.push(right);
                    }
                    unsafe { ManuallyDrop::drop(&mut d.value) };
                    drop(d);
                }
            }
        }
    }
}

impl<T> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let tree = ManuallyDrop::new(self);
        if let Some(root) = tree.root {
            IntoIter::Full(vec![root])
        } else {
            IntoIter::Empty
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // If it is Empty then we return immediately, otherwise the branch prediction will
        // always prove to be right from then on. Hence it is a free branch!
        match self {
            IntoIter::Empty => None,
            IntoIter::Full(n) => {
                let mut d = unsafe { Box::from_raw(n.pop()?.as_ptr()) };
                if let Some(left) = d.children[0] {
                    n.push(left);
                }
                if let Some(right) = d.children[1] {
                    n.push(right);
                }
                let value = unsafe { ManuallyDrop::take(&mut d.value) };
                drop(d);
                Some(value)
            }
        }
    }
}

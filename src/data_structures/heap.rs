#![allow(dead_code)]

pub trait Heap<'a>
where
    Self: 'a,
{
    type Item;
    type Index: TryFrom<(&'a Self, usize)> + Into<usize>;

    /// Pushes an element into the Heap according to the Heap property that has been implemented.
    fn push(&mut self, element: Self::Item);

    /// Removes the Max or Min element as per the implementation.
    fn pop(&mut self) -> Option<Self::Item>;

    /// Takes an index and bubbles down the element if needed according to the Heap property.
    /// Panics if index is out of bounds.
    fn bubble_down(&mut self, index: Self::Index);

    /// Takes an index and bubbles up the element if needed according to the Heap property.
    /// Panics if index is out of bounds.
    fn bubble_up(&mut self, index: Self::Index);

    /// Returns the number of elements in the Heap.
    fn len(&self) -> usize;

    /// Checks whether the Heap has no elements.
    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
}

pub struct Index(usize);

impl From<Index> for usize {
    fn from(value: Index) -> usize {
        value.0
    }
}

//----------------------------------------------------------------------------------------------------------

impl<'a, T: Ord> TryFrom<(&'a MinHeap<T>, usize)> for Index {
    type Error = Error;

    fn try_from(value: (&'a MinHeap<T>, usize)) -> Result<Self, Self::Error> {
        if value.1 >= value.0.len() {
            Err(Error::OutOfBounds)
        } else {
            Ok(Index(value.1))
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MinHeap<T: Ord> {
    elements: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            elements: Vec::with_capacity(cap),
        }
    }
}

impl<'a, T> Heap<'a> for MinHeap<T>
where
    Self: 'a,
    T: Ord,
{
    type Item = T;
    type Index = Index;

    fn push(&mut self, element: Self::Item) {
        self.elements.push(element);
        let index = self.len() - 1;
        self.bubble_up(Index(index));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        let len = self.len();
        if len == 0 {
            return None;
        }

        self.elements.swap(0, len - 1);

        // SAFETY:
        //    We did the bounds check above.
        let ret = unsafe { self.elements.pop().unwrap_unchecked() };

        if len == 1 {
            return Some(ret);
        }

        self.bubble_down(Index(0));
        Some(ret)
    }

    fn bubble_down(&mut self, index: Self::Index) {
        let len = self.len();
        let mut index: usize = index.into();
        while index < len {
            let left_child = index * 2 + 1;
            let right_child = index * 2 + 2;

            let mut to_swap = index;

            if left_child < len && self.elements[index] > self.elements[left_child] {
                to_swap = left_child;
            }

            if right_child < len && self.elements[right_child] < self.elements[to_swap] {
                to_swap = right_child;
            }

            if to_swap == index {
                return;
            } else {
                self.elements.swap(index, to_swap);
                index = to_swap;
            }
        }
    }

    fn bubble_up(&mut self, index: Self::Index) {
        let mut index: usize = index.into();

        loop {
            let parent = (index as isize - 1) / 2;

            if parent >= 0 && self.elements[parent as usize] > self.elements[index] {
                self.elements.swap(index, parent as usize);
                index = parent as usize;
            } else {
                break;
            }
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct MinIntoIter<T: Ord> {
    heap: MinHeap<T>,
}

impl<T: Ord> Iterator for MinIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

impl<T: Ord> IntoIterator for MinHeap<T> {
    type Item = T;
    type IntoIter = MinIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        MinIntoIter { heap: self }
    }
}

// ----------------------------------------------------------------------------------------------------

impl<'a, T: Ord> TryFrom<(&'a MaxHeap<T>, usize)> for Index {
    type Error = Error;

    fn try_from(value: (&'a MaxHeap<T>, usize)) -> Result<Self, Self::Error> {
        if value.1 >= value.0.len() {
            Err(Error::OutOfBounds)
        } else {
            Ok(Index(value.1))
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct MaxHeap<T: Ord> {
    elements: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            elements: Vec::with_capacity(cap),
        }
    }
}

impl<'a, T> Heap<'a> for MaxHeap<T>
where
    Self: 'a,
    T: Ord,
{
    type Item = T;
    type Index = Index;

    fn push(&mut self, element: Self::Item) {
        self.elements.push(element);
        let index = self.len() - 1;
        self.bubble_up(Index(index));
    }

    fn pop(&mut self) -> Option<Self::Item> {
        let len = self.len();

        if len == 0 {
            return None;
        }

        self.elements.swap(0, len - 1);

        let ret = unsafe { self.elements.pop().unwrap_unchecked() };

        if len == 1 {
            return Some(ret);
        }

        self.bubble_down(Index(0));
        Some(ret)
    }

    fn bubble_down(&mut self, index: Self::Index) {
        let len = self.len();

        let mut index: usize = index.into();
        while index < len {
            let left_child = index * 2 + 1;
            let right_child = index * 2 + 2;

            let mut to_swap = index;

            if left_child < len && self.elements[left_child] > self.elements[index] {
                to_swap = left_child;
            }

            if right_child < len && self.elements[right_child] > self.elements[to_swap] {
                to_swap = right_child;
            }

            if to_swap == index {
                return;
            }

            self.elements.swap(index, to_swap);

            index = to_swap;
        }
    }

    fn bubble_up(&mut self, index: Self::Index) {
        let mut index: usize = index.into();

        loop {
            let parent = ((index as isize) - 1) / 2;

            if parent >= 0 && self.elements[index] > self.elements[parent as usize] {
                self.elements.swap(index, parent as usize);
                index = parent as usize;
            } else {
                break;
            }
        }
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

pub struct MaxIntoIter<T: Ord> {
    heap: MaxHeap<T>,
}

impl<T: Ord> Iterator for MaxIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

impl<T: Ord> IntoIterator for MaxHeap<T> {
    type Item = T;
    type IntoIter = MaxIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        MaxIntoIter { heap: self }
    }
}

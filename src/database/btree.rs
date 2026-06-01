use std::fmt::{self, Display, Formatter};
use std::mem::swap;

const ORDER: usize = 5;
const ELEMENTS_LEN: usize = ORDER;
const POINTERS_LEN: usize = ORDER + 1;
const MIDLE: usize = ORDER / 2;

// t has to be key value pair for this to make sence
#[derive(Debug)]
pub struct BTree<T: PartialOrd> {
    pub elements: [Option<Box<T>>; ORDER],
    pub pointers: [Option<Box<BTree<T>>>; ORDER + 1],
}

struct OverFlow<T: PartialOrd> {
    element: Option<Box<T>>,
    left: Option<Box<BTree<T>>>,
    right: Option<Box<BTree<T>>>,
}

impl<T: PartialOrd + PartialEq + std::fmt::Display + std::fmt::Debug> OverFlow<T> {
    fn new(
        element: Option<Box<T>>,
        left: Option<Box<BTree<T>>>,
        right: Option<Box<BTree<T>>>,
    ) -> Self {
        Self {
            element,
            left,
            right,
        }
    }
}

impl<T> Default for BTree<T>
where
    T: PartialOrd + PartialEq + Display + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BTree<T>
where
    T: PartialOrd + PartialEq + Display + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            elements: [const { None }; ORDER],
            pointers: [const { None }; ORDER + 1],
        }
    }

    pub fn insert(&mut self, element: T) {
        let tmp = self._insert(element);
        if let Some(tmp) = tmp {
            let mut btree: BTree<T> = BTree::<T>::new();
            btree.elements[0] = tmp.element;
            btree.pointers[0] = tmp.left;
            btree.pointers[1] = tmp.right;
            swap(self, &mut btree);
        }
    }
    fn _len(&self) -> usize {
        let len = self.elements.iter().position(|e| e.is_none());
        match len {
            Some(l) => l,
            None => ELEMENTS_LEN,
        }
    }

    pub fn find(&self, element: T) -> Option<&T> {
        let len = self._len();
        if len == 0 {
            return None;
        }

        let idx = self.find_index(&element);
        if idx < len {
            let v = self.elements[idx].as_ref().unwrap();
            if **v == element {
                return Some(v);
            }
        }

        match self.pointers[idx].as_ref() {
            Some(ptr) => ptr.find(element),
            None => None,
        }
    }

    // Return the index of the element if present, or the child/index
    // where it should be inserted (first index greater than element).
    fn find_index(&self, element: &T) -> usize {
        let len = self._len();
        if len == 0 {
            return 0;
        }

        let mut lo: usize = 0;
        let mut hi: usize = len;
        while lo < hi {
            let mid = (lo + hi) / 2;
            let v = self.elements[mid].as_ref().unwrap();
            if **v == *element {
                return mid;
            }
            if **v < *element {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo
    }

    fn _is_leaf_full(&self) -> bool {
        self.elements[ELEMENTS_LEN - 1].is_some()
    }

    fn _split_node(&mut self) -> OverFlow<T> {
        let mut left: Box<BTree<T>> = Box::default();
        let mut right: Box<BTree<T>> = Box::default();

        let median = MIDLE;
        let left_count = median;
        let right_count = ELEMENTS_LEN - median - 1;

        for i in 0..left_count {
            left.elements[i] = self.elements[i].take();
        }

        for i in 0..right_count {
            right.elements[i] = self.elements[median + 1 + i].take();
        }

        for i in 0..=left_count {
            left.pointers[i] = self.pointers[i].take();
        }

        for i in 0..=right_count {
            right.pointers[i] = self.pointers[median + 1 + i].take();
        }

        let tmp = self.elements[median].take();
        OverFlow::new(tmp, Some(left), Some(right))
    }

    fn _insert(&mut self, element: T) -> Option<OverFlow<T>> {
        let idx = self.find_index(&element);

        match self.pointers[idx].as_mut() {
            Some(child) => match child._insert(element) {
                Some(extra) => {
                    if self._is_leaf_full() {
                        let mut n_extra = self._split_node();
                        if idx <= MIDLE {
                            n_extra
                                .left
                                .as_mut()
                                .unwrap()
                                ._insert_overflow_at_index(idx, extra);
                        } else {
                            let right_index = idx.saturating_sub(MIDLE + 1);
                            n_extra
                                .right
                                .as_mut()
                                .unwrap()
                                ._insert_overflow_at_index(right_index, extra);
                        }
                        Some(n_extra)
                    } else {
                        self._insert_overflow_at_index(idx, extra);
                        None
                    }
                }
                None => None,
            },
            None => {
                // leaf insertion
                if self._is_leaf_full() {
                    let mut extra = self._split_node();
                    if **extra.element.as_ref().unwrap() > element {
                        extra.left.as_mut().unwrap()._insert(element);
                    } else {
                        extra.right.as_mut().unwrap()._insert(element);
                    }
                    Some(extra)
                } else {
                    self.elements[idx..].rotate_right(1);
                    self.elements[idx] = Some(Box::new(element));
                    self.pointers[idx..].rotate_right(1);
                    None
                }
            }
        }
    }

    //check if the node is full before calling
    fn _insert_overflow_at_index(&mut self, i: usize, mut extra: OverFlow<T>) {
        // make room for the new element
        self.elements[i..].rotate_right(1);
        swap(&mut self.elements[i], &mut extra.element);

        // make room for the two new child pointers and place them
        self.pointers[i..].rotate_right(1);
        // after rotation the slot at i is free
        swap(&mut self.pointers[i], &mut extra.left);
        // place right child into i+1
        if i + 1 < POINTERS_LEN {
            swap(&mut self.pointers[i + 1], &mut extra.right);
        }
    }

    fn fmt_pretty_with_indent(&self, depth: usize, out: &mut String) {
        let indent = "  ".repeat(depth);

        out.push_str(&format!("{indent}BTreeNode {{\n"));

        out.push_str(&format!("{indent}  elements: ["));
        let mut first = true;
        for e in &self.elements {
            if !first {
                out.push_str(", ");
            }
            first = false;
            match e {
                Some(v) => out.push_str(&format!("{v}")),
                None => out.push('_'),
            }
        }
        out.push_str("]\n");

        for (i, ptr) in self.pointers.iter().enumerate() {
            if let Some(child) = ptr.as_ref() {
                out.push_str(&format!("{indent}  child[{i}]:\n"));
                child.fmt_pretty_with_indent(depth + 1, out);
            }
        }

        out.push_str(&format!("{indent}}}\n"));
    }
}

impl<T> Display for BTree<T>
where
    T: PartialOrd + PartialEq + Display + std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.fmt_pretty_with_indent(0, &mut out);
        write!(f, "{out}")
    }
}

use std::{cell::RefCell, collections::VecDeque, fs, rc::{Rc, Weak}};

use weak_table::{PtrWeakHashSet, PtrWeakKeyHashMap};

use crate::core::symbol::Symbol;

#[macro_export]
macro_rules! S {
    ($x: expr) => {
        String::from($x)
    };
}

pub fn is_file_cs(path: String) -> bool {
    match fs::canonicalize(path) {
        Ok(canonical_path) => {
            return fs::metadata(canonical_path).unwrap().is_file()
        }
        Err(_err) => {
            return false;
        }
    }
}

pub fn is_dir_cs(path: String) -> bool {
    match fs::canonicalize(path) {
        Ok(canonical_path) => {
            return fs::metadata(canonical_path).unwrap().is_dir()
        }
        Err(_err) => {
            return false;
        }
    }
}

//TODO use it?
pub fn is_symlink_cs(path: String) -> bool {
    match fs::canonicalize(path) {
        Ok(canonical_path) => {
            return fs::metadata(canonical_path).unwrap().is_symlink()
        }
        Err(_err) => {
            return false;
        }
    }
}

#[derive(Debug)]
pub struct SortedWeakPtrSet {
    vec: VecDeque<Weak<RefCell<Symbol>>>,
    set: PtrWeakKeyHashMap<Weak<RefCell<Symbol>>, usize>
}

impl SortedWeakPtrSet {

    pub fn new() -> Self {
        SortedWeakPtrSet {
            vec: VecDeque::new(),
            set: PtrWeakKeyHashMap::new()
        }
    }

    pub fn contains(&self, dep: &Rc<RefCell<Symbol>>) -> bool {
        self.set.contains_key(dep)
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    pub fn len(&self) -> usize {
        self.set.len()
    }

    pub fn remove(&mut self, symbol: &Rc<RefCell<Symbol>>) -> Option<Weak<RefCell<Symbol>>> {
        let index = self.set.remove(&symbol);
        if let Some(index) = index {
            return self.vec.remove(index);
        }
        return None;
    }

    pub fn insert(&mut self, symbol: Rc<RefCell<Symbol>>) {
        if !self.set.contains_key(&symbol) {
            self.set.insert(symbol.clone(), self.vec.len());
            self.vec.push_back(Rc::downgrade(&symbol));
        }
    }

    pub fn pop(&mut self) -> Option<Rc<RefCell<Symbol>>> {
        while let Some(weak) = self.vec.pop_front() {
            if let Some(upgraded) = weak.upgrade() {
                self.set.remove(&upgraded);
                return Some(upgraded);
            }
        }
        None
    }

    pub(crate) fn clear(&mut self) {
        self.set.clear();
        self.vec.clear();
    }

}
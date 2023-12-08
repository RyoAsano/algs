pub mod btreeimpl;

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct BTree<T, K: Ord>(Chi<RefCell<Trunk<BTree<T, K>, K>>, RefCell<Trunk<T, K>>>);

#[derive(Debug)]
struct Trunk<ChiTy, K: Ord> {
    keys: Vec<K>,
    chi: Vec<ChiTy>,
}

#[derive(Debug)]
enum Chi<TrTy, BrTy> {
    Tr(TrTy),
    Br(BrTy),
}

impl<TrTy, BrTy> Chi<TrTy, BrTy> {
    fn as_ref(&self) -> Chi<&TrTy, &BrTy> {
        match self {
            Chi::Br(br) => Chi::Br(br),
            Chi::Tr(tr) => Chi::Tr(tr),
        }
    }

    fn unwrap_tr(&self) -> &TrTy {
        match self {
            Chi::Tr(tree) => tree,
            _ => panic!("Failed to unwrap the tree."),
        }
    }

    fn unwrap_br(&self) -> &BrTy {
        match self {
            Chi::Br(branch) => branch,
            _ => panic!("Failed to unwrap the tree."),
        }
    }
}

impl<T, K: Ord + Copy> Chi<RefCell<Trunk<BTree<T, K>, K>>, RefCell<Trunk<T, K>>> {
    fn keycap(&self) -> usize {
        match self {
            Chi::Br(branch) => branch.borrow().keys.capacity(),
            Chi::Tr(tree) => tree.borrow().keys.capacity(),
        }
    }

    fn chicap(&self) -> usize {
        match self {
            Chi::Br(branch) => branch.borrow().chi.capacity(),
            Chi::Tr(tree) => tree.borrow().chi.capacity(),
        }
    }

    fn size(&self) -> usize {
        match self {
            Chi::Br(branch) => branch.borrow().chi.capacity() - 1,
            Chi::Tr(tree) => tree.borrow().chi.capacity() - 1,
        }
    }

    fn len(&self) -> usize {
        match self {
            Chi::Br(branch) => branch.borrow().chi.len(),
            Chi::Tr(tree) => tree.borrow().chi.len(),
        }
    }

    fn minkey(&self) -> K {
        match self {
            Chi::Br(branch) => *branch.borrow().keys.iter().min().unwrap(),
            Chi::Tr(tree) => *tree.borrow().keys.iter().min().unwrap(),
        }
    }

    fn maxkey(&self) -> K {
        match self {
            Chi::Br(branch) => *branch.borrow().keys.iter().max().unwrap(),
            Chi::Tr(tree) => *tree.borrow().keys.iter().max().unwrap(),
        }
    }
}

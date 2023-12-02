pub(self) mod rotation;

use core::panic;
use std::{cell::RefCell, rc::Rc};

use super::{trunk::Trunk, BinTree};

pub trait Avl<T, K: Ord + Copy> {
    fn new(key: K, value: T) -> Self;

    fn delete(self: Self, key: K) -> (bool, Event<T, K>);

    fn find(&self, cand: K) -> (bool, K);

    fn insert(self: Self, key: K, value: T) -> Event<T, K>;
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    LeftSided,
    Balanced,
    RightSided,
}

pub enum Event<T, K: Ord + Copy> {
    None(BinTree<T, K, State>),
    Grown(BinTree<T, K, State>),
    Shrunk(Option<BinTree<T, K, State>>),
}

impl<T, K: Ord + Copy> Event<T, K> {
    pub fn unwrap(self: Self) -> BinTree<T, K, State> {
        match self {
            Self::None(t) => t,
            Self::Grown(t) => t,
            Self::Shrunk(t) => t.unwrap(),
        }
    }
}

impl<T, K: Ord + Copy> BinTree<T, K, State> {
    fn take_rightmost_leaf(self: Self) -> (Event<T, K>, Self) {
        let right = self.t.borrow_mut().right.take().expect("No right branch.");
        if right.t.borrow().right.is_none() {
            let state = self.t.borrow().state;
            match state {
                State::Balanced => {
                    self.t.borrow_mut().state = State::LeftSided;
                    (Event::None(self), right)
                }
                State::RightSided => {
                    self.t.borrow_mut().state = State::Balanced;
                    (Event::Shrunk(None), right)
                }
                State::LeftSided => {
                    panic!("Contradiction: cannot be left sided as right leaf exists.")
                }
            }
        } else {
            let (event, the_leaf) = right.take_rightmost_leaf();
            (self.remerge_right_branch(event), the_leaf)
        }
    }

    fn remerge_right_branch(self: Self, event: Event<T, K>) -> Event<T, K> {
        match event {
            Event::None(new_right) => {
                self.t.borrow_mut().right = Some(new_right);
                Event::None(self)
            }
            Event::Shrunk(new_right) => {
                self.t.borrow_mut().right = new_right;
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => self.rotate_to_right(),
                    State::Balanced => {
                        self.t.borrow_mut().state = State::LeftSided;
                        Event::None(self)
                    }
                    State::RightSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::Shrunk(Some(self))
                    }
                }
            }
            Event::Grown(new_right) => {
                self.t.borrow_mut().right = Some(new_right);
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::None(self)
                    }
                    State::Balanced => {
                        self.t.borrow_mut().state = State::RightSided;
                        Event::Grown(self)
                    }
                    State::RightSided => {
                        // return None as rotating rolls back the state before insertion.
                        // note that self.rotate_to_left() can return Shrunk.
                        Event::None(self.rotate_to_left().unwrap())
                    }
                }
            }
        }
    }

    fn remerge_left_branch(self: Self, event: Event<T, K>) -> Event<T, K> {
        match event {
            Event::None(new_left) => {
                self.t.borrow_mut().left = Some(new_left);
                Event::None(self)
            }
            Event::Shrunk(new_left) => {
                self.t.borrow_mut().left = new_left;
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::Shrunk(Some(self))
                    }
                    State::Balanced => {
                        self.t.borrow_mut().state = State::RightSided;
                        Event::None(self)
                    }
                    State::RightSided => self.rotate_to_left(),
                }
            }
            Event::Grown(new_left) => {
                self.t.borrow_mut().left = Some(new_left);
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => {
                        // return None as rotating rolls back the state before insertion.
                        // note that self.rotate_to_right() can return Shrunk.
                        Event::None(self.rotate_to_right().unwrap())
                    }
                    State::Balanced => {
                        self.t.borrow_mut().state = State::LeftSided;
                        Event::Grown(self)
                    }
                    State::RightSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::None(self)
                    }
                }
            }
        }
    }
}

impl<T, K: Ord + Copy> Avl<T, K> for BinTree<T, K, State> {
    fn new(key: K, value: T) -> Self {
        Self {
            t: Rc::new(RefCell::new(Trunk {
                key,
                value,
                left: None,
                right: None,
                state: State::Balanced,
            })),
        }
    }

    fn insert(self: Self, key: K, value: T) -> Event<T, K> {
        if key < self.t.borrow().key {
            if self.t.borrow().left.is_none() {
                self.t.borrow_mut().left = Some(Self::new(key, value));
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => self.rotate_to_right(),
                    State::Balanced => {
                        self.t.borrow_mut().state = State::LeftSided;
                        Event::Grown(self)
                    }
                    State::RightSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::None(self)
                    }
                }
            } else {
                let event = self.t.borrow_mut().left.take().unwrap().insert(key, value);
                self.remerge_left_branch(event)
            }
        } else if self.t.borrow().key < key {
            if self.t.borrow().right.is_none() {
                self.t.borrow_mut().right = Some(Self::new(key, value));
                let state = self.t.borrow().state;
                match state {
                    State::LeftSided => {
                        self.t.borrow_mut().state = State::Balanced;
                        Event::None(self)
                    }
                    State::Balanced => {
                        self.t.borrow_mut().state = State::RightSided;
                        Event::Grown(self)
                    }
                    State::RightSided => self.rotate_to_left(),
                }
            } else {
                let event = self.t.borrow_mut().right.take().unwrap().insert(key, value);
                self.remerge_right_branch(event)
            }
        } else {
            panic!("The key already exists.")
        }
    }

    fn delete(self: Self, key: K) -> (bool, Event<T, K>) {
        if self.t.borrow().key == key {
            if self.t.borrow().left.is_none() {
                (true, Event::Shrunk(self.t.borrow_mut().right.take()))
            } else {
                let left = self.t.borrow_mut().left.take().unwrap();
                let (event, replacing_branch) = if left.t.borrow().right.is_none() {
                    let left_of_left = left.t.borrow_mut().left.take();
                    (Event::Shrunk(left_of_left), left)
                } else {
                    left.take_rightmost_leaf()
                };
                // 1. Exchange the positions of self and self's left in effect.
                // 2. Remerge the left's left branch as if its left gets shrunk.
                replacing_branch.t.borrow_mut().state = self.t.borrow().state;
                replacing_branch.t.borrow_mut().right = self.t.borrow_mut().right.take();
                (true, replacing_branch.remerge_left_branch(event))
            }
        } else if self.t.borrow().key < key {
            let right = self.t.borrow_mut().right.take();
            if let Some(right) = right {
                let (found, event) = right.delete(key);
                (found, self.remerge_right_branch(event))
            } else {
                (false, Event::None(self))
            }
        } else {
            let left = self.t.borrow_mut().left.take();
            if let Some(left) = left {
                let (found, event) = left.delete(key);
                (found, self.remerge_left_branch(event))
            } else {
                (false, Event::None(self))
            }
        }
    }
    fn find(&self, cand: K) -> (bool, K) {
        if self.t.borrow().key == cand {
            return (true, cand);
        } else if cand < self.t.borrow().key {
            if self.t.borrow().left.is_none() {
                return (false, self.t.borrow().key);
            } else {
                return self.t.borrow().left.as_ref().unwrap().find(cand);
            }
        } else {
            if self.t.borrow().right.is_none() {
                return (false, self.t.borrow().key);
            } else {
                return self.t.borrow().right.as_ref().unwrap().find(cand);
            }
        }
    }
}

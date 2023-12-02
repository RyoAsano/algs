use crate::trees::bintree::BinTree;

use super::{State, Event};

impl<T, K: Ord + Copy> BinTree<T, K, State> {
    pub(super) fn rotate_to_left(self: Self) -> Event<T, K> {
        let state_of_right = self.t.borrow().right.as_ref().unwrap().t.borrow().state;
        match state_of_right {
            State::RightSided => self.rotate_to_left_once(),
            State::LeftSided => self.rotate_to_left_twice(),
            State::Balanced => panic!("Rotation should not be necessary in this case."),
        }
    }

    pub(super) fn rotate_to_right(self: Self) -> Event<T, K> {
        let state_of_left = self.t.borrow().left.as_ref().unwrap().t.borrow().state;
        match state_of_left {
            State::LeftSided => self.rotate_to_right_once(),
            State::RightSided => self.rotate_to_right_twice(),
            State::Balanced => panic!("Rotation should not be necessary in this case."),
        }
    }

    fn rotate_to_right_once(self: Self) -> Event<T, K> {
        // Take necessary branches
        let left = self.t.borrow_mut().left.take().unwrap();
        let right_of_left = left.t.borrow_mut().right.take();

        // Update the state before move.
        let shrunk;
        let state = left.t.borrow().state;
        match state {
            State::LeftSided => {
                self.t.borrow_mut().state = State::Balanced;
                shrunk = true;
            }
            State::Balanced => {
                self.t.borrow_mut().state = State::LeftSided;
                shrunk = false;
            }
            State::RightSided => panic!("Single rotation cannot be applicable in this case."),
        };
        left.t.borrow_mut().state = State::Balanced;

        // Here we move
        self.t.borrow_mut().left = right_of_left;
        left.t.borrow_mut().right = Some(self);
        if shrunk {
            Event::Shrunk(Some(left))
        } else {
            Event::None(left)
        }
    }

    fn rotate_to_right_twice(self: Self) -> Event<T, K> {
        // Take necessary branches
        let left = self.t.borrow_mut().left.take().unwrap();
        let right_of_left = left.t.borrow_mut().right.take().unwrap();
        let right_of_right_of_left = right_of_left.t.borrow_mut().right.take();
        let left_of_right_of_left = right_of_left.t.borrow_mut().left.take();

        // Update the state before move.
        let state = right_of_left.t.borrow().state;
        match state {
            State::RightSided => {
                self.t.borrow_mut().state = State::Balanced;
                left.t.borrow_mut().state = State::LeftSided;
            }
            State::LeftSided => {
                self.t.borrow_mut().state = State::RightSided;
                left.t.borrow_mut().state = State::Balanced;
            }
            State::Balanced => {
                panic!("Not supposed to arrive here.");
            }
        };
        right_of_left.t.borrow_mut().state = State::Balanced;

        // Here we move
        self.t.borrow_mut().left = right_of_right_of_left;
        left.t.borrow_mut().right = left_of_right_of_left;
        right_of_left.t.borrow_mut().right = Some(self);
        right_of_left.t.borrow_mut().left = Some(left);

        // Double rotation always makes trees contract.
        Event::Shrunk(Some(right_of_left))
    }

    fn rotate_to_left_once(self: Self) -> Event<T, K> {
        // Take necessary branches
        let right = self.t.borrow_mut().right.take().unwrap();
        let left_of_right = right.t.borrow_mut().left.take();

        // Update the state before move.
        let shrunk;
        let state = right.t.borrow_mut().state;
        match state {
            State::RightSided => {
                self.t.borrow_mut().state = State::Balanced;
                shrunk = true;
            }
            State::Balanced => {
                self.t.borrow_mut().state = State::RightSided;
                shrunk = false;
            }
            State::LeftSided => panic!("Single rotation cannot be applicable in this case."),
        };
        right.t.borrow_mut().state = State::Balanced;

        // Here we move
        self.t.borrow_mut().right = left_of_right;
        right.t.borrow_mut().left = Some(self);
        if shrunk {
            Event::Shrunk(Some(right))
        } else {
            Event::None(right)
        }
    }

    fn rotate_to_left_twice(self: Self) -> Event<T, K> {
        // Take necessary branches
        let right = self.t.borrow_mut().right.take().unwrap();
        let left_of_right = right.t.borrow_mut().left.take().unwrap();
        let right_of_left_of_right = left_of_right.t.borrow_mut().right.take();
        let left_of_left_of_right = left_of_right.t.borrow_mut().left.take();

        // Update the state before move.
        let state = left_of_right.t.borrow().state;
        match state {
            State::RightSided => {
                self.t.borrow_mut().state = State::LeftSided;
                right.t.borrow_mut().state = State::Balanced;
            }
            State::LeftSided => {
                self.t.borrow_mut().state = State::Balanced;
                right.t.borrow_mut().state = State::RightSided;
            }
            State::Balanced => {
                panic!("Not supposed to arrive here.");
            }
        };
        left_of_right.t.borrow_mut().state = State::Balanced;

        // Here we move
        self.t.borrow_mut().right = left_of_left_of_right;
        right.t.borrow_mut().left = right_of_left_of_right;
        left_of_right.t.borrow_mut().right = Some(right);
        left_of_right.t.borrow_mut().left = Some(self);

        // Double rotation always makes trees contract.
        Event::Shrunk(Some(left_of_right))
    }
}
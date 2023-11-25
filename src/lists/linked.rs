use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
pub struct Linked<T: Default> {
    value: T,
    next: Option<Rc<RefCell<Linked<T>>>>,
}

impl<T: Default> Linked<T> {
    pub fn new_head(value: T) -> RefCell<Self> {
        RefCell::new(Linked {
            value: T::default(),
            next: Some(Rc::new(RefCell::new(Linked { value, next: None }))),
        })
    }

    pub fn append(&mut self, value: T) {
        if self.next.is_none() {
            self.next = Some(Rc::new(RefCell::new(Linked { value, next: None })))
        } else {
            self.next.as_ref().unwrap().borrow_mut().append(value);
        }
    }

    pub fn delete_from_head(head: &RefCell<Linked<T>>, n: u32) {
        if n == 0 {
            if head.borrow().next.is_some() {
                let next = head.borrow().next.as_ref().unwrap().clone();
                head.borrow_mut().next = next.borrow_mut().next.take();

            } else {
                 panic!("The length of the list is less than the specified number.")
            }
        } else {
            Self::delete_from_head(head.borrow().next.as_ref().unwrap(), n-1)
        }
    }
}

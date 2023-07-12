use std::fmt::Display;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Parent(Option<Rc<RefCell<Child>>>);

impl Parent {
    fn assign_child(&mut self, child: Rc<RefCell<Child>>) {
        self.0 = Some(child);
    }
}

#[derive(Debug)]
struct Child(Rc<RefCell<Parent>>);

fn main() {
    let p = Rc::new(RefCell::new(Parent(None)));
    let c = Child(p.clone());
    p.borrow_mut().assign_child(Rc::new(RefCell::new(c)));
    
    println!("{:?}",p);
}
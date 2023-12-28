use std::{
  cell::RefCell,
  ops::Deref,
  rc::{Rc, Weak},
};

////////////////////////////////////////////////////////! End Imports

struct Parent {
  child: Option<Rc<RefCell<Child>>>,
}

impl Parent {
  pub fn new() -> Rc<RefCell<Self>> {
    let new_parent = Rc::new(RefCell::new(Self { child: None }));

    new_parent.deref().borrow_mut().child = Some(Child::new());

    new_parent
      .deref()
      .borrow_mut()
      .child
      .as_ref()
      .unwrap()
      .deref()
      .borrow_mut()
      .parent = Some(Rc::downgrade(&new_parent.clone()));

    new_parent
  }

  pub fn main(&mut self) {
    println!("parent: running mutation on child");
    self.child.as_deref().unwrap().borrow_mut().mutate_parent();
  }

  pub fn parent_mutate_procedure(&mut self) {
    println!("parent: I'm a mutant")
  }
}

impl Drop for Parent {
  fn drop(&mut self) {
    println!("parent dropped!")
  }
}

////////////////////////////////////////////////////////! End Parent

struct Child {
  parent: Option<Weak<RefCell<Parent>>>,
}

impl Child {
  pub fn new() -> Rc<RefCell<Self>> {
    Rc::new(RefCell::new(Self { parent: None }))
  }

  pub fn mutate_parent(&mut self) {
    println!("child: Attempting to mutate parent!");
    self
      .parent
      .as_ref()
      .unwrap()
      .upgrade()
      .unwrap()
      .deref()
      .borrow_mut()
      .parent_mutate_procedure();
  }
}

impl Drop for Child {
  fn drop(&mut self) {
    println!("child dropped!")
  }
}

////////////////////////////////////////////////////////! End Child

///
/// logic flow:
///
/// parent: main()
/// child: mutate_parent()
/// parent: parent_mutate_procedure()
fn main() {
  Parent::new().deref().borrow_mut().main();
}

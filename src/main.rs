use std::{
  cell::RefCell,
  ops::Deref,
  rc::{Rc, Weak},
};

////////////////////////////////////////////////////////! End Imports

struct Message {
  pub closure: Rc<dyn Fn(&mut Parent)>,
}

////////////////////////////////////////////////////////! End Message

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
    let mutation_attempt = self.child.as_deref().unwrap().borrow_mut().mutate_parent();

    (mutation_attempt.unwrap().closure)(self);
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

  pub fn mutate_parent(&mut self) -> Option<Message> {
    println!("child: Attempting to mutate parent!");
    Some(Message {
      closure: Rc::new(|parent| {
        parent.parent_mutate_procedure();
      }),
    })
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
/// child: mutate_parent() -> this passes back a Message which contains a closure
/// parent: closure() -> (contains parent_mutate_procedure execution)
/// parent: parent_mutate_procedure()
fn main() {
  Parent::new().deref().borrow_mut().main();
}

extern crate message_to_parent;

use message_to_parent::MessageToParent;

struct Parent {
  thing_was_done: bool,
  child: Child,
}

impl Parent {
  pub fn new() -> Self {
    Parent {
      thing_was_done: false,
      child: Child::new(),
    }
  }

  pub fn main(&mut self) {
    println!("Parent: Thing was done? {}", self.thing_was_done);

    let mut child_message = self.child.do_thing();

    child_message.run_side_effects(self);

    println!("Parent: Thing was done? {}", self.thing_was_done);
  }
}

struct Child {}

impl Child {
  pub fn new() -> Self {
    Child {}
  }

  pub fn do_thing(&self) -> MessageToParent<Parent, ()> {
    let mut message = MessageToParent::<Parent, ()>::new();

    println!("Child: Doing thing!");

    message.add_side_effect(|parent| parent.thing_was_done = true);

    message
  }
}

///
/// This is literally the most basic example I could come up with.
///
fn main() {
  Parent::new().main();
}

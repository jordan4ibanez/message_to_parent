use std::{
  cell::RefCell,
  ops::Deref,
  rc::{Rc, Weak},
};

////////////////////////////////////////////////////////! End Imports

struct MessageToParent<ParentType, ReturnType> {
  side_effects: Vec<fn(&mut ParentType) -> ReturnType>,
  results: Vec<ReturnType>,
  has_results: bool,
}

impl<ParentType, ReturnType> MessageToParent<ParentType, ReturnType> {
  pub fn new() -> Self {
    MessageToParent {
      side_effects: vec![],
      results: vec![],
      has_results: false,
    }
  }

  ///
  /// Create a new side effect for the parent to run.
  ///
  /// Note: This is a "stack". The parent will run these in the order they were added.
  ///
  pub fn add_side_effect(&mut self, new_side_effect: fn(&mut ParentType) -> ReturnType) {
    self.side_effects.push(new_side_effect);
  }

  ///
  /// Run all created side effects.
  ///
  /// This also will accumulate the results in a "stack" which maintains the order that
  /// the functions were added into the function "stack".
  ///
  /// This is slightly slower than run_side_effects due to pushing into the results "stack".
  ///
  /// Note: Only the parent should be running this after it was received by the child.
  ///
  pub fn run_side_effects_accumulate_results(&mut self, parent: &mut ParentType) {
    self.has_results = true;
    for side_effect in &self.side_effects {
      self.results.push(side_effect(parent));
    }
  }

  ///
  /// Run all created side effects.
  ///
  /// Note: Only the parent should be running this after it was received by the child.
  ///
  pub fn run_side_effects(&mut self, parent: &mut ParentType) {
    for side_effect in &self.side_effects {
      side_effect(parent);
    }
  }

  ///
  /// Parse the results of the side effects ran on the Parent.
  ///
  /// If you do not execute run_side_effects_accumulate_results this will panic
  /// because you have no results and I'd like for you to know that you made a mistake
  /// instead of it blindly allowing you to do that. :)
  ///
  pub fn get_results(&self) -> &Vec<ReturnType> {
    if !self.has_results {
      panic!("MessageToParent: You forgot to execute run_side_effects_accumulate_results().");
    }
    &self.results
  }

  ///
  /// Clears out all currently pushed side effects for the Message.
  ///
  /// **Don't use this unless you know what you're doing!**
  ///
  pub fn clear_side_effects(&mut self) {
    self.side_effects.clear();
  }
}

impl<ParentType, ReturnType> Drop for MessageToParent<ParentType, ReturnType> {
  fn drop(&mut self) {
    println!("Message was dropped!")
  }
}

////////////////////////////////////////////////////////! End MessageToParent

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

    let mut mutation_attempt = self.child.as_deref().unwrap().borrow_mut().mutate_parent();

    mutation_attempt.run_side_effects(self);

    for result in mutation_attempt.get_results() {
      println!("result: {}", result);
    }
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

  pub fn mutate_parent(&mut self) -> MessageToParent<Parent, bool> {
    println!("child: Attempting to mutate parent!");
    let mut returning_message = MessageToParent::<Parent, bool>::new();

    returning_message.add_side_effect(|_| {
      println!("Parental advisory");

      false
    });

    returning_message.add_side_effect(|parent| {
      parent.parent_mutate_procedure();

      true
    });

    returning_message
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

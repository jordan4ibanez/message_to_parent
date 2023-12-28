use std::{cell::RefCell, ops::Deref, rc::Rc};

////////////////////////////////////////////////////////! End Imports

struct MessageToParent<ParentType, ReturnType> {
  side_effects: Vec<fn(&mut ParentType) -> ReturnType>,
  results: Vec<ReturnType>,
  executed: bool,
  has_results: bool,
}

impl<ParentType, ReturnType> MessageToParent<ParentType, ReturnType> {
  pub fn new() -> Self {
    MessageToParent {
      side_effects: vec![],
      results: vec![],
      executed: false,
      has_results: false,
    }
  }

  ///
  /// Create a new side effect for the Parent to run.
  ///
  /// Note: This is a "stack". The Parent will run these in the order they were added.
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
  /// Note: Only the Parent should be running this after it was received from the Child.
  ///
  /// I've made this function panic if you try to run this after you've already executed a side
  /// effect runner function so you don't make a mistake. This prevents a plethora of headaches.
  ///
  pub fn run_side_effects_accumulate_results(&mut self, parent: &mut ParentType) {
    self.has_results = true;

    if self.executed {
      panic!(
        "MessageToParent: Attempted to execute side effects after side effects already executed!"
      )
    }

    for side_effect in &self.side_effects {
      self.results.push(side_effect(parent));
    }

    self.executed = true;
  }

  ///
  /// Run all created side effects.
  ///
  /// Note: Only the Parent should be running this after it was received from the Child.
  ///
  /// I've made this function panic if you try to run this after you've already executed a side
  /// effect runner function so you don't make a mistake. This prevents a plethora of headaches.
  ///
  pub fn run_side_effects(&mut self, parent: &mut ParentType) {
    if self.executed {
      panic!(
        "MessageToParent: Attempted to execute side effects after side effects already executed!"
      )
    }

    for side_effect in &self.side_effects {
      side_effect(parent);
    }

    self.executed = true
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
  /// Clears out all currently pushed side effects for the MessageToParent.
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
  child: Child,
  is_mutant: bool,
}

impl Parent {
  pub fn new() -> Self {
    Self {
      child: Child::new(),
      is_mutant: false,
    }
  }

  pub fn main(&mut self) {
    println!("parent: running mutation on child");

    let mut mutation_attempt = self.child.mutate_parent();

    mutation_attempt.run_side_effects_accumulate_results(self);

    for result in mutation_attempt.get_results() {
      println!("result: {}", result);
    }
  }

  pub fn parent_mutate_procedure(&mut self) {
    println!("parent: Am I a mutant? {}", self.is_mutant);
  }
}

impl Drop for Parent {
  fn drop(&mut self) {
    println!("parent dropped!")
  }
}

////////////////////////////////////////////////////////! End Parent

struct Child {}

impl Child {
  pub fn new() -> Self {
    Self {}
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

    returning_message.add_side_effect(|parent| {
      println!("child: mutating parent!");
      parent.is_mutant = true;

      parent.is_mutant
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
/// child: mutate_parent() -> this passes back a MessageToParent
/// parent: runs the message -> (contains parent_mutate_procedure execution)
/// parent: parent_mutate_procedure()
fn main() {
  Rc::new(RefCell::new(Parent::new()))
    .deref()
    .borrow_mut()
    .main();
}

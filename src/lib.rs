pub struct MessageToParent<ParentType, ReturnType> {
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
}

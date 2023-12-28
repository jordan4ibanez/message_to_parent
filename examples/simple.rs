extern crate message_to_parent;

use message_to_parent::MessageToParent;

////////////////////////////////////////////////////////! End Imports

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
    println!("parent: running mutation procedure on child.");

    // Follow this function to see the example of it getting loaded into the struct.

    let mut mutation_instructions = self.child.mutate_parent();

    // Parent now has ownership of the side effect instructions, we do not have to burden the borrow checker.

    // Now that you've seen it "not running" let's actually cause those side effects.

    println!("Tutorial side effects example begin ------------");

    mutation_instructions.run_side_effects_accumulate_results(self);

    println!("Tutorial side effects example end ------------\n");

    // I've put some booleans in for the sake of having actual results we can observe for the
    // sake of the tutorial. Let's take a look!

    println!("Tutorial observing side effects begin ------------");
    for result in mutation_instructions.get_results() {
      println!("result: {}", result);
    }
    println!("Tutorial observing side effects end ------------");

    // Side effects are of course generic, and you can implement anything you want into them.
    // This can create some extremely cool scenarios.

    // That's the end of the simple tutorial.
    // The intermediate and advanced tutorials will be less documented so feel free to look
    // back upon this one to get a refresher.
  }

  pub fn parent_mutate_procedure(&mut self) {
    println!("parent: Am I a mutant? {}", self.is_mutant);
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

    // I'm going to add some markers in so you can see this happening in your terminal.
    // Keep in mind: None of this is actually running in here. Please see above for where it's
    // actually getting executed. Hence why it has "not running" in the output.

    println!("\nTutorial not running example begin ------------");

    // This will very simply tell the parent to print out if it's mutated. It's not mutated yet

    returning_message.add_side_effect(|parent| {
      parent.parent_mutate_procedure();

      false
    });

    returning_message.add_side_effect(|parent| {
      println!("child: mutating parent!");
      parent.is_mutant = true;

      parent.is_mutant
    });

    // This will very simply tell the parent to print out if it's mutated. It's now a mutant.

    returning_message.add_side_effect(|parent| {
      parent.parent_mutate_procedure();

      true
    });

    println!("Tutorial not running example end ------------\n");

    // Back to the parent we go!

    returning_message
  }
}

////////////////////////////////////////////////////////! End Child

///
/// logic flow:
///
/// parent: main() -> runs child.mutate_parent()
/// child: mutate_parent() -> this passes back a MessageToParent with side effect instructions
/// parent: runs the message instructions -> (contains parent_mutate_procedure execution)
/// parent: parent_mutate_procedure() from message -> parent is now mutated
fn main() {
  Parent::new().main();
}

extern crate message_to_parent;

use std::{cell::RefCell, rc::Rc};

use message_to_parent::MessageToParent;

////////////////////////////////////////////////////////! End Imports

struct App {
  worker: Worker,
  finalizer: Finalizer,
}

impl App {
  pub fn new() -> Self {
    App {
      worker: Worker::new(),
      finalizer: Finalizer::new(),
    }
  }

  pub fn main(&mut self) {
    // Let's start going down the chain.
    let mut worker_message = self.worker.do_work();

    worker_message.run_side_effects(self);
  }
}

////////////////////////////////////////////////////////! End App

struct Worker {
  compositor: Compositor,
}

impl Worker {
  pub fn new() -> Self {
    Worker {
      compositor: Compositor::new(),
    }
  }

  pub fn do_work(&mut self) -> MessageToParent<App, ()> {
    // Now we can create a message.
    let mut message = MessageToParent::<App, ()>::new();

    // Which we can pass down to the compositor mutably.
    self.compositor.composite(&mut message);

    message
  }
}

////////////////////////////////////////////////////////! End Worker

struct Compositor {}

impl Compositor {
  pub fn new() -> Self {
    Compositor {}
  }

  pub fn composite(&mut self, message: &mut MessageToParent<App, ()>) {
    // And now the compositor can talk up the chain: Self -> Worker -> App
    // And then back down the chain: App -> Finalizer
    // And we don't break any borrowing rules, yay!
    println!("Compositor: Attempting communication with Finalizer!");
    message.add_side_effect(|app| {
      app.finalizer.finalize("some random data from Compositor");
    });
  }
}

////////////////////////////////////////////////////////! End Compositor

struct Finalizer {}

impl Finalizer {
  pub fn new() -> Self {
    Finalizer {}
  }

  pub fn finalize(&mut self, data: &str) {
    // I have no idea what a finalizer would do but it's sure doing something.
    println!("Finalizer: Finalizing data [{}]", data);
  }
}

////////////////////////////////////////////////////////! End Finalizer

///
/// Okay I have my App, and my App has a Worker, and that Worker has a Compositor.
///
/// But my App also has a Finalizer. How do we get the Compositor to talk to the Finalizer??
///
/// Well let's see how we do this with MessageToParent.
///
fn main() {
  // I just like using RC RefCell so I'm gonna use it here for no reason. :D
  Rc::new(RefCell::new(App::new())).borrow_mut().main();
}

extern crate message_to_parent;

use message_to_parent::MessageToParent;

struct Game {
  network: Network,
  database: Database,
}

impl Game {
  pub fn new() -> Self {
    Game {
      network: Network::new(),
      database: Database::new(),
    }
  }

  pub fn main(&mut self) {
    // We'll "receive" some network data.
    let mut network_message = self.network.receive();

    network_message.run_side_effects(self);

    println!("Game: Completed network side effects");

    // Now we can have the database send some network data for some reason.

    let mut database_message = self.database.some_random_thing();

    database_message.run_side_effects(self);

    println!("Game: Completed database side effects");

    // That was easy enough. Very nice. Please see the advanced tutorial where we start getting kind of ridiculous.
  }
}

struct Network {}

impl Network {
  pub fn new() -> Self {
    Network {}
  }

  pub fn receive(&mut self) -> MessageToParent<Game, ()> {
    let mut message = MessageToParent::<Game, ()>::new();

    println!("Network: Received verification for player: [cool_guy]");

    // Okay we got a player that connected and we verified him using our imaginary verification.

    // Let's put him into the database!

    message.add_side_effect(|parent| {
      parent.database.add_player("cool_guy".to_string());
    });

    message
  }

  pub fn send(&mut self, data: String) {
    println!("Network: Sending data to some random endpoint: [{}]", data);
    // And that data literally went no where in this example, incredible.
  }
}

struct Database {
  // Why yes, this is a terrible way to store players!
  players: Vec<String>,
}

impl Database {
  pub fn new() -> Self {
    Database { players: vec![] }
  }

  pub fn add_player(&mut self, new_player: String) {
    println!("Database: Adding player [{}]", &new_player);

    self.players.push(new_player);
  }

  pub fn some_random_thing(&mut self) -> MessageToParent<Game, ()> {
    let mut message = MessageToParent::<Game, ()>::new();

    println!("Database: Sending some network data out for some reason!");

    message.add_side_effect(|parent| {
      parent.network.send("Hello from the database!".to_string());
    });

    message
  }
}

///
/// Okay so for this tutorial I want to simply have my game's network component
/// talk to the player database component.
///
/// We can't talk through the parent of the network directly. It's already borrowed mutably!
///
/// Let's see how we can solve this with MessageToParent.
///
fn main() {
  Game::new().main();
}

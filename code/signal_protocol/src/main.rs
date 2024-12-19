mod mod_group;
mod utils;
mod participant;
mod server;

use mod_group::ModGroup;
use participant::Participant;
use server::Server;

const P: u64 = 233;
const G: u64 = 3;

fn main() {
    println!("Hello, world!");
    let group = ModGroup::new(P, G);

    let alice = Participant::new("Alice", &group);
    let bob = Participant::new("Bob", &group);

    let mut server = Server::new();

    server.register(&alice);
    server.register(&bob);
}

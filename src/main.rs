mod models;
mod github;

use std::env;
use github::get_events;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} <username>", args[0]);
        return;
    }
    let username = &args[1];
    let results = get_events(username);
    match results {
        Ok(events) => {
            for event in events {
                println!("{}: '{}' event from login '{}' in the '{}'", event.created_at, event.type_field, event.actor.login, event.repo.name);
            }
        },
        Err(e) => println!("Error fetching events: {}", e),
    }
}

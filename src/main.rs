use crate::startup::load_environments;

mod webserver;
mod startup;
mod models;

fn main() {
    load_environments();
    webserver::start::main();
}
use crate::startup::load_environments;

mod webserver;
mod startup;

fn main() {
    load_environments();
    webserver::start::main();
}
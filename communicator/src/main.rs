extern crate communicator;

//Use brings a specific module into scope
use communicator::client;
//Alternatively use communicator::client::* would work 

fn main() {
    client::connect();
}
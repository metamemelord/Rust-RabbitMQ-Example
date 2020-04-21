#![allow(unused_must_use)]
mod consumer;
mod publisher;
mod user;

fn main() {
    match publisher::publish() {
        Ok(_) => {
            consumer::consume();
            println!("Done");
        }
        Err(_) => println!("Could not publish"),
    }
}

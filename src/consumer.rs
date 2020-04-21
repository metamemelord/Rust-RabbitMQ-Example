use crate::user::User;
use amiquip::{Connection, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};
use bincode::deserialize;

pub fn consume() -> Result<()> {
    let mut conn = Connection::insecure_open("amqp://localhost:5672")?;
    let ch = conn.open_channel(None)?;

    let queue = ch.queue_declare("user", QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Consumer will exit after receiving one message");

    for (_i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let usr: User = deserialize(&delivery.body).unwrap();
                consumer.ack(delivery)?;
                println!("Received: {:?}", usr);
            }
            _ => {}
        }
        break;
    }

    conn.close()
}

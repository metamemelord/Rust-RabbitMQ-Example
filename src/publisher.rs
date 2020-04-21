use crate::user::User;
use amiquip::{Connection, Exchange, Publish, Result};
use bincode;

pub fn publish() -> Result<()> {
    let usr = User {
        username: "metamemelord".to_string(),
        name: "Gaurav Saini".to_string(),
        age: 23,
    };
    let user_bytes = bincode::serialize(&usr).unwrap();

    let mut conn = Connection::insecure_open("amqp://localhost:5672")?;
    let ch = conn.open_channel(None)?;
    let exchange = Exchange::direct(&ch);
    exchange.publish(Publish::new(&user_bytes, "user"))?;

    println!("Published: {:?}", usr);
    conn.close()
}

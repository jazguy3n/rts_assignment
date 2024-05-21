use amiquip::{Connection, Exchange, Publish, ConsumerMessage, ConsumerOptions, QueueDeclareOptions, Result};

pub fn send_msg(msg: String, queue_addr: &str) -> Result<()> {
    // Open connection.
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = Exchange::direct(&channel);

    // Publish a message to the queue.
    exchange.publish(Publish::new(msg.as_bytes(), queue_addr))?;

    connection.close()
}

pub fn recv_msg(queue_name: &str) -> String {
    let mut order = "".to_string();

    // Open connection.
    let mut connection: Connection = Connection::insecure_open(
        "amqp://guest:guest@localhost:5672"
    ).unwrap();

    // Open a channel - None says let the library choose the channel ID.
    let channel: amiquip::Channel = connection.open_channel(None).unwrap();

    // Declare the queue.
    let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default()).unwrap();

    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default()).unwrap();
    // println!("Waiting for messages. Press Ctrl-C to exit.");

    for message in consumer.receiver().iter() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body);
                order = body.to_string();
                consumer.ack(delivery).unwrap();
                break;
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    let _ = connection.close();
    order
}

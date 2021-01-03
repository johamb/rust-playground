// adapted from: https://github.com/eclipse/paho.mqtt.rust/blob/master/examples/mqttrs_chat.rs

#[macro_use] extern crate paho_mqtt as mqtt;
use uuid::Uuid;

use std::{
    process,
    io,
    time::Duration,
};

fn main() -> mqtt::Result<()> {

    // Using local broker eclipse-mosquitto running in docker
    let host = "localhost";

    // default topic
    let topic_name = "default";
    let mut buf = Uuid::encode_buffer();
    let client_id = Uuid::new_v4().to_simple().encode_lower(&mut buf).to_string();

    const QOS: i32 = 1;
    const MQTTV5: u32 = 5;
    const NO_LOCAL: bool = true;

    // The LWT is a message that will be broadcasted to the subscribers when this client loses its connection

    let lwt_props = mqtt::properties!{
        mqtt::PropertyCode::WillDelayInterval => 10,
    };

    let lwt = mqtt::MessageBuilder::new()
        .topic(topic_name)
        .payload(format!("<<< client with id: {} has disconnected >>>", &client_id))
        .qos(QOS)
        .properties(lwt_props)
        .finalize();

    // Create a new client without persistence
    let create_opts = mqtt::CreateOptionsBuilder::new()
        .mqtt_version(MQTTV5)
        .server_uri(host)
        .client_id(&client_id)
        .persistence(None)
        .finalize();

    let mut client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|err| {
        eprintln!("Error creating the client: {}", err);
        process::exit(1);
    });

    let props = mqtt::properties!{
        mqtt::PropertyCode::SessionExpiryInterval => 60,
    };

    // Connect with default options
    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .mqtt_version(MQTTV5)
        .keep_alive_interval(Duration::from_secs(20))
        .clean_start(false)
        .properties(props)
        .will_message(lwt)
        .finalize();

    // Set a closure to be called when the client loses the connection.
    // It will simply end the session.
    client.set_connection_lost_callback(|_| {
        println!("*** Connection lost ***");
        process::exit(2);
    });

    // Attach a closure to the client to receive callbacks on incoming
    // messages. Just print them to the console.
    client.set_message_callback(|_client, msg| {
        if let Some(msg) = msg {
            println!("{}", msg.payload_str());
        }
    });

    // Connect and wait for it to complete or fail
    if let Err(err) = client.connect(conn_opts).wait() {
        eprintln!("Unable to connect: {}", err);
        process::exit(1);
    }

    // Subscribe to the topic
    let topic = mqtt::Topic::new(&client, topic_name, QOS);
    println!("Subscribing to topic '{}'...", topic_name);
    topic.subscribe_with_options(NO_LOCAL, None).wait()?;

    // Publish initial message
    topic.publish(format!("<<< {} subscribed to this topic >>>", client_id)).wait()?;

    // Read messages from the console and publish them.
    // Quit when the use enters an empty line, or a read error occurs.
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let msg = input.trim();
                if msg.is_empty() { break; }

                // Publish payload as "<user>: <message>"
                let chat_msg = format!("{}: {}", client_id, msg);
                if let Err(err) = topic.publish(chat_msg).wait() {
                    eprintln!("Error: {}", err);
                    break;
                }
            },
            Err(err) => println!("Error: {}", err),
        }
    }

    if client.is_connected() {
        println!("Disconnecting...");
        // Disconnect and tell the server to publish the LWT (after the expiry)
        let opts = mqtt::DisconnectOptionsBuilder::new()
            .publish_will_message()
            .finalize();
        client.disconnect(opts).wait()?;
    }

    Ok(())
}
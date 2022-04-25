mod client;
use log::{debug, error, info};
use paho_mqtt::{AsyncClient, Message};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, thread, time::Duration};

use crate::client::Client;
lazy_static::lazy_static! {
    static ref TOPIC_SUBSCRIPTION: String = {
        env::var("MQTT_TOPIC_SUBSCRIPTION").unwrap_or_else(|_| "action".into())
    };
    static ref TOPIC_PUBLISHING: String = {
        env::var("MQTT_TOPIC_PUBLISHING").unwrap_or_else(|_| "action_resp".into())
    };
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("welcome !!!");
    let client = Client::new(Default::default())?;
    client.on_message(on_message);
    client.connect(|cli, _msgid| {
        cli.subscribe(TOPIC_SUBSCRIPTION.clone(), 1);
    })?;

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

fn on_message(cli: &AsyncClient, msg: Option<Message>) {
    if let Some(msg) = msg {
        let msg: Result<Value, _> = serde_json::from_slice(msg.payload());
        if let Ok(msg) = msg {
            debug!(" Got msg : {msg:#?}");
            cli.publish(Message::new(
                TOPIC_PUBLISHING.clone(),
                serde_json::to_vec(&Hello {
                    message: "Hello".into(),
                    author: "rust".into(),
                })
                .expect("json malformed"),
                1,
            ));
        } else {
            error!("Payload not json! Not supported.");
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Hello {
    message: String,
    author: String,
}

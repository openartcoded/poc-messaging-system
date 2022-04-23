mod connect;
use connect::Connection;
use log::{debug, error, info};
use paho_mqtt::{AsyncClient, Message};
use serde_json::Value;
use std::{thread, time::Duration};
fn main() {
    env_logger::init();
    info!("welcome !!!");
    let conn = Connection::new(on_message);

    conn.connect();

    // Just wait for incoming messages.
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

fn on_message(_cli: &AsyncClient, msg: Option<Message>) {
    if let Some(msg) = msg {
        let msg: Result<Value, _> = serde_json::from_slice(msg.payload());
        if let Ok(msg) = msg {
            debug!(" Got msg : {msg:#?}");
        } else {
            error!("Payload not json! Not supported.");
        }
    }
}

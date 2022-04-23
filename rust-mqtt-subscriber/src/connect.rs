use std::{env, thread, time::Duration};

use log::{debug, error, info};
use mqtt::{AsyncClient, Message};
use paho_mqtt as mqtt;

lazy_static::lazy_static! {
    static ref TOPIC: String = {
        env::var("MQTT_TOPIC_SUBSCRIPTION").unwrap_or_else(|_| "action".into())
    };
    static ref SERVER_URI: String =  {
        let host = env::var("MQTT_HOST").unwrap_or_else(|_| "127.0.0.1".into());
        let port = env::var("MQTT_PORT").unwrap_or_else(|_| "11883".into());

        format!("tcp://{host}:{port}")
    };
    static ref CLIENT_ID: String = env::var("MQTT_CLIENT_ID").unwrap_or_else(|_| "poc_mqtt_subscriber".into());
    static ref USERNAME: String = env::var("MQTT_USERNAME").unwrap_or_else(|_| "root".into());
    static ref PASSWORD: String = env::var("MQTT_PASSWORD").unwrap_or_else(|_| "root".into());

}

pub struct Connection(AsyncClient);

impl Connection {
    pub fn new<F>(on_message: F) -> Connection
    where
        F: FnMut(&AsyncClient, Option<Message>) + 'static,
    {
        let cli = create_cli();
        configure_connection_callback(&cli);
        configure_on_message_callback(&cli, on_message);
        Connection(cli)
    }

    pub fn connect(&self) {
        connect(&self.0);
    }
}

fn on_connect_success(cli: &mqtt::AsyncClient, _msgid: u16) {
    info!("Connection succeeded");
    cli.subscribe(TOPIC.clone(), 1);
}
fn on_connect_failure(cli: &mqtt::AsyncClient, _msgid: u16, rc: i32) {
    error!("Connection attempt failed with error code {}.\n", rc);
    thread::sleep(Duration::from_millis(2500));
    cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
}

fn create_cli() -> AsyncClient {
    debug!("server_uri {}", *SERVER_URI);

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(SERVER_URI.clone())
        .client_id(CLIENT_ID.clone())
        .finalize();
    mqtt::AsyncClient::new(create_opts)
        .unwrap_or_else(|e| panic!("Error creating the client: {:?}", e))
}

fn configure_connection_callback(cli: &AsyncClient) {
    // Set a closure to be called whenever the client connection is established.
    cli.set_connected_callback(|_cli: &mqtt::AsyncClient| {
        info!("Connected.");
    });
    // Set a closure to be called whenever the client loses the connection.
    // It will attempt to reconnect, and set up function callbacks to keep
    // retrying until the connection is re-established.
    cli.set_connection_lost_callback(|cli: &mqtt::AsyncClient| {
        error!("Connection lost. Attempting reconnect.");
        thread::sleep(Duration::from_millis(2500));
        cli.reconnect_with_callbacks(on_connect_success, on_connect_failure);
    });
}

fn configure_on_message_callback<F>(cli: &AsyncClient, on_message: F)
where
    F: FnMut(&AsyncClient, Option<Message>) + 'static,
{
    cli.set_message_callback(on_message);
}

fn connect(cli: &AsyncClient) {
    debug!("username {}, password {}", *USERNAME, *PASSWORD);
    // Define the set of options for the connection
    let lwt = mqtt::Message::new("test", "Async subscriber lost connection", 1);

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .mqtt_version(mqtt::MQTT_VERSION_3_1_1)
        .clean_session(true)
        .password(USERNAME.clone())
        .user_name(PASSWORD.clone())
        .will_message(lwt)
        .finalize();

    // Make the connection to the broker
    info!("Connecting to the MQTT server...");
    cli.connect_with_callbacks(conn_opts, on_connect_success, on_connect_failure);
}

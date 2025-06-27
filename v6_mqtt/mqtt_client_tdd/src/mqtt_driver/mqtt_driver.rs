use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use std::error::Error;

#[async_trait]
trait MqttDriver : Send + Sync{
  async fn connect(&self, broker_url: &str, client_id: &str ) -> Result<(), Box<dyn Error>>;
  async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>>;
  async fn subscribe($self, topic: &str) -> Result<(), Box<dyn Error>>;
  async fn receive(&mut self) -> Option<String>;
}

pub struct MqttClient {
  driver: Box<dyn MqttDriver>;
}




pub struct RealMqttDriver {
  client: Option<AsyncClient>, 
  #[allow(dead_code)]
  _eventloop_handle: Option<tokio::task::JoinHandle<()>>,
  receiver: mpsc::Receiver<String>,
  sender: mpsc::Sender<String>,
}
impl RealMqttDriver {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::channel(100);
    RealMqttDriver {
      client: None,
      _eventloop_handle: None,
      receiver,
      sender,
    }
  }
}

#[async_trait]
impl MqttDriver for RealMqttDriver {
  async fn connect(&self, broker_url: &str, client_id: &str) -> Result<(), Box<dyn Error>> {
    let mut mqtt_options = MqttOptions::new(client_id, broker_url.to_string(), 1883);
    mqtt_options.set_keep_alive( Duration::from_secs(5) );
    let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    let event_sender
  }
}


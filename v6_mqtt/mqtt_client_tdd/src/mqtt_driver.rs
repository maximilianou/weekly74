pub mod mqtt_driver {

use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use rumqttc::Incoming;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;
use std::error::Error;

#[async_trait]
trait MqttDriver : Send + Sync {
  async fn connect(&self, broker_url: &str, client_id: &str ) -> Result<(), Box<dyn Error>>;
  async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>>;
  async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn Error>>;
  async fn receive(&mut self) -> Option<String>;
}

pub struct MqttClient {
  driver: Box<dyn MqttDriver>,
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
    let (_client, mut eventloop) = AsyncClient::new(mqtt_options, 10);
    let event_sender = self.sender.clone();
    let handle = task::spawn(async move {
      loop {
        match eventloop.poll().await {
          Ok(rumqttc::Event::Incoming(Incoming::Publish(p))) => {
            let payload_str = String::from_utf8_lossy(&p.payload).to_string();
            println!("RealMqttDriver received message on topic '{}' : '{}'", p.topic, payload_str);
            if let Err(e) = event_sender.send(payload_str).await {
              eprintln!("Failed to send received message to internal channel: {}", e);
              break;
            }
          },
          Ok(rumqttc::Event::Incoming(Incoming::ConnAck(_))) => {
            println!("RealMqttDriver connected!");
          },
          Ok(_) => {
            // Ignora altri messagi
          },
          Err(e) => {
            eprintln!("RealMqttDriver eventloop error: {:?}", e);
            break;
          }
        }
      }
      println!("RealMqttDriver eventloop terminated!");
    });
    Ok(())
  }

   async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>> {
    if let Some(client) = &self.client {
      client.publish(topic, QoS::AtLeastOnce, false, payload).await?;
      println!("RealMqttDriver published to topoc: {}", topic);
      Ok(())
    }else{
      Err("Client not connected".into())
    }
  }

   async fn subscribe(&self, topic: &str ) -> Result<(), Box<dyn Error>> {
    if let Some(client) = &self.client {
      client.subscribe(topic, QoS::AtLeastOnce).await?;
      println!("RealMqttDriver subscribed to topic :  {}", topic);
      Ok(())
    } else {
      Err("Error clienet not connected".into())
    }

  }

  async fn receive(&mut self ) -> Option<String> {
    self.receiver.recv().await
  }

}



}

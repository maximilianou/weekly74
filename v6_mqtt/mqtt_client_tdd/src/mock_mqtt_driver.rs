use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{self, Duration}; 
use async_trait::async_trait;


pub struct MockMqttDriver {
  pub published_messages: mpsc::Sender<(String, Vec<u8>)>,
  pub incoming_sender: mpsc::Sender<String>,
  pub incoming_receiver: mpsc::Receiver<String>,
  pub subscriptions: HashMap<String, ()>,

}

impl MockMqttDriver {
  pub fn new() -> (Self, mpsc::Receiver<(String, Vec<u8>)>) {
    let (pub_tx, pub_rx) = mpsc::channel(100);
    let (inc_tx, inc_rx) = mpsc::channel(100); 
    (
      MockMqttDriver {
        published_messages: pub_tx,
        incoming_sender: inc_tx,
        incoming_receiver: inc_rx,
        subscriptions: HashMap::new(),
      },
      pub_rx,
    )
  }

  pub async fn simulate_incoming_message(&self, message: String){
    self.incoming_sender.send(message).await.expect("Failed to send mock incoming message");
  }

}

#[async_trait]
impl MqttDriver for MockMqttDriver {
  async fn connect(&self, broker_url: &str, client_id: &str) -> Result<(), Box<dyn Error>>{
    println!("MockMqttDriver: simulating connection to {} with Id: {}", broker_url, client_id);
    Ok(())
  }
  async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>>{
    println!("MockMqttDriver: simulating publish to topic {} with payload {}", topic, 
             std::str::from_utf8(payload).unwrap_or("INVALID UTF8"));
    self.publish_messages.send( (topic.to_string(), payload.to_vec()) ).await?;
    Ok(())
  }
  async fn subscribe(&self, topic: &str ) -> Result<(), Box<dyn Error>>{
    println!("MockMqttDriver: Simulating subscribe to topic: {}");
    self.subscriptions.insert(topic.to_string());
    Ok(())
  }
  async fn receive(&mut self) -> Option<String> {
    time::sleep(Duration::from_millis(10)).await;
    self.incoming_receiver.recv().await
  }
}




use std::error::Error;
use tokio::sync::mpsc;
pub mod mqtt_driver;
pub struct MqttClient {
  _inner_client: (),
  _broker_url: String,
  _client_id: String,
  _sender: mpsc::Sender<String>,
  _receiver: mpsc::Receiver<String>,  
}

impl MqttClient {
  pub async fn new(broker_url: &str, client_id: &str) -> Result<Self, Box<dyn Error>>{

    let (tx, rx) = mpsc::channel(100);
    Ok(Self {
      _inner_client: (),
      _broker_url: broker_url.to_string(),
      _client_id: client_id.to_string(),
      _sender: tx,
      _receiver: rx, 
    }) 

  }

  pub async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn Error>>{
    println!("Simulating publish to topic: {} with payload: {}", topic, std::str::from_utf8(payload).unwrap_or("Invalid UTF8"));
    Ok(())
  }

  pub async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn Error>>{
    println!("Simulating subscribe to topic: {}", topic);
    Ok(())
  }

  pub async fn receive(&mut self) -> Option<String>{
    self._receiver.recv().await
  }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_create_a_client() {
      let broker_url = "mqtt://localhost:1883/"; 
      let client_id  = "test_client_id";
      let client = MqttClient::new(broker_url, client_id).await;
      assert!(client.is_ok(), "Should be able to create a client");
    }

    #[tokio::test]
    async fn it_publishes_a_message(){
      let broker_url = "mqtt://localhost:1883";
      let client_id  = "publisher_test_client";
      let client = MqttClient::new(broker_url, client_id).await.expect("Failed to create client");
      let topic = "test/topic";
      let payload = b"Hello MQTT";
      let result = client.publish(topic, payload).await;
      assert!(result.is_ok(), "Should be able to publish a message");
    }
    
    #[tokio::test]
    async fn it_subscribes_and_receives_messages(){
      let broker_url = "mqtt://localhost:1883/"; 
      let client_id = "subscriber_test_client";
      let mut client = MqttClient::new(broker_url, client_id).await.expect("Failed to create client");
      let topic = "test/messages";
      let test_message = "hello from publisher";
      let sub_result = client.subscribe(topic).await;
      assert!(sub_result.is_ok(), "Should be able to subscribe");
      client._sender.send(test_message.to_string()).await.expect("Filed to send a mock message");
      let received_message = tokio::time::timeout(std::time::Duration::from_secs(1), client.receive()).await;
      assert!(received_message.is_ok(), "Should not timeout");
      assert!(received_message.as_ref().unwrap().is_some(), "Should receive a message");
      assert_eq!(received_message.unwrap().unwrap(), test_message.to_string(), "Received message should match sent message");
    }


}

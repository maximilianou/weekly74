use std::error::Error;

pub struct MqttClient {
  driver: Box<dyn MqttDriver>,
  client_id: String,

} 

impl MqttClient {
  pub fn new(driver: Box<dyn MqttDriver>, client_id: &str) -> Self {
    MqttClient {
      driver,
      client_id: client_id.to_string(), 
    }
  } 

  pub async fn connect(&self, broker_url: &str) -> Result<(), Box<dyn Error>>{
    self.driver.connect(broker_url, &self.client_id).await
  }

  pub async fn publish(&self, topic: &str, payload: &str) -> Result<(), Box<dyn Error>>{
    self.driver.publish( topic, payload ).await 
  }

  pub async fn subscribe(&self, topic: &str ) -> Result<(), Box<dyn Error>>{
    self.driver.subscribe(topic).await
  }
  pub async fn receive(&mut self) -> Option<String> {
    self.driver.receive().await
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::time::{self, Duration};
  
  #[tokio::test] 
  async fn mqtt_client_publishes_message_with_mock(){
    let (mock_driver, mut published_rx) = MockMqttDriver::new();
    let client_id = "test_publisher";
    let mqtt_client  = MqttClient::new(Box::new(mock_driver), client_id);
    let topic = "test/topic";
    let payload = b"Hello From TDD!";
    let result = mqtt_client.publish(topic, payload).await;
    assert!(result.is_ok(), "Should be able to publish messages");
    let received = time::timeout(Duration::from_secs(1), published_rx.recv()).await;
    assert!(received.is_ok(), "Should not timeout waiting from published messages");
    let (rec_topic, rec_payload) =   received.unwrap().expect("No message received");
    assert_eq!(rec_topic, topic);
    assert_eq!(rec_payload, payload.to_vec());
  }

  #[tokio::test]
  async fn mqtt_client_subscribes_and_receives_with_mock(){
    let (mock_driver, mock_published_rx) = MockMqttDriver::new();
    let mock_driver_clone = mock_driver.incoming_sender.clone();
    let client_id  = "test_subscriber";
    let mut mqtt_client = MqttClient::new(Box::new(mock_driver), client_id); 
    let topic = "notifications/alerts";
    let test_message = "System message: High CPU usage"; 
    let sub_result = mqtt_client.subscribe(topic).await;
    assert!(sub_result.is_ok(), "Should be able to subscribe");
    mock_driver_clone.send(test_manager.to_string()).await.expect("Failed to send mock message");
    let received = time::timeout(Duration::from_secs(1), mqtt_client.receive()).await;
    assert!(received.is_ok(), "Should not timeout waiting for received messages " );
    let msg = received.unwrap().expect("No message Received");
    assert_eq!(msg, test_message);


 
  }




}

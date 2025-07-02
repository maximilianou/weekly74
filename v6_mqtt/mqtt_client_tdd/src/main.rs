use std::error::Error;
use std::time::Duration;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  println!("Starting RealMqttDriver example..");
  let broker_url = "localhost";
  let client_id = "my_rust_client";
  let mut real_driver = RealMqttDriver::new();
  let mut mqtt_driver = MqttClient::new(Box::new(real_driver), client_id);
  println!("Attenpting to connect..");
  mqtt_client.connect(broker_url).await;
  println!("Connected!");
  let topic_pub = "rust/messages/hello";
  let payload_pub = b"Hello from Rust Real Client!"; 
  mqtt_client.publish( topic_pub, payload_pub ).await?;
  println!("Published Message");
  let topic_sub = "rust/messages/#";
  mqtt_client.subscribe(topic_sub).await?;
  println!("Subscribe to : {}", topic_sub);
  println!("Listening for incoming messages... ( press Ctrl+C to exit ) ");
  loop {
    if let Some(message) = mqtt_client.receive().await {
      println!("Received: {}", message);
    }else{
      println!("Disconnected from broker or event loop terminated.");
      break;

    }
    tokio::time::sleep(Duration::from_millis(100)).await;
  }
  Ok(())
  
}

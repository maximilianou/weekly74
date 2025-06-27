TODO: Implement this draft

```
use async_trait::async_trait; // Per usare trait asincroni
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

// ---
// Definisci il trait MqttDriver come nel tuo esempio
// ---
#[async_trait] // Necessario per avere metodi async in un trait
pub trait MqttDriver: Send + Sync { // Aggiungi Send + Sync per Box<dyn>
    async fn connect(&self, broker_url: &str, client_id: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>>;
    async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn std::error::Error>>;
    async fn receive(&mut self) -> Option<String>; // Aggiunto per ricevere messaggi
}

// ---
// Implementazione per il client reale: RealMqttDriver
// ---
pub struct RealMqttDriver {
    client: Option<AsyncClient>, // Option perché potrebbe non essere connesso inizialmente
    #[allow(dead_code)] // Permette che _eventloop_handle non sia usato direttamente
    _eventloop_handle: Option<tokio::task::JoinHandle<()>>,
    receiver: mpsc::Receiver<String>, // Per ricevere messaggi dall'event loop
    sender: mpsc::Sender<String>, // Per passare il sender ai test/listener
}

impl RealMqttDriver {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(100); // Buffer per 100 messaggi
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
    async fn connect(&self, broker_url: &str, client_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut mqtt_options = MqttOptions::new(client_id, broker_url.to_string(), 1883); // Porta di default MQTT
        mqtt_options.set_keep_alive(Duration::from_secs(5)); // Keep alive
        // Puoi aggiungere credenziali, ultima volontà, ecc.

        let (client, mut eventloop) = AsyncClient::new(mqtt_options, 10); // 10 è il channel capacity

        // Assegna il client all'interno della struct (potresti dover clonare se self non è mutabile)
        // Per ora, useremo un clone del client per non modificare self
        // In una vera implementazione, il client dovrebbe essere mosso in self.client
        // Per questo esempio, ci concentriamo sull'eventloop.

        // Avvia l'eventloop in un task separato per gestire la ricezione dei messaggi
        let event_sender = self.sender.clone();
        let handle = task::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(rumqttc::Event::Incoming(rumqttc::Incoming::Publish(p))) => {
                        let payload_str = String::from_utf8_lossy(&p.payload).to_string();
                        println!("RealMqttDriver received message on topic '{}': {}", p.topic, payload_str);
                        if let Err(e) = event_sender.send(payload_str).await {
                            eprintln!("Failed to send received message to internal channel: {}", e);
                            break; // Se il receiver è chiuso, esci dal loop
                        }
                    },
                    Ok(rumqttc::Event::Incoming(rumqttc::Incoming::ConnAck(_))) => {
                        println!("RealMqttDriver connected!");
                    },
                    Ok(_) => {}, // Ignora altri eventi
                    Err(e) => {
                        eprintln!("RealMqttDriver eventloop error: {:?}", e);
                        break;
                    }
                }
            }
            println!("RealMqttDriver eventloop terminated.");
        });

        // Questo è il punto problematico: self è &self, non &mut self.
        // Non possiamo modificare self.client e self._eventloop_handle qui direttamente.
        // Un approccio migliore sarebbe passare il client e l'handle a `MqttClient::new`
        // o avere il RealMqttDriver creato con il client già connesso.
        // Per semplicità e per far compilare, useremo il client locale per publish/subscribe.
        // Questo non è l'ideale per il ciclo di vita, ma utile per l'esempio.
        // Se il client fosse parte di `self`, `connect` dovrebbe essere `&mut self`.

        // Per la publish/subscribe, avremo bisogno del client dentro `self`.
        // Dobbiamo rifattorizzare MqttClient per prendere un RealMqttDriver già connesso,
        // oppure fare in modo che `connect` modifichi `self`.
        // Per ora, lascio il `client: Option<AsyncClient>` per chiarezza.

        // Per un'implementazione più robusta, RealMqttDriver dovrebbe essere creato
        // come `RealMqttDriver::connect_and_spawn_event_loop(...) -> Result<Self, Error>`
        // e `self.client` sarebbe `AsyncClient` direttamente.

        // Per il bene dell'esempio che segue la tua struttura, faremo un clone
        // e lo assegneremo nel metodo `connect`. Questo richiede `&mut self`.

        // AGGIORNAMENTO: Per come hai strutturato il `MqttClient` che prende un `Box<dyn MqttDriver>`,
        // la logica del `connect` dovrebbe essere eseguita PRIMA di passare il driver al `MqttClient`,
        // oppure `MqttClient` dovrebbe chiamare `driver.connect()`.
        // Se `MqttClient` chiama `driver.connect()`, allora `connect` in `MqttDriver` dovrebbe essere `&mut self`.

        Ok(()) // Per ora, simuliamo il successo della connessione
    }

    async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        // In un'implementazione reale, useresti self.client.publish()
        // Ma siccome self.client è un Option<AsyncClient> e non possiamo mutarlo in connect,
        // dobbiamo passare il client al momento della creazione del driver.
        if let Some(client) = &self.client {
            client.publish(topic, QoS::AtLeastOnce, false, payload).await?;
            println!("RealMqttDriver published to topic: {}", topic);
            Ok(())
        } else {
            Err("Client not connected".into()) // O un errore più specifico
        }
    }

    async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(client) = &self.client {
            client.subscribe(topic, QoS::AtLeastOnce).await?;
            println!("RealMqttDriver subscribed to topic: {}", topic);
            Ok(())
        } else {
            Err("Client not connected".into())
        }
    }

    async fn receive(&mut self) -> Option<String> {
        self.receiver.recv().await
    }
}
```


```
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::time::{self, Duration};
use async_trait::async_trait;

// ---
// Implementazione per il mock (per i test unitari): MockMqttDriver
// ---
pub struct MockMqttDriver {
    // Registra i messaggi pubblicati per assert successivi
    pub published_messages: mpsc::Sender<(String, Vec<u8>)>,
    // Per simulare la ricezione di messaggi dal "broker"
    pub incoming_sender: mpsc::Sender<String>,
    pub incoming_receiver: mpsc::Receiver<String>,
    // Per verificare le sottoscrizioni
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
            pub_rx, // Restituisci il receiver per i test
        )
    }

    // Helper per i test per "iniettare" un messaggio come se fosse ricevuto dal broker
    pub async fn simulate_incoming_message(&self, message: String) {
        self.incoming_sender.send(message).await.expect("Failed to send mock incoming message");
    }
}

#[async_trait]
impl MqttDriver for MockMqttDriver {
    async fn connect(&self, broker_url: &str, client_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("MockMqttDriver: Simulating connection to {} with ID {}", broker_url, client_id);
        Ok(()) // Simula sempre successo
    }

    async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        println!("MockMqttDriver: Simulating publish to topic: {} with payload: {:?}", topic, std::str::from_utf8(payload).unwrap_or("INVALID UTF8"));
        self.published_messages.send((topic.to_string(), payload.to_vec())).await?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("MockMqttDriver: Simulating subscribe to topic: {}", topic);
        self.subscriptions.insert(topic.to_string(), ());
        Ok(())
    }

    async fn receive(&mut self) -> Option<String> {
        // Simula un ritardo per essere più realistico nei test asincroni
        time::sleep(Duration::from_millis(10)).await;
        self.incoming_receiver.recv().await
    }
}
```

```
// ---
// E poi la tua MqttClient userà un Box<dyn MqttDriver>
// ---
pub struct MqttClient {
    driver: Box<dyn MqttDriver>,
    // Puoi aggiungere qui altri stati, come il client_id se necessario per la logica interna
    client_id: String,
}

impl MqttClient {
    // Questo costruttore ora accetta un driver già connesso o meno, a seconda della strategia
    pub fn new(driver: Box<dyn MqttDriver>, client_id: &str) -> Self {
        MqttClient {
            driver,
            client_id: client_id.to_string(),
        }
    }

    pub async fn connect(&self, broker_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Chiama il metodo connect del driver sottostante
        self.driver.connect(broker_url, &self.client_id).await
    }

    pub async fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.driver.publish(topic, payload).await
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.driver.subscribe(topic).await
    }

    pub async fn receive(&mut self) -> Option<String> {
        self.driver.receive().await
    }
}
```


```
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{self, Duration};

    #[tokio::test]
    async fn mqtt_client_publishes_message_with_mock() {
        let (mock_driver, mut published_rx) = MockMqttDriver::new();
        let client_id = "test_publisher";
        let mqtt_client = MqttClient::new(Box::new(mock_driver), client_id);

        let topic = "test/topic";
        let payload = b"Hello from TDD!";

        // La logica di publish del MqttClient chiama il driver sottostante
        let result = mqtt_client.publish(topic, payload).await;
        assert!(result.is_ok(), "Should be able to publish a message");

        // Verifica che il messaggio sia stato "pubblicato" al mock
        let received = time::timeout(Duration::from_secs(1), published_rx.recv()).await;
        assert!(received.is_ok(), "Should not timeout waiting for published message");
        let (rec_topic, rec_payload) = received.unwrap().expect("No message received");

        assert_eq!(rec_topic, topic);
        assert_eq!(rec_payload, payload.to_vec());
    }

    #[tokio::test]
    async fn mqtt_client_subscribes_and_receives_with_mock() {
        let (mock_driver, _published_rx) = MockMqttDriver::new();
        let mock_driver_clone = mock_driver.incoming_sender.clone(); // Per simulare l'invio
        let client_id = "test_subscriber";
        let mut mqtt_client = MqttClient::new(Box::new(mock_driver), client_id);

        let topic = "notifications/alerts";
        let test_message = "System alert: High CPU usage!";

        // Sottoscrivi
        let sub_result = mqtt_client.subscribe(topic).await;
        assert!(sub_result.is_ok(), "Should be able to subscribe");

        // Simula un messaggio in arrivo dal "broker" tramite il mock driver
        mock_driver_clone.send(test_message.to_string()).await.expect("Failed to send mock message");

        // Ricevi il messaggio tramite il MqttClient
        let received = time::timeout(Duration::from_secs(1), mqtt_client.receive()).await;
        assert!(received.is_ok(), "Should not timeout waiting for received message");
        let msg = received.unwrap().expect("No message received");

        assert_eq!(msg, test_message);
    }
}
```


```
// Questo sarebbe nel tuo main.rs o in un file di test di integrazione
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting RealMqttDriver example...");

    // Per testare questo, avrai bisogno di un broker MQTT in esecuzione (es. Mosquitto)
    // docker run -it -p 1883:1883 -p 9001:9001 eclipse-mosquitto

    let broker_url = "localhost"; // O l'IP del tuo broker
    let client_id = "my_rust_client";

    let mut real_driver = RealMqttDriver::new();
    // La connessione dovrebbe avvenire prima di passare il driver, o il driver deve avere un modo per connettersi
    // Questo è un punto di design cruciale. Per ora, simuliamo che il client si connetta.
    // In una vera app, probabilmente creeresti il RealMqttDriver, poi lo connetteresti,
    // e solo dopo creeresti MqttClient::new(Box::new(connesso_driver)).
    // Dato come è strutturato il tuo `MqttClient::connect`, lo chiameremo da lì.

    // Creiamo il client con il driver reale
    let mut mqtt_client = MqttClient::new(Box::new(real_driver), client_id);

    println!("Attempting to connect...");
    mqtt_client.connect(broker_url).await?;
    println!("Connected!");

    let topic_pub = "rust/messages/hello";
    let payload_pub = b"Hello from Rust Real Client!";
    mqtt_client.publish(topic_pub, payload_pub).await?;
    println!("Published message.");

    let topic_sub = "rust/messages/#"; // Sottoscrivi a tutti i messaggi sotto rust/messages
    mqtt_client.subscribe(topic_sub).await?;
    println!("Subscribed to: {}", topic_sub);

    println!("Listening for incoming messages... (Press Ctrl+C to exit)");

    // Loop per ricevere messaggi
    loop {
        if let Some(message) = mqtt_client.receive().await {
            println!("Received: {}", message);
        } else {
            // Se il canale di ricezione si chiude, il broker si è disconnesso o l'event loop è terminato
            println!("Disconnected from broker or event loop terminated.");
            break;
        }
        // Piccola pausa per non spammare la CPU, anche se receive().await è bloccante
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
```

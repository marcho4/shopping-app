use std::sync::Arc;
use std::time::Duration;
use log::info;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;
use crate::models::outbox_task::OutboxTask;

pub struct KafkaRepo {
    broker: String,
    outbox_topic: String,
    inbox_topic: String,
    producer: FutureProducer,
    consumer: Arc<StreamConsumer>
}

impl KafkaRepo {
    pub fn new(kafka_url: String, kafka_port: String) -> Self {
        let broker = kafka_url + ":" + &kafka_port;
        let group_id = "orders_consumers";

        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", broker.clone())
            .set("enable.idempotence", "true")
            .set("acks", "all")
            .set("retries", "5")
            .create()
            .unwrap();
        
        let consumer : StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", broker.clone())
            .set("group.id", group_id)
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .set("isolation.level", "read_committed")
            .set("allow.auto.create.topics", "true")
            .create()
            .expect("Failed to create transactional Kafka consumer");
        
        Self {
            broker,
            outbox_topic: "outbox".to_string(),
            inbox_topic: "inbox".to_string(),
            producer,
            consumer: Arc::new(consumer)
        }
    }

    pub async fn init(&self) {
        let admin: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", self.broker.as_str())
            .create()
            .unwrap();

        let topic = NewTopic::new(self.inbox_topic.as_str(), 3, TopicReplication::Fixed(1));

        admin.create_topics(&[topic], &AdminOptions::new()).await.unwrap();
        info!("Kafka topic created");
        self.consumer.subscribe(&[self.inbox_topic.as_str()]).unwrap();
        info!("Kafka topic subscribed");
    }

    pub async fn send_task(&self, task: &OutboxTask) -> Result<(), KafkaError> {
        let message_id = task.id.to_string().clone();

        let payload = serde_json::to_string(task).unwrap();

        let record = FutureRecord::to(self.outbox_topic.as_str())
            .payload(&payload)
            .key(message_id.as_str());
        
        match self.producer
            .send(record, Timeout::After(Duration::from_secs(3)))
            .await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.0)
        }
    }

    pub fn get_consumer(&self) -> Arc<StreamConsumer> {
        self.consumer.clone()
    }
}
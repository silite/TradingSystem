use dashmap::DashMap;

use super::Event;

trait EventSender: Sync + Send {
    fn send(&self, event: Event) -> anyhow::Result<()>;
}
struct CrossbeamSender(crossbeam::channel::Sender<Event>);
struct TokioSender(tokio::sync::mpsc::UnboundedSender<Event>);

impl EventSender for CrossbeamSender {
    fn send(&self, event: Event) -> anyhow::Result<()> {
        self.0.send(event)?;
        Ok(())
    }
}
impl EventSender for TokioSender {
    fn send(&self, event: Event) -> anyhow::Result<()> {
        self.0.send(event)?;
        Ok(())
    }
}

/// 事件总线，解耦各个模块，并异步处理事件
pub struct EventBus {
    senders: DashMap<String, Box<dyn EventSender>>,
}
impl EventBus {
    pub fn new() -> Self {
        EventBus {
            senders: DashMap::new(),
        }
    }

    pub fn subscribe(&self, topic: String) -> crossbeam::channel::Receiver<Event> {
        let (sender, receiver) = crossbeam::channel::unbounded();
        self.senders
            .insert(topic, Box::new(CrossbeamSender(sender)));
        receiver
    }

    pub fn subscribe_sync(&self, topic: String) -> tokio::sync::mpsc::UnboundedReceiver<Event> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        self.senders.insert(topic, Box::new(TokioSender(sender)));
        receiver
    }

    pub fn publish(&self, topic: &str, event: Event) -> anyhow::Result<()> {
        if let Some(sender) = self.senders.get(topic) {
            sender.send(event)?
        } else {
            ftlog::error!("No topic {} event: {:?}", topic, event);
        }
        Ok(())
    }
}

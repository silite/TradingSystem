use dashmap::DashMap;

use super::Command;

pub struct CommandBus {
    senders: DashMap<String, tokio::sync::mpsc::UnboundedSender<Command>>,
}
impl CommandBus {
    pub fn new() -> Self {
        CommandBus {
            senders: DashMap::new(),
        }
    }

    pub fn subscribe(&self, topic: String) -> tokio::sync::mpsc::UnboundedReceiver<Command> {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        self.senders.insert(topic, sender);
        receiver
    }

    pub fn publish(&self, topic: &str, event: Command) -> anyhow::Result<()> {
        if let Some(sender) = self.senders.get(topic) {
            sender.send(event)?
        } else {
            ftlog::error!("No topic {} event: {:?}", topic, event);
        }
        Ok(())
    }
}

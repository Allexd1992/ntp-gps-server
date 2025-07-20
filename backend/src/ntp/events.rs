use tokio::sync::mpsc::{unbounded_channel,UnboundedReceiver,UnboundedSender};

use super::NtpTimestamp;




#[derive(Debug, Clone)]
pub struct Event {
    pub(crate) event_type:EUdpEvents
}
#[derive(Debug, Clone)]
pub struct EventManager {
    subscribers: Vec<UnboundedSender<Event>>,
}

impl EventManager {
    pub fn new() -> EventManager {
        EventManager { subscribers: Vec::new() }
    }

    pub fn subscribe(&mut self) -> UnboundedReceiver<Event> {
        let (sender, receiver ): (UnboundedSender<Event>, UnboundedReceiver<Event>) = unbounded_channel();
        self.subscribers.push(sender);
        receiver
    }

    pub fn notify(&self, event: Event) {
        for subscriber in &self.subscribers {
          let result =subscriber.send(event.clone());
          if let Err(err)=result{
            error!("{}",err.to_string());
          }
        }
    }
}


#[derive(Debug, Clone)]
pub enum EUdpEvents{
    NewPackets(Vec<u8>),
    NewGPSTimestamp(NtpTimestamp),
    NewRemoteTimestamp(NtpTimestamp),
    NewGpsSky(u16),
}

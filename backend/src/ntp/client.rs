use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;
use tokio::time::sleep;

use super::events::{Event, EventManager, EUdpEvents};
use super::NtpTimestamp;
#[derive(Clone)]
pub struct Client {
    list: Arc<Mutex<Vec<String>>>,
    event_manager: Arc<Mutex<EventManager>>,
    cycle: u16,
}

impl Client {
    pub fn new(list: Arc<Mutex<Vec<String>>>, cycle: u16) -> Self {
        Self {
            list,
            event_manager: Arc::new(Mutex::new(EventManager::new())),
            cycle,
        }
    }

    pub async fn subscribe(&self) -> UnboundedReceiver<Event> {
        self.event_manager.lock().await.subscribe()
    }

    pub async fn start(&mut self) {
        let cycle_time= self.cycle;
        let event_manager = Arc::clone(&self.event_manager);
        let arc_list = Arc::clone(&self.list);
        tokio::spawn(async move {
            loop {
                let result = get_ntp(Arc::clone(&arc_list)).await;
                debug!("{:?}",result);
                if let Some(timestamp) = result {
                    event_manager.lock().await.notify(Event {
                        event_type: EUdpEvents::NewRemoteTimestamp(
                            NtpTimestamp::new(timestamp.ts))})
                    };
                    sleep(Duration::from_millis(cycle_time.into())).await;
                }
               
            }
        
        );
    }
}


pub fn _get_ntp(url: String) -> Option<NtpTimestamp> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Unable to crate UDP socket");
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .expect("Unable to set UDP socket read timeout");
    let result = sntpc::simple_get_time(url.as_str(), socket);
    match result {
        Ok(time) => Some(NtpTimestamp::new(time.seconds.into())),
        Err(err) => None,
    }
}

async fn get_ntp(list: Arc<Mutex<Vec<String>>>) -> Option<NtpTimestamp> {
    let mut result = None;
    let mut i = 0;
    let list = list.lock().await;
    while result.is_none() && i < list.len() {
        let item = list.get(i);
        if let Some(url) = item {
            result = _get_ntp(url.to_string());
        } else {
            result = None;
        }
        i += 1;
    }

    result
}

use std::sync::{Arc};
use tokio::sync::Mutex;

use crate::{settings::interfaces::IStore, services::{login::LoginSRC, network::NetworkSRC}, ntp::request::MonitorSender, diagnostic::types::MonitoringPacket};

use super::interfaces::Iapi;



pub struct AppState {
    pub store: Arc<Mutex<dyn Iapi>>,
    pub driver: Arc<Mutex<dyn IStore>>,
    pub login_detector: Arc<Mutex<LoginSRC>>,
    pub network: Arc<Mutex<NetworkSRC>>,
    pub monitor: Arc<Mutex<MonitorSender>>,
    pub info: Arc<Mutex<MonitoringPacket>>
}

impl AppState {
    pub fn new(store: Arc<Mutex<dyn Iapi>>, driver: Arc<Mutex<dyn IStore>>,login_detector: Arc<Mutex<LoginSRC>>,network: Arc<Mutex<NetworkSRC>>, monitor: Arc<Mutex<MonitorSender>>)-> Self {
        Self {
            store: Arc::clone(&store), driver, login_detector,network,monitor,info:Arc::new(Mutex::new(MonitoringPacket::new()))
        }
    }
}

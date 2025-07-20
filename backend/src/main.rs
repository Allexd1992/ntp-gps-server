#[macro_use]
extern crate log;

use std::env;
use std::sync::Arc;
use std::time::Duration;

use env_logger::Env;
use ntp::request::MonitorSender;
use ntp::NtpTimestamp;
use rocket::data::N;
use rocket::Config;
use settings::interfaces::IStore;
use settings::store::Keeper;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::sleep;

use crate::diagnostic::types::DiagnosticPacket;
use crate::http::api::Api;
use crate::http::context::get_rocket;
use crate::http::interfaces::Iapi;
use crate::ntp::NtpClient;
use crate::ntp::NtpConnectorGPS;
use crate::ntp::NtpServer;
use crate::services::login::LoginSRC;
use crate::services::network::NetworkSRC;
mod http;
mod ntp;
mod services;
mod settings;
use settings::store::Settings;
mod diagnostic;
use diagnostic::types::MonitoringPacket;
#[tokio::main]
async fn main() {
    let mut diag: Arc<Mutex<MonitoringPacket>> = Arc::new(Mutex::new(MonitoringPacket::new()));
    let drviver: Arc<Mutex<dyn IStore>> = Arc::new(Mutex::new(Keeper::new(
        String::from("settings"),
        String::from("config"),
    )));
    let settings = drviver.lock().await.Restore().await.unwrap();
    let env = Env::default().filter_or("MY_LOG_LEVEL", "info");
    let monitor = Arc::new(Mutex::new(MonitorSender {
        last_ntp: NtpTimestamp::new(1),
        last_gps: NtpTimestamp::new(1),
        actial: NtpTimestamp::new(1),
        satilite: 0,
    }));

    env_logger::init_from_env(env);

    let mut server = Arc::new(Mutex::new(
        NtpServer::new(vec!["0.0.0.0".to_string()], true).await,
    ));
    server.lock().await.run().await;
    let ts = monitor.lock().await.get_actual_data().await;
    if let Ok(timestamp) = ts {
        if timestamp != 0 {
            server
                .lock()
                .await
                .update_state(NtpTimestamp { ts: timestamp })
                .await;
        }
    }
    let rtc_enable_gps = settings.rtc.enable;
    let mut gps: NtpConnectorGPS = NtpConnectorGPS::new();
    let mut gps_sub = gps.subscribe().await;
    let arc_server = Arc::clone(&server);
    let arc_01 = Arc::clone(&monitor);
    task::spawn(async move {
        while let Some(event) = gps_sub.recv().await {
            match event.event_type {
                ntp::events::EUdpEvents::NewGPSTimestamp(timestamp) => {
                    trace!("GPS:{:?}", timestamp);
                    let mut mon = arc_01.lock().await;
                    mon.last_gps = timestamp;
                    mon.actial = timestamp;
                    arc_server.lock().await.update_state(timestamp).await;
                    if rtc_enable_gps {
                        mon.save_actual_data();
                    }
                }
                ntp::events::EUdpEvents::NewGpsSky(numb) => {
                    let mut mon = arc_01.lock().await;
                    mon.satilite = numb;
                    trace!("GPS Satelite:{:?}", numb);
                }
                _ => (),
            }
        }
    });
    if settings.gps.enable {
        gps.start().await;
    }

    let mut ntp = NtpClient::new(
        Arc::new(Mutex::new(settings.ntp.server_list.clone())),
        settings.ntp.cycle as u16,
    );
    let rtc_enable_ntp = settings.rtc.enable;
    let arc_02 = Arc::clone(&monitor);
    let mut ntp_sub = ntp.subscribe().await;
    let arc_server2 = Arc::clone(&server);
    task::spawn(async move {
        while let Some(event) = ntp_sub.recv().await {
            match event.event_type {
                ntp::events::EUdpEvents::NewRemoteTimestamp(timestamp) => {
                    trace!("NTP:{:?}", timestamp);
                    let mut mon = arc_02.lock().await;
                    mon.last_ntp = timestamp;
                    mon.actial = timestamp;
                    arc_server2.lock().await.update_state(timestamp).await;
                    if rtc_enable_ntp {
                        mon.save_actual_data();
                    }
                }
                _ => (),
            }
        }
    });
    if settings.ntp.enable && settings.ntp.server_list.len() > 0 {
        ntp.start().await;
    }

    let monitor_enable = settings.display.enable;
    let arc_03 = Arc::clone(&monitor);
    task::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await;
            if monitor_enable {
                let mut mon = arc_03.lock().await;
                mon.print_oled().await;
            }
        }
    });

    let rtc_enable = settings.rtc.enable;
    let rtc_cycle = settings.rtc.cycle / 1000;
    let arc_04 = Arc::clone(&monitor);
    let arc_server = Arc::clone(&server);
    task::spawn(async move {
        loop {
            sleep(Duration::from_secs(rtc_cycle as u64)).await;
            if rtc_enable {
                let mut mon = arc_04.lock().await;

                let ts = mon.get_actual_data().await;
                if let Ok(timestamp) = ts {
                    if timestamp != 0 {
                        arc_server
                            .lock()
                            .await
                            .update_state(NtpTimestamp { ts: timestamp })
                            .await;
                    }
                }
            }
        }
    });
    let web_port = env::var("WEB_SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap();
    let rocket_config = Config {
        address: "0.0.0.0".parse().unwrap(),
        port: web_port,

        ..Default::default()
    };
    let api: Arc<Mutex<dyn Iapi>> = Arc::new(Mutex::new(settings.clone()));
    tokio::select! {

        _ = get_rocket(rocket_config,Arc::clone(&api),Api::new(),drviver,Arc::new(Mutex::new(LoginSRC::new())),Arc::new(Mutex::new(NetworkSRC::new())),Arc::clone(&monitor)).await.launch()=>{},
    }

    froze_task().await;
}

async fn froze_task() {
    loop {
        sleep(Duration::from_secs(5)).await;
    }
}

use std::env;
use std::sync::Arc;

use super::events::EUdpEvents;
use super::events::Event;
use super::events::EventManager;
use super::NtpTimestamp;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use gpsd_proto::UnifiedResponse;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::net::TcpStream;
use tokio::stream;
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::sync::Mutex;
pub struct ConnectorGPS {
    host: String,
    port: u16,
    event_manager: Arc<Mutex<EventManager>>,
}

impl ConnectorGPS {
    pub fn new() -> Self {
        Self {
            host: env::var("GPS_HOST")
                .unwrap_or_else(|_| "localhost".to_string())
                .parse::<String>()
                .unwrap(),
            port: env::var("GPS_PORT")
                .unwrap_or_else(|_| "2947".to_string())
                .parse::<u16>()
                .unwrap(),
            event_manager: Arc::new(Mutex::new(EventManager::new())),
        }
    }

    pub async fn subscribe(&self) -> UnboundedReceiver<Event> {
        self.event_manager.lock().await.subscribe()
    }

    pub async fn start(&mut self) {
        let server_address = format!("{}:{}", self.host, self.port);
        let result: Result<TcpStream> = TcpStream::connect(server_address).await;
        if let Ok(stream) = result {
            let arc_stream = Arc::new(Mutex::new(stream));
            let event_manager = Arc::clone(&self.event_manager);
            tokio::spawn(async move {
                let mut stream_in = arc_stream.lock().await;
                let mut buffer = [0; 2048];
                let _ =stream_in.write(r#"?WATCH={"enable":true,"json":true,"nmea":false,"raw":0,"scaled":true,"timing":true}"#.as_bytes()).await;
                loop {
                    let bytes_read = stream_in.read(&mut buffer).await.unwrap();
                    let response = String::from_utf8_lossy(&buffer[0..bytes_read]);
                    match serde_json::from_str(&response) {
                        Ok(rd) => match rd {
                            UnifiedResponse::Version(v) => {
                                if v.proto_major < gpsd_proto::PROTO_MAJOR_MIN {
                                    error!("Gpsd major version mismatch");
                                }
                                info!("Gpsd version {} connected", v.rev);
                            }
                            UnifiedResponse::Devices(_) => (),
                            UnifiedResponse::Watch(_) => (),
                            UnifiedResponse::Device(d) => trace!("Device {:?}", d),
                            UnifiedResponse::Tpv(t) => {
                                let time = t.time;
                                if let Some(time) = time {
                                    let datetime = DateTime::parse_from_rfc3339(&time);
                                    if let Ok(datetime) = datetime {
                                        event_manager.lock().await.notify(Event {
                                            event_type: EUdpEvents::NewGPSTimestamp(
                                                NtpTimestamp::new(
                                                    datetime.timestamp().try_into().unwrap(),
                                                ),
                                            ),
                                        });
                                    }
                                }
                            }

                            UnifiedResponse::Sky(s) => {
                                event_manager.lock().await.notify(Event {
                                    event_type: EUdpEvents::NewGpsSky(
                                        s.satellites.len() as u16
                                    ),
                                });
                            trace!("Sky {:?}::{:?}", s.satellites.len(), s.satellites)},
                            UnifiedResponse::Pps(p) => trace!("PPS {:?}", p),
                            UnifiedResponse::Gst(g) => trace!("GST {:?}", g),
                        },
                        Err(e) => {
                            error!("Error decoding: {}", e);
                        }
                    }
                }
            });
        }
    }
}

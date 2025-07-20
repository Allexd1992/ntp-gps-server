use crate::ntp::timestamp::{self, Timestamp};
use chrono::{TimeZone, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use tokio::io::{AsyncReadExt, Result};
use tokio::{io::AsyncWriteExt, net::TcpStream};
#[derive(Debug, Serialize, Deserialize)]
pub struct OledPacket {
    pub(crate)  gps: String,
    pub(crate)  ntp: String,
    pub(crate)  time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CmdPacket {
    pub(crate) cmd: String,
    pub(crate) ts: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MonitorSender {
    pub last_ntp: Timestamp,
    pub last_gps: Timestamp,
    pub actial: Timestamp,
    pub satilite:u16
}

impl MonitorSender {
    pub async fn print_oled(&self) -> Result<()> {
        let datetime = Utc.timestamp(self.actial.ts as i64, 0);
        let banch = OledPacket {
            gps: format!(" {} sec ago", self.actial.ts - self.last_gps.ts),
            ntp: format!(" {} sec ago", self.actial.ts - self.last_ntp.ts),
            time: format!(" {}", datetime),
        };
        let host = env::var("DISPLAY_HOST")
            .unwrap_or_else(|_| "localhost".to_string())
            .parse::<String>()
            .unwrap();
        let port = env::var("DISPLAY_PORT")
            .unwrap_or_else(|_| "5050".to_string())
            .parse::<u16>()
            .unwrap();
        let enable = env::var("DISPLAY_ENABLE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap();
        if enable {
            let server_address = format!("{}:{}", host, port);

            // Устанавливаем соединение с сервером
            let mut stream = TcpStream::connect(server_address).await.unwrap();

            // Отправляем данные на сервер
            let payload = serde_json::to_string_pretty(&banch).unwrap();
            //println!("{:?}",payload);
            let res = stream.write_all(payload.as_bytes()).await;
            let mut buffer = [0; 1024];
            
            let n = stream.read(&mut buffer).await?;

            // Преобразуем ответ в строку и печатаем его
            let response = String::from_utf8_lossy(&buffer[0..n]);
            trace!("Received: {}", response);
        }
        //trace!("{:?}",res);

        Ok(())
    }

    pub async fn save_actual_data(&self) -> Result<()> {
        let host = env::var("RTC_HOST")
            .unwrap_or_else(|_| "localhost".to_string())
            .parse::<String>()
            .unwrap();
        let port = env::var("RTC_PORT")
            .unwrap_or_else(|_| "6060".to_string())
            .parse::<u16>()
            .unwrap();
        let enable = env::var("RTC_ENABLE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap();
        let server_address = format!("{}:{}", host, port);

        if enable {
            let banch = CmdPacket {
                cmd: String::from("set"),
                ts: format!("{:?}", self.actial.ts),
            };
            let mut stream = TcpStream::connect(server_address).await.unwrap();

            let payload = serde_json::to_string_pretty(&banch).unwrap();
            let res = stream.write_all(payload.as_bytes()).await;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;

            // Преобразуем ответ в строку и печатаем его
            let response = String::from_utf8_lossy(&buffer[0..n]);
            trace!("Received: {}", response);
        }

        Ok(())
    }

    pub async fn get_actual_data(&self) -> Result<u64> {
        let host = env::var("RTC_HOST")
            .unwrap_or_else(|_| "localhost".to_string())
            .parse::<String>()
            .unwrap();
        let port = env::var("RTC_PORT")
            .unwrap_or_else(|_| "6060".to_string())
            .parse::<u16>()
            .unwrap();
        let server_address = format!("{}:{}", host, port);

        let enable = env::var("RTC_ENABLE")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap();
        let server_address = format!("{}:{}", host, port);

        if enable {
            let banch = CmdPacket {
                cmd: String::from("get"),
                ts: format!("{:?}", self.actial.ts),
            };
            let mut stream = TcpStream::connect(server_address).await.unwrap();

            let payload = serde_json::to_string_pretty(&banch).unwrap();
            let res = stream.write_all(payload.as_bytes()).await;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;

            // Преобразуем ответ в строку и печатаем его
            let response = String::from_utf8_lossy(&buffer[0..n]);
            trace!("Received: {}", response);
            let obj: Value = serde_json::from_str(&response).unwrap();
            let ts = obj.get("timestamp").unwrap().as_u64().unwrap();

            Ok(ts)
        } else {
            Ok(0)
        }
    }
    pub  async fn get_json(&self)->Result<String>{


        Ok(serde_json::to_string_pretty(&self).unwrap())
    } 
}

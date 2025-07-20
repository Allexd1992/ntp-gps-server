use crate::http::interfaces::Iapi;

use super::interfaces::IStore;
use async_trait::async_trait;
use rocket::data::N;
use serde_derive::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::{fs, io::Result};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Settings {
    pub ntp: Ntp,
    pub gps: Gps,
    pub display: Display,
    pub rtc: RTC,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            ntp: Ntp {
                server_list: vec!["0.ru.pool.ntp.org:123".to_string()],
                enable: true,
                cycle: 5000,
            },
            gps: Gps { enable: true },
            display: Display { enable: true },
            rtc: RTC {
                enable: true,
                cycle: 10000,
            },
        }
    }
}

impl Iapi for Settings {
    fn get_settings(&mut self) -> Settings {
        self.clone()
    }

    fn get_ntp(&mut self) -> Ntp {
        self.ntp.clone()
    }

    fn get_gps(&mut self) -> Gps {
        self.gps.clone()
    }

    fn get_display(&self) -> Display {
        self.display.clone()
    }

    fn get_rtc(&self) -> RTC {
        self.rtc.clone()
    }

    fn set_settings(&mut self, settings: Settings) {
        self.display = settings.display.clone();
        self.ntp = settings.ntp.clone();
        self.gps = settings.gps.clone();
        self.rtc = settings.rtc.clone();
    }

    fn set_ntp(&mut self, ntp: Ntp) {
        self.ntp = ntp.clone();
    }

    fn set_gps(&mut self, gps: Gps) {
        self.gps = gps.clone()
    }

    fn set_display(&mut self, display: Display) {
        self.display = display.clone()
    }

    fn set_rtc(&mut self, rtc: RTC) {
        self.rtc = rtc.clone();
    }
}
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Ntp {
    pub server_list: Vec<String>,
    pub enable: bool,
    pub cycle: u32,
}
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Gps {
    pub enable: bool,
}
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Display {
    pub enable: bool,
}
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct RTC {
    pub enable: bool,
    pub cycle: u32,
}
pub struct Keeper {
    file: String,
    folder: String,
}

impl Keeper {
    pub fn new(file: String, folder: String) -> Self {
        Self { file, folder }
    }
}

#[async_trait]
impl IStore for Keeper {
    async fn Backup(&self, store: Settings) -> Result<()> {
        let exist = fs::try_exists(String::from(self.folder.clone()))
            .await
            .unwrap();
        if !exist {
            fs::create_dir(String::from(self.folder.clone()))
                .await
                .unwrap();
        }

        let path = format!("{}/{}.toml", self.folder, self.file);
        let data = toml::to_string(&store.clone()).unwrap();

        fs::write(path, data).await.unwrap();

        Ok(())
    }
    async fn Restore(&self) -> Result<Settings> {
        let path = format!("{}/{}.toml", self.folder, self.file);
        if file_exists(&path).await {
            let file = File::open(path).await.unwrap();
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).await.unwrap();
            println!("Reading file:{}", contents);
            let settings: Settings = toml::from_str(&contents).unwrap();
            Ok(settings)
        } else {
            let settings = Settings::new();
            self.Backup(settings.clone()).await;
            Ok(settings)
        }
    }
}

async fn file_exists(path: &str) -> bool {
    match fs::metadata(path).await {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

#[macro_use]
use rocket;
use std::{sync::{Arc}, env, collections::HashSet, io::Cursor};
use rocket::{Config, Rocket, Build, fs::FileServer,  fairing::{AdHoc, Info, Fairing, Kind}, data::ByteUnit, http::HeaderMap, Response};
use tokio::{sync::Mutex, fs::File, io::AsyncWriteExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use rocket::http::Method;

use rocket::http::Header;
use rocket::Request;
use crate::{settings::interfaces::IStore, services::{login::LoginSRC, network::{NetworkSRC, GetRequestPayload}}, ntp::request::MonitorSender};

use super::{swagger::ApiDoc, state::AppState, api::Api, interfaces::Iapi};




pub async fn get_rocket(config:Config, store:Arc<Mutex<dyn Iapi>>,api:Api, driver: Arc<Mutex<dyn IStore>>,login_detector: Arc<Mutex<LoginSRC>>, network: Arc<Mutex<NetworkSRC>>, monitor: Arc<Mutex<MonitorSender>>)->Rocket<Build>{
    write_config_js(Arc::clone(&network)).await;

    rocket::custom(config)
    
    .manage(AppState::new(Arc::clone(&store), driver,Arc::clone(&login_detector),Arc::clone(&network),Arc::clone(&monitor)))
    .mount(
        "/",
        SwaggerUi::new("/api/v1/swagger/<_..>").url("/api-docs/openapi.json", ApiDoc::openapi()),
    )
    .mount("/api/v1",api.list)
   
  .mount("/", FileServer::from("/root/public").rank(10))
  .attach(CORS)
}

    async fn write_config_js(network: Arc<Mutex<NetworkSRC>>) -> Result<(), Box<dyn std::error::Error>> {
        let  res =network.lock().await.get_network(GetRequestPayload{type_:String::from("GET")}).await.unwrap();
        let mut host=res.config.address.clone();
        host.pop();
        host.pop();
        host.pop();
        let content = format!(
            "window.REACT_APP_SERVER_HOST='{}';\nwindow.REACT_APP_SERVER_PORT={};",
            host, env::var("WEB_SERVER_PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap()
        );

        let mut file = File::create("/root/public/config.js").await?;
        file.write_all(content.as_bytes()).await?;
        Ok(())
    }

    pub struct CORS;

    #[rocket::async_trait]
    impl Fairing for CORS {
        fn info(&self) -> Info {
            Info {
                name: "Add CORS headers to responses",
                kind: Kind::Response
            }
        }
    
        async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
    }
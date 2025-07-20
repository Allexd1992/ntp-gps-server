use std::env;

use rocket::serde;
use ::serde::{Deserialize, Serialize};
use tokio::{io::{Result, AsyncWriteExt, AsyncReadExt}, net::TcpStream};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct GetRequestPayload {
    #[serde(rename(serialize = "type", deserialize = "type"))]
   pub type_:String
} 


#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct GetResponsePayload {

   pub status:String,
   pub config:Config
} 
#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct SetRequestPayload {
   #[serde(rename(serialize = "type", deserialize = "type"))]
   pub type_:String,
   pub config:Config
} 


#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct SetResponsePayload {

   pub status:String,
   pub message:String
} 

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Config {
    pub address:String,
    pub gateway:String,
    pub dns:String
}

pub struct NetworkSRC{
host:String,
port:u16
}
impl NetworkSRC{
    pub fn new()->Self{
        Self{host:env::var("NETWORK_HOST")
        .unwrap_or_else(|_| "localhost".to_string())
        .parse::<String>()
        .unwrap(),
          port:env::var("NETWORK_PORT")
          .unwrap_or_else(|_| "7575".to_string())
          .parse::<u16>()
          .unwrap()}
    }

pub async fn get_network(&self,request:GetRequestPayload)->Result<GetResponsePayload>{
  
let server_address = format!("{}:{}", self.host.clone(), self.port);
let mut stream = TcpStream::connect(server_address).await.unwrap();

let payload = serde_json::to_string_pretty(&request).unwrap();
trace!("{:?}",payload);
let res = stream.write_all(payload.as_bytes()).await;
let mut buffer = [0; 1024];
let n = stream.read(&mut buffer).await?;

let response = String::from_utf8_lossy(&buffer[0..n]);
trace!("Received: {}", response);
let result= serde_json::from_str(&response).unwrap();
    Ok(result)
}

pub async fn set_network(&self,request:SetRequestPayload)->Result<SetResponsePayload>{
  
    let server_address = format!("{}:{}", self.host.clone(), self.port);
    let mut stream = TcpStream::connect(server_address).await.unwrap();
    
    let payload = serde_json::to_string_pretty(&request).unwrap();
    let res = stream.write_all(payload.as_bytes()).await;
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    
    let response = String::from_utf8_lossy(&buffer[0..n]);
    trace!("Received: {}", response);
    let result= serde_json::from_str(&response).unwrap();
        Ok(result)
    }
}
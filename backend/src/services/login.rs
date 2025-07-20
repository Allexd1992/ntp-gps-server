use std::env;
use tokio::{io::{Result, AsyncWriteExt, AsyncReadExt}, net::TcpStream};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct RequestPayload {
pub login:String,
pub password:String

}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct ResponsePayload{
    pub result:String,
    pub message:String
}

pub struct LoginSRC{

}
impl LoginSRC{
    pub fn new()->Self{
        Self{}
    }

pub async fn login_detect(&self,request:RequestPayload)->Result<ResponsePayload>{
    let host = env::var("LOGIN_HOST")
    .unwrap_or_else(|_| "localhost".to_string())
    .parse::<String>()
    .unwrap();
let port = env::var("LOGIN_PORT")
    .unwrap_or_else(|_| "7070".to_string())
    .parse::<u16>()
    .unwrap();
let server_address = format!("{}:{}", host, port);
let mut stream = TcpStream::connect(server_address).await.unwrap();

let payload = serde_json::to_string_pretty(&request).unwrap();
let res = stream.write_all(payload.as_bytes()).await;
let mut buffer = [0; 1024];
let n = stream.read(&mut buffer).await?;

let response = String::from_utf8_lossy(&buffer[0..n]);
trace!("Received: {}", response);
let result= serde_json::from_str(&response).unwrap();
    Ok(result)
}}
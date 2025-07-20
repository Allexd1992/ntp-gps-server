
use std::sync::Arc;


use rocket::data::ByteUnit;
use rocket::http::Status;
use rocket::{routes, Route, State, get, post, Data};

use crate::diagnostic::types::DiagnosticPacket;
use crate::http::state::{AppState, self};
use crate::services::login::{RequestPayload, ResponsePayload};
use crate::services::network::{GetRequestPayload, SetRequestPayload, Config};
use crate::settings::store::{Ntp, Display, RTC, Settings, Gps};



pub struct  Api{
    pub list:Vec<Route>
    }
    
    impl Api{
        pub fn new()->Self{
            let list = routes![
                get_settings,
                get_display,
                get_gps,
                get_ntp,
                get_rtc,
                set_display,
                set_rtc,
                set_ntp,
                set_gps,
                set_settings,
                login,
                get_network,
                set_network,
                get_monitor,
                get_sys_info

                ];
            Self{list}
        }
    }
    
    

/// Get Store
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current store state", body = Settings)
    )
    ,
    params(),
)]
#[get("/settings")]
pub async fn get_settings(state: &State<AppState>) -> Result<String, Status> {
    let store = state.store.lock().await.get_settings();
    Ok(serde_json::to_string_pretty(&store).unwrap())
} 
/// Get NTP
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current template state", body = Ntp)
    )
    ,
    params(
),
)]
#[get("/ntp")]
pub async fn get_ntp(state: &State<AppState>) -> Result<String, Status> {
    let ntp = state.store.lock().await.get_ntp();
    Ok(serde_json::to_string_pretty(&ntp).unwrap())
}
/// Get GPS settings
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current resorce state", body = Gps)
    )
    ,
    params(
),
)]
#[get("/gps")]
pub async fn get_gps(state: &State<AppState>) -> Result<String, Status> {
    let gps = state.store.lock().await.get_gps();
    Ok(serde_json::to_string_pretty(&gps).unwrap())
}

/// Get Display settings
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current tasks state", body = Display)
    )
    ,
    params(
),
)]
#[get("/display")]
pub async fn get_display(state: &State<AppState>) -> Result<String, Status> {
    let display = state.store.lock().await.get_display();
    Ok(serde_json::to_string_pretty(&display).unwrap())
}

/// Get  RTC settings
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current Template layer", body = RTC)
    )
    ,
    params(
        
),)]
#[get("/rtc")]
pub async fn get_rtc(state: &State<AppState>) -> Result<String, Status> {
    let rtc = state.store.lock().await.get_rtc();
    Ok(serde_json::to_string_pretty(&rtc).unwrap())
}




/// Update settings
#[utoipa::path(
    context_path = "/api/v1",
    request_body = Settings,
    responses(
        (status = 200, description = "Adding is Success")
    )
    ,

    params(
        ),
)]
#[post("/settings", data="<values>")]
pub async fn set_settings(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Settings = serde_json::from_str(payload_str).unwrap();
    state.store.lock().await.set_settings(values.clone());
    let mut store= state.store.lock().await;
    state.driver.lock().await.Backup(store.get_settings()).await;
    Ok(serde_json::to_string_pretty(&values).unwrap())
}
/// Update gps
#[utoipa::path(
    context_path = "/api/v1",
    request_body = Gps,
    responses(
        (status = 200, description = "Update is Success")
    )
    ,

    params(
        ),
)]
#[post("/gps", data="<values>")]
pub async fn set_gps(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Gps = serde_json::from_str(payload_str).unwrap();
    state.store.lock().await.set_gps(values.clone());
    let mut store= state.store.lock().await.get_settings();
    state.driver.lock().await.Backup(store.clone()).await;
    Ok(serde_json::to_string_pretty(&values).unwrap())
}


/// update NTP
#[utoipa::path(
    context_path = "/api/v1",
    request_body = Ntp,
    responses(
        (status = 200, description = "Adding is Success")
    )
    ,

    params(
        ),
)]
#[post("/ntp", data="<values>")]
pub async fn set_ntp(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Ntp = serde_json::from_str(payload_str).unwrap();
    state.store.lock().await.set_ntp(values.clone());
    let store= state.store.lock().await.get_settings();
    state.driver.lock().await.Backup(store.clone()).await;
    Ok(serde_json::to_string_pretty(&values).unwrap())
}
/// Update Display
#[utoipa::path(
    context_path = "/api/v1",
    request_body = Display,
    responses(
        (status = 200, description = "Update is Success")
    )
    ,

    params(
        ),
)]
#[post("/display", data="<values>")]
pub async fn set_display(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Display = serde_json::from_str(payload_str).unwrap();
    state.store.lock().await.set_display(values.clone());
    let store= state.store.lock().await.get_settings();
    state.driver.lock().await.Backup(store.clone()).await;
    Ok(serde_json::to_string_pretty(&values).unwrap())
}
/// Update RTC 
#[utoipa::path(
    context_path = "/api/v1",
    request_body = RTC,
    responses(
        (status = 200, description = "Update is Success")
    )
    ,

    params(
        ),
)]
#[post("/rtc", data="<values>")]
pub async fn set_rtc(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Display = serde_json::from_str(payload_str).unwrap();
    state.store.lock().await.set_display(values.clone());
    let store= state.store.lock().await.get_settings();
    state.driver.lock().await.Backup(store.clone()).await;
    Ok(serde_json::to_string_pretty(&values).unwrap())
}
/// Login and password valid
#[utoipa::path(
    context_path = "/api/v1",
    request_body = RequestPayload,
    responses(
        (status = 200, description = "Update is Success")
    )
    ,

    params(
        ),
)]
#[post("/login", data="<values>")]
pub async fn login(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: RequestPayload = serde_json::from_str(payload_str).unwrap();
    let result =state.login_detector.lock().await.login_detect(values).await;
if let Ok(payload)=result{
Ok(serde_json::to_string_pretty(&payload).unwrap())
}
else{
    Ok(serde_json::to_string_pretty(&ResponsePayload { result: String::from("error"), message:String::from("Service is not found")}).unwrap())
}
}


/// Get  network settings
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current Template layer", body = GetRequestPayload)
    )
    ,
    params(
        
),)]
#[get("/network")]
pub async fn get_network(state: &State<AppState>) -> Result<String, Status> {
    let result = state.network.lock().await.get_network(GetRequestPayload{type_:String::from("GET")}).await;
    if let Ok(payload)=result{
        Ok(serde_json::to_string_pretty(&payload).unwrap())
    }
    else{
        Ok(String::from(r#"{"result":"error","message":"Network service is not found"}"#))
    }
   
}

/// Set  network settings

#[utoipa::path(
    context_path = "/api/v1",
    request_body = Config,
    responses(
        (status = 200, description = "Update is Success")
    )
    ,

    params(
        ),
)]
#[post("/network", data="<values>")]
pub async fn set_network(values: Data<'_>,state: &State<AppState>) -> Result<String, Status> {
    let payload = values.open(ByteUnit::MB).into_string().await.unwrap();
    let payload_str = payload.as_str();
    let values: Config = serde_json::from_str(payload_str).unwrap();
let result = state.network.lock().await.set_network(SetRequestPayload{type_:String::from("SET"),config:values}).await;
    if let Ok(payload)=result{
        Ok(serde_json::to_string_pretty(&payload).unwrap())
    }
    else{
        Ok(String::from(r#"{"result":"error","message":"Network service is not found"}"#))
    }
   
}
#[utoipa::path(
    context_path = "/api/v1",
    responses(
        (status = 200, description = "Current Template layer", body = GetRequestPayload)
    )
    ,
    params(
        
),)]
#[get("/status")]
pub async fn get_monitor(state: &State<AppState>) -> Result<String, Status> {
    Ok(state.monitor.lock().await.get_json().await.unwrap())
    }

/// Get system info
    #[utoipa::path(
        context_path = "/api/v1",
        responses(
            (status = 200, description = "Current Template layer", body = DiagnosticPacket)
        )
        ,
        params(
            
    ),)]
    #[get("/system")]
    pub async fn get_sys_info(state: &State<AppState>) -> Result<String, Status> {

let mut diag =state.info.lock().await;
        diag.refresh();
        let packet = DiagnosticPacket {
            UsageRam: diag.UsageRam,
            TotalRam: diag.TotalRam,
            UsageCPU: diag.UsageCPU,
            UplinkSpeed: diag.UplinkSpeed,
            UplinkData: diag.UplinkData,
            DownlinkSpeed: diag.DownlinkSpeed,
            DownlinkData: diag.DownlinkData,
            Uptime: diag.Uptime,
        };
        Ok(serde_json::to_string_pretty(&packet).unwrap())
        }
    
   


use crate::{http::api, settings::store::{Display, RTC, Gps, Ntp, Settings}, services::{login::RequestPayload as LoginRequestPayload, network::{GetResponsePayload, GetRequestPayload, Config}}, diagnostic::types::DiagnosticPacket};
use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(
    paths(
     api::get_settings,
     api::get_ntp,
     api::get_gps,
     api::get_display,
     api::get_rtc,
     api::set_settings,
     api::set_ntp,
     api::set_gps,
     api::set_display,
     api::set_rtc,
     api::login,
     api::get_network,
     api::set_network,
     api::get_monitor,
     api::get_sys_info


    ),
    components(
        schemas(Settings,Ntp,Gps,RTC,Display, LoginRequestPayload,GetResponsePayload,Config,DiagnosticPacket),
    ),
    tags(
        (name = "NTP Service Web API", description = "Integration API")
))]
pub struct ApiDoc;

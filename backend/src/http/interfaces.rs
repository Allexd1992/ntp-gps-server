

use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::settings::{store::{Settings, Ntp, Gps, RTC, Display}, self};



pub trait Iapi: Sync + Send {

fn get_settings(&mut self)->Settings;
fn get_ntp(&mut self)->Ntp;
fn get_gps(&mut self)->Gps;
fn get_display(&self)->Display;
fn get_rtc(&self)->RTC;
fn set_settings(&mut self, settings:Settings);
fn set_ntp(&mut self, ntp:Ntp);
fn set_gps(&mut self, gps:Gps);
fn set_display(&mut self, display:Display);
fn set_rtc(&mut self, rtc:RTC);
}


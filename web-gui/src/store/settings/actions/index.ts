import { action } from 'typesafe-actions';
import { Constants } from '../constans';
import * as types from '../types';
import * as api from '../../../api/rest';
import store from '../..';



export const setSettings = (data: types.ISettings) => action(Constants.SET_SETTINGS, { data: data });
export const getSettings = () => action(Constants.GET_SETTINGS, {});
export const displatyEnable = () => action(Constants.DISPLAY_ENABLE, {});
export const displayDisable = () => action(Constants.DISPLAY_DISABLE, {});
export const gpsEnable = () => action(Constants.GPS_ENABLE, {});
export const gpsDisable = () => action(Constants.GPS_DISABLE, {});
export const ntpEnable = () => action(Constants.NTP_ENABLE, {});
export const ntpDisable = () => action(Constants.NTP_DISABLE, {});
export const rtcEnable = () => action(Constants.RTC_ENABLE, {});
export const rtcDisable = () => action(Constants.RTC_DISABLE, {});
export const setNtpCycle = (data: number) => action(Constants.NTP_CYCLE, { data });
export const setRtcCycle = (data: number) => action(Constants.RTC_CYCLE, { data });
export const addNtpAddr = (data: string) => action(Constants.NTP_ADD_ADRESS_ITEM, { data });
export const removeNtpAddr = (data: string) => action(Constants.NTP_REMOVE_ADRESS_ITEM, { data });




export const initSettings = () => {
    api.getSettings((settings: types.ISettings) => { 
        console.log("Test",settings); 
    store.dispatch(action(Constants.SET_SETTINGS, { data:settings}));
     })

};

export const saveSettings = () => {
    api.setSettings(() => { 
        console.log("Save setttings:",store.getState().settings); 

     },store.getState().settings)

};
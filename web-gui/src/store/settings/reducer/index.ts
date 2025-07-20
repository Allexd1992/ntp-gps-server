import { Constants } from "../constans";
import { ISettings, SettingsActions } from "../types";




const init: ISettings =
{
    display: { enable: false },
    gps: { enable: false },
    ntp: {
        cycle: 0,
        enable: false,
        server_list: []
    },
    rtc: {
        cycle: 0,
        enable: false
    }
};


export function settingsReducer(state: ISettings = init,
    action: SettingsActions): ISettings {
    switch (action.type) {
        case Constants.GET_SETTINGS:
            return state;
        case Constants.SET_SETTINGS:
            if ('data' in action.payload) {
                return {
                    ...state,
                    display: { ...action.payload.data.display },
                    gps: { ...action.payload.data.gps },
                    ntp: { ...action.payload.data.ntp },
                    rtc: { ...action.payload.data.rtc },
                };
            } else {
                return state;
            }
        case Constants.DISPLAY_DISABLE: {
            return {
                ...state,
                display: { enable: false }
            }
        }
        case Constants.DISPLAY_ENABLE: {
            if (state.display.enable){
                return {
                    ...state,
                    display: { ...state.display, enable: false }
                }
               
            }
            else{
                return {
                    ...state,
                    display: { ...state.display, enable: true }
                }
            }
        }
        case Constants.GPS_DISABLE: {
            return {
                ...state,
                gps: { enable: false }
            }
        }
        case Constants.GPS_ENABLE: {
            if (state.gps.enable){
                return {
                    ...state,
                    gps: { ...state.gps, enable: false }
                }
               
            }
            else{
                return {
                    ...state,
                    gps: { ...state.gps, enable: true }
                }
            }
        }
        case Constants.NTP_DISABLE: {
            return {
                ...state,
                ntp: { ...state.ntp, enable: false }
            }
        }
        case Constants.NTP_ENABLE: {
            if (state.ntp.enable){
                return {
                    ...state,
                    ntp: { ...state.ntp, enable: false }
                }
               
            }
            else{
                return {
                    ...state,
                    ntp: { ...state.ntp, enable: true }
                }
            }
        }
        case Constants.NTP_CYCLE: {
            if ('data' in action.payload) {
                return {
                    ...state,
                    ntp: { ...state.ntp, cycle: action.payload.data }
                }
            } else return state;
        }
        case Constants.NTP_ADD_ADRESS_ITEM: {
            if ('data' in action.payload) {
                let list = state.ntp.server_list;
                if (!list.includes(action.payload.data)) {
                }
                return {
                    ...state,
                    ntp: { ...state.ntp, server_list: [...state.ntp.server_list, action.payload.data] }
                }
            } else return state;
        }
        case Constants.NTP_REMOVE_ADRESS_ITEM: {
            if ('data' in action.payload) {
                let list = state.ntp.server_list;
                if (list.includes(action.payload.data)) {
                    const updatedServerList = state.ntp.server_list.filter(
                        server => server !== action.payload.data
                      );
            
                return {
                    ...state,
                    ntp: { ...state.ntp, server_list:updatedServerList }
                }
            } else return state;
        }  else return state;}

        case Constants.RTC_DISABLE: {
            return {
                ...state,
                rtc: { ...state.rtc, enable: false }
            }
        }
        case Constants.RTC_ENABLE: {
            if (state.rtc.enable){
                return {
                    ...state,
                    rtc: { ...state.rtc, enable: false }
                }
               
            }
            else{
                return {
                    ...state,
                    rtc: { ...state.rtc, enable: true }
                }
            }
           
        }
        case Constants.RTC_CYCLE: {
            if ('data' in action.payload) {
                return {
                    ...state,
                    rtc: { ...state.rtc, cycle: action.payload.data }
                }
            } else return state;
        }


        default:
            return state;
    }
}





import { ActionType, Types } from 'typesafe-actions';
import * as actions from '../actions';

export interface IEnable{
    enable: boolean,
}
  
  
export interface INtp{
    cycle: number,
    enable: boolean,
    server_list: string[]
}

export interface IPublish{
    urlTemplate: string,
}

export interface IRtc{
    cycle: number,
    enable: boolean
}


export interface ISettings{
    display: IEnable,
    gps:IEnable,
    ntp:INtp,
    rtc:IRtc
}


export type SettingsActions = ActionType<typeof actions>;
import { ActionType, Types } from 'typesafe-actions';
import * as actions from '../actions';

export interface IMonitoring{
    last_ntp:number,
    last_gps:number,
    actial:number,
    satilite:number
 
}

export type userActions = ActionType<typeof actions>;
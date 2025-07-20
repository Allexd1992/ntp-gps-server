
import { ActionType, Types } from 'typesafe-actions';
import * as actions from '../actions';

export interface IInfo{
  UsageRam: number,
  UsageCPU: number,
  UplinkSpeed: number,
  DownlinkSpeed: number,
  UplinkData: number,
  DownlinkData: number,
  Uptime: number,
  TotalRam:number 
}

export type infoActions = ActionType<typeof actions>;

import { ActionType, Types } from 'typesafe-actions';
import * as actions from '../actions';

export interface INetworck{
    address: string,
    gateway: string,
    dns: string
}
  


export type networkActions = ActionType<typeof actions>;
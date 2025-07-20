import { ActionType, Types } from 'typesafe-actions';
import * as actions from '../actions';

export interface IUser{
 login: string,
 password:string ,
 loginStatus:boolean,
 
}

export type userActions = ActionType<typeof actions>;
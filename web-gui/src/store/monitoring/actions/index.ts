import { action } from 'typesafe-actions';
import { Constants } from '../constants';
import * as api from '../../../api/rest';
import store from '../..';
import { IMonitoring } from '../types';



export const getMonitoring = () => action(Constants.GET_MONITORING, {});
export const setMonitoring = (data: IMonitoring) => action(Constants.SET_MONITORING, { data });




export const initMonitoring = () => {
    api.getMonitoring((monitoring: IMonitoring) => {
        store.dispatch(action(Constants.SET_MONITORING, { data: monitoring }))
    });
}



import { action } from 'typesafe-actions';
import { Constants } from '../constans';


import store from '../..';
import * as types from '../types';
import * as api from '../../../api/rest';


export const initInfo= ()=>{api.getSystem((infoData:types.IInfo)=>{store.dispatch(action(Constants.SET_INFO, { infoData }))});}

export const setInfo = (eventItem: types.IInfo) => action(Constants.SET_INFO, { infoData:eventItem });
export const getInfo = () => action(Constants.GET_INFO, {});

/*
export const saveSettings = (settings :types.ISettings) => {
    api.setSettings((ISettings: types.ISettings) => { console.log(ISettings); store.dispatch(action(Constants.SET_SETTINGS, { settings: ISettings })); },settings);
    return action(Constants.GET_SETTINGS, {});
};

export const loadSettings = () => {
    api.getSettings((ISettings: types.ISettings) => { console.log(ISettings); store.dispatch(action(Constants.SET_SETTINGS, { settings: ISettings })); })
    return action(Constants.GET_SETTINGS, {})
};

export const runScan = () => {
    api.runScan((ISettings: types.ISettings) => { console.log('Scanner is runed')})
    return action(Constants.RUN_SCAN, {})
};
export const stopScan = () => {
    api.stopScan((ISettings: types.ISettings) => { console.log('Scanner is stoped')})
    return action(Constants.STOP_SCAN, {})
};
*/
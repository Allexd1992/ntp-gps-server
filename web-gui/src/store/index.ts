import { configureStore } from '@reduxjs/toolkit';
import logger from 'redux-logger';
import { ISettings } from './settings/types';
import { settingsReducer } from './settings/reducer';
import { IUser } from './login/types';
import { loginReducer } from './login/reducer';
import { INetworck } from './network/types';
import { networkReducer } from './network/reducer';
import { IMonitoring } from './monitoring/types';
import { monitoringReducer } from './monitoring/reducer';
import { infoReducer } from './info/reducer';
import { IInfo } from './info/types';

export interface IRootState {
    settings: ISettings,
    login:IUser,
    network:INetworck,
    monitoring:IMonitoring
    info:IInfo
}

 const store = configureStore<IRootState, any, any, any>({
    reducer:{
    settings:settingsReducer,
    login:loginReducer,
    network:networkReducer,
    monitoring:monitoringReducer,
    info:infoReducer
    },
    middleware:(getDefaultMiddleware:any) => getDefaultMiddleware().concat(logger)

});

export default store;

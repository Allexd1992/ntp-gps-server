import { userActions, IMonitoring } from '../types';
import { Constants } from '../constants';

const init: IMonitoring =
{
    last_ntp: 0,
    last_gps: 0,
    actial: 0,
    satilite: 0
};


export function monitoringReducer(state: IMonitoring = init,
    action: userActions): IMonitoring {
    switch (action.type) {
       case  Constants.SET_MONITORING:
        return {...action.payload.data};
        case  Constants.GET_MONITORING:
            return state;
        
        default:
            return state;
    }
}
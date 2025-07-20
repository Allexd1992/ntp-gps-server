import { networkActions, INetworck } from '../types';
import { Constants } from '../constants';

const init: INetworck =
{
    address: "10.15.5.144/24",
    gateway: "10.15.5.1",
    dns: "10.15.5.1"
  };


export function networkReducer(state: INetworck = init,
    action: networkActions): INetworck {
    switch (action.type) {
       case  Constants.GET_NETWORK:
        return state;
        case  Constants.SET_ADDRES:
            return {...state, address:action.payload.data};
        case  Constants.SET_GATEWAY:

                    return {...state,gateway:action.payload.data}
            
        case Constants.SET_DNS:
            return {...state,dns:action.payload.data}
            case Constants.SET_NETWORK:
                return {...action.payload.data}
      
        default:
            return state;
    }
}
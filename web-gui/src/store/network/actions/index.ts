import { action } from 'typesafe-actions';
import { Constants } from '../constants';
import * as api from '../../../api/rest';
import store from '../..';
import { INetworck } from '../types';



export const getNetwok = () => action(Constants.GET_NETWORK, {  });
export const setNetwok = (network:INetworck) => action(Constants.SET_NETWORK, { data:network });
export const setAddres = (adr:string) => action(Constants.SET_ADDRES, {data:adr  });
export const setGateway = (adr:string) => action(Constants.SET_GATEWAY, {data:adr  });
export const setDns = (adr:string) => action(Constants.SET_DNS, {data:adr  });




export const initNetwork = () => {
    api.getNetwork((network:INetworck) => { 
store.dispatch(action(Constants.SET_NETWORK,{data:network}))}
        );
     }


     export const saveNetwork = () => {
        api.setNetwork(() => { 
    console.log("Save  networck")
},store.getState().network);
         }

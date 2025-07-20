import { infoActions, IInfo } from '../types';
import { Constants } from '../constans';


  


const init: IInfo =
{
    UsageRam: 0,
    UsageCPU: 0,
    UplinkSpeed: 0,
    DownlinkSpeed: 0,
    UplinkData: 0,
    DownlinkData: 0,
    Uptime: 0,
    TotalRam:0
    
};


  

  
  
  
  
  
  
  
  
  



export function infoReducer(state: IInfo = init,
    action: infoActions): IInfo {
    switch (action.type) {
       case  Constants.GET_INFO:
        return state;
        case  Constants.SET_INFO:
            return {...action.payload.infoData};

        
      
        default:
            return state;
    }
}
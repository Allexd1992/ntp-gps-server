import { userActions, IUser } from '../types';
import { Constants } from '../constant';

const init: IUser =
{
   login:'',
   password:'',
   loginStatus:false

};


export function loginReducer(state: IUser = init,
    action: userActions): IUser {
    switch (action.type) {
       case  Constants.SET_LOGIN:
        return {...state,login:action.payload.login};
        case  Constants.SET_PASS:
            return {...state,password:action.payload.pass};
            case  Constants.LOGIN:

                    return {...state,loginStatus:true}
            
        case Constants.LOGOUT:
            return {login:'',password:'',loginStatus:false} 
      
        default:
            return state;
    }
}
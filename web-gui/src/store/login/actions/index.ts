import { action } from 'typesafe-actions';
import { Constants } from '../constant';
import * as api from '../../../api/rest';
import store from '../..';



export const setLogin = (login: string) => action(Constants.SET_LOGIN, { login });
export const setPassword = (pass:string) => action(Constants.SET_PASS, {pass});
export const login = () => action(Constants.LOGIN, {});




export const logout = () => action(Constants.LOGOUT, {});



export const login2 = () => {
    api.login(() => { 
store.dispatch(action(Constants.LOGIN,{}))},store.getState().login
        );
     }


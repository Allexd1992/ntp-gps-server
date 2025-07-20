
import { error } from 'console';
import * as types from '../../store/settings/types';
import * as user_types from '../../store/login/types';
import { IUser } from '../../store/login/types';
import { INetworck } from '../../store/network/types';
import { IMonitoring } from '../../store/monitoring/types';
import { IInfo } from '../../store/info/types';

const uri = `http://${window.REACT_APP_SERVER_HOST}:${window.REACT_APP_SERVER_PORT}`
export type callbackType = (settings: types.ISettings) => void;
export type callbackGetNtw = (network:INetworck) => void;
export type callbackMonitoring = (monitoring:IMonitoring) => void;
export type callbackSystemInfo= (monitoring:IInfo) => void;
export type callbackSet = () => void;




const api = {

    GET: {
        settings: {
            method: "GET",
            url: "/api/v1/settings",
            payload: { null: {} }
        },
        network: {
            method: "GET",
            url: "/api/v1/network",
            payload: { null: {} }
        },
        monitoring: {
            method: "GET",
            url: "/api/v1/status",
            payload: { null: {} }
        },
        info: {
            method: "GET",
            url: "/api/v1/system",
            payload: { null: {} }
        },

    },
    POST: {
        settings: {
            method: "POST",
            url: "/api/v1/settings",
            payload: { config: null }
        },
        login: {
            method: "POST",
            url: "/api/v1/login",
            payload: {
                credentials: {
                    login: "root",
                    password: "orangepi"
                }
            }
        },
        network: {
            method: "POST",
            url: "/api/v1/network",
            payload: {

                address: "10.15.5.144/24",
                dns: "10.15.5.1",
                gateway: "10.15.5.1"

            }
        },

    }


}


export const getSettings = async (callbackIn: callbackType): Promise<void> => {
    fetch(uri + api.GET.settings.url, {
        mode: "cors",
    })
        .then(response => response.json())
        .then(data => {
            if (data) {
                console.log("Request GET settings:", data)
                if (callbackIn) callbackIn(data as types.ISettings);

            }
        })
        .catch(error => console.error(error));
}

export const getNetwork = async (callbackIn: callbackGetNtw): Promise<void> => {
    fetch(uri + api.GET.network.url, {
        mode: "cors",
    })
        .then(response => response.json())
        .then(data => {
            if (data) {
                console.log("Request GET networck:", data.config)
                if (callbackIn) callbackIn(data.config as INetworck);

            }
        })
        .catch(error => console.error(error));
}

export const getMonitoring = async (callbackIn: callbackMonitoring): Promise<void> => {
    fetch(uri + api.GET.monitoring.url, {
        mode: "cors",
    })
        .then(response => response.json())
        .then(data => {
            if (data) {
                console.log("Request GET Monitoring:", data)
                let payload:IMonitoring ={
                    last_ntp: data.last_ntp.ts,
                    last_gps: data.last_gps.ts,
                    actial: data.actial.ts,
                    satilite: data.satilite
                }
                if (callbackIn) callbackIn(payload);

            }
        })
        .catch(error => console.error(error));
}

export const setSettings = async (callbackIn: callbackSet, settings:types.ISettings): Promise<void> => {
    console.log(settings);
    const payload = {...settings};
    fetch(uri + api.POST.settings.url, {

        method: api.POST.settings.method,
        body: JSON.stringify(payload),
    })
        .then(response => {
            console.log(response)
            return response.json()
        })
        .then(data => {
            if (data.status == "success") {
                console.log("Request POST settings:", data)
                if (callbackIn)callbackIn();

            }
            else {
                console.error(data);
            }
        })
        .catch(error => console.error(error));
}

export const setNetwork = async (callbackIn: callbackSet, network:INetworck): Promise<void> => {
    console.log(network);
    const payload = {...network};
    fetch(uri + api.POST.network.url, {

        method: api.POST.network.method,
        body: JSON.stringify(payload),
    })
        .then(response => {
            console.log(response)
            return response.json()
        })
        .then(data => {
            if (data.status == "success") {
                console.log("Request POST settings:", data)
                if (callbackIn) callbackIn();

            }
            else {
                console.error(data);
            }
        })
        .catch(error => console.error(error));
}


export const login = async (callbackIn: callbackSet, user: IUser): Promise<void> => {
    console.log(user);
    const payload = api.POST.login.payload.credentials;
    payload.login = user.login;
    payload.password = user.password;
    fetch(uri + api.POST.login.url, {

        method: api.POST.login.method,
        body: JSON.stringify(payload),
    })
        .then(response => {
            console.log(response)
            return response.json()
        })
        .then(data => {
            if (data.result == "success") {
                console.log("Request POST settings:", data)
                if (callbackIn) callbackIn();

            }
            else {
                console.error(data);
            }
        })
        .catch(error => console.error(error));


        
}

export const getSystem = async (callbackIn: callbackSystemInfo): Promise<void> => {
    fetch(uri + api.GET.info.url, {
        mode: "cors",
    })
        .then(response => response.json())
        .then(data => {
            if (data) {
                console.log("Request GET System Info:", data)
         
                if (callbackIn) callbackIn(data);

            }
        })
        .catch(error => console.error(error));
}
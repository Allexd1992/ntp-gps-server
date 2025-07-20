import * as React from 'react';
import './App.css';
import { useSelector } from 'react-redux';
import { IRootState } from './store';
import { IUser } from './store/login/types';
import LoginPanel from './components/login-panel/loginPanel';
import MainPanel from './components/main-window/mainWindow';
import * as actions from './store/settings/actions';
import * as actionsNtw from './store/network/actions';
import * as actionsMonitoring from './store/monitoring/actions';
import * as actionsinfo from './store/info/actions';
function App() {
  const login:IUser = useSelector((state:IRootState)=>state.login);
  React.useEffect(() => { setTimeout(actions.initSettings, 1000) }, []);
  React.useEffect(() => { setTimeout(actionsNtw.initNetwork, 1000) }, []);
  React.useEffect(() => { setTimeout(()=>setInterval(()=>{actionsinfo.initInfo(); },5000), 2000) }, []);
  React.useEffect(() => { setTimeout(()=>setInterval(()=>{actionsMonitoring.initMonitoring();actionsinfo.initInfo(); },5000), 1000) }, []);
  return (
    <div className="App">
    { !login.loginStatus?<LoginPanel/>:<div><MainPanel/></div>}
  </div>
  );
}

export default App;

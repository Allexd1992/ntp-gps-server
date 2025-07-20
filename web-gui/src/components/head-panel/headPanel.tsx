import React, { FC } from 'react';
import { AppBar, Box, Toolbar, Typography, Button } from '@mui/material';
import BluetoothIcon from '@mui/icons-material/';
import IconButton from '@mui/material/IconButton';
import { shallowEqual, useDispatch, useSelector } from 'react-redux';
import * as action from '../../store/login/actions';
import { IMonitoring } from '../../store/monitoring/types';
type props = {};
const HeadPanel: FC<props> = ({}) => {
  let monitoring: IMonitoring = useSelector((state: any) => state.monitoring, shallowEqual);
  const dispatch = useDispatch();
    return (
        <AppBar position="sticky"   sx={{background:"#05101efe" , opacity:'0.95'}}>
        <Toolbar sx={{ margin:"0px" , padding:"0px"}}>
    
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Актуальное время NTP сервера:
          </Typography>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            {new Date(monitoring.actial * 1000).toUTCString()}
          </Typography>
          <Button onClick={()=>{dispatch(action.logout())}} sx={{color: "#E4E4E4"}}>Выход</Button>
        </Toolbar>
      </AppBar>
        
        

    );
}
export default HeadPanel;


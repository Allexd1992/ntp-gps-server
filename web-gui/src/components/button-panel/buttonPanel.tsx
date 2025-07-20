import React, { FC } from 'react';
import { useSelector, useDispatch } from 'react-redux';
import {Box, SpeedDial, SpeedDialIcon,SpeedDialAction  } from '@mui/material';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import StopIcon from '@mui/icons-material/Stop';
import MenuIcon from '@mui/icons-material/Menu';
import CloseIcon from '@mui/icons-material/Close';
import FileUploadIcon from '@mui/icons-material/FileUpload';
import FileDownloadIcon from '@mui/icons-material/FileDownload';
import { ISettings } from '../../store/settings/types';
import * as actions from '../../store/settings/actions';
import * as networkActions from '../../store/network/actions';
import { INetworck } from '../../store/network/types';




type props ={};
const ButtonPanel: FC<props> = ({ }) => {
    const actionsList = [
        { icon: <FileDownloadIcon />, name: 'Dowload', onClick: () => { networkActions.initNetwork();actions.initSettings();} },
        { icon: <FileUploadIcon />, name: 'Uplink', onClick: () => { actions.saveSettings();
          networkActions.saveNetwork();
        } }
      ];
      const dispatch = useDispatch();

    return (
        <Box sx={{ height: 300, position: "sticky", bottom: "0vh", transform: 'translateZ(0px)', flexGrow: 1 }}>
        <SpeedDial
          ariaLabel="SpeedDial openIcon example"
          sx={{
            position: 'absolute', bottom: 16, right: 16, ".MuiSpeedDial-fab": { backgroundColor: "#05101efe", opacity: "0.8" },
            ".MuiSpeedDial-fab:hover": { backgroundColor: "#05101efe", opacity: "0.8" },
            ".MuiSpeedDialAction-fab": { backgroundColor: "#05101efe", color: "#E4E4E4", opacity: "0.9" },
          }}
          icon={<SpeedDialIcon openIcon={<CloseIcon />} icon={<MenuIcon />} />}>
          {actionsList.map((action) => (
            <SpeedDialAction
              key={action.name}
              icon={action.icon}
              tooltipTitle={action.name}
              onClick={action.onClick}
            />
          ))}
        </SpeedDial>
      </Box>
    );
}
export default ButtonPanel;
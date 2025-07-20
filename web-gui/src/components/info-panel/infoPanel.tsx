
import React, { FC } from 'react';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import MenuItem from '@mui/material/MenuItem';
import InputLabel from '@mui/material/InputLabel';
import Chip from '@mui/material/Chip';
import {Box, Typography, styled, TextField} from '@mui/material';
import FormControl, { useFormControl } from '@mui/material/FormControl';
import { useDispatch, useSelector , shallowEqual} from 'react-redux';
import * as actions from '../../store/settings/actions';
import LinearProgress, { LinearProgressProps } from '@mui/material/LinearProgress';
import NorthIcon from '@mui/icons-material/North';

import { IRootState } from '../../store';
import { IInfo } from '../../store/info/types';

type props ={ };



type Props = {
    fontcolor?: string;
};

const options = {
    shouldForwardProp: (prop: any) => prop !== 'fontcolor',
};


const StyledTextField = styled(
    TextField,
    options,
)<Props>(({ fontcolor }) => ({
    input: {
        color: fontcolor,
        borderColor: fontcolor,
        textAlign:"right"

    },
    label: {
        color: fontcolor,
        borderColor: fontcolor,
    },


}));

const InfoPanel: FC<props> = ({ }) => {

const  info:IInfo =useSelector((state:IRootState)=>{return state.info},shallowEqual);

  return(
    <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "flex-start", width: "200px", background: "linear-gradient(130deg,#05101e, #061834f0)", borderRadius: "10px", padding: "10px", margin: "5px" ,  boxShadow: '8px 8px 3px -3px rgba(0,0,0,.09)' }}>
       <Typography variant="subtitle1" component="div" sx={{  color: "#E4E4E4" }}>
          Diagnostic
          </Typography>
       <FormControl  variant="standard" sx={{
              margin: "5%",
              "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
              "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #E2E2E2" },
              "& .MuiSvgIcon-root": {
                  color: "#E2E2E2",
              }}}>
       <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Usage RAM"
                value={(info.UsageRam/1024/1024).toFixed(2)+"/"+(info.TotalRam/1024/1024).toFixed(0)+" MB"}
                sx={{
                    margin: "0% 5% 0% 5%",
                    "& .MuiInput-underline:before": { borderBottom: "0px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "0px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                />
                <LinearProgress variant="determinate" value={info.UsageRam*100/(info.TotalRam)} sx={{margin:"0% 5% 0% 5%",background:"#E2E2E2",".MuiLinearProgress-barColorPrimary":{background:"#1e4774"}}} />
        <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Usage CPU"
                value={info.UsageCPU.toFixed(2)+" %"}
                sx={{
                    margin: "5% 5% 0% 5%",
                    "& .MuiInput-underline:before": { borderBottom: "0px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "0px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                />
   <LinearProgress variant="determinate" value={info.UsageCPU} sx={{margin:"0% 5% 0% 5%",background:"#E2E2E2",".MuiLinearProgress-barColorPrimary":{background:"#1e4774"}}} />
        <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Upload data"
                value={(info.UplinkData/1024/1024).toFixed(2)+" MB"}
                sx={{
                    margin: "5%",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
            />
        <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Download data"
                value={(info.DownlinkData/1024/1024).toFixed(2)+" MB"}
                sx={{
                    margin: "5%",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                />
        <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Upload speed"
                value={(info.UplinkSpeed/1024).toFixed(2)+" kB/s"}
                sx={{
                    margin: "5%",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                 />
        <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Download speed"
                value={(info.DownlinkSpeed/1024).toFixed(2)+" kB/s"}
                sx={{
                    margin: "5%",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                />
                <StyledTextField
                aria-readonly
                color="info"
                id="outlined-required"
                label="Up time"
                value={getTimeString(info.Uptime/1000)}
                sx={{
                    margin: "5%",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                 />
        </FormControl>
      </Box>
  
  
  );}
  
  const getTimeString=(numb:number)=>{
    return `${(numb/3600).toFixed(0)}h:${((numb%3600)/60).toFixed(0)}m:${((numb%3600)%60).toFixed(0)}s:${(((numb%3600)%60)%1*1000).toFixed(0)}ms`
  }
  
  export default InfoPanel;
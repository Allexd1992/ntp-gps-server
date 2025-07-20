
import React, {FC} from 'react';

import * as logActions from '../../store/login/actions';
import Backdrop from '@mui/material/Backdrop';
import { Box, Typography, Button, TextField, createTheme, MenuItem, InputLabel, styled, FormControl } from '@mui/material';

import { IUser } from '../../store/login/types';
import { useSelector } from 'react-redux';
import { IRootState } from '../../store';
import { useDispatch } from 'react-redux';


type props ={};

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
  
  
    },
    label: {
        color: fontcolor,
        borderColor: fontcolor,
    },
  
  
  }));
const LoginPanel: FC<props> = ({ }) => {


    const login:IUser = useSelector((state:IRootState)=>state.login);
    const dispatch = useDispatch();
return(<Backdrop   open={true} sx={{ color: '#red', zIndex: "2px" }}
>
<Box sx={{display: "flex", flexDirection: "column", justifyContent: "flex-start", width: "400px", background: "linear-gradient(130deg,#05101efe, #061834f0)", borderRadius: "10px", padding: "10px", margin: "5px" ,  boxShadow: '8px 8px 3px -3px rgba(0,0,0,.09)' }}>
<FormControl  >
<Typography variant="h6" component="div" sx={{  color: "#E4E4E4",textAlign:"center", margin:"10px 10px 5px 0px"  }}>
 GPS NTP Server
  </Typography>

      <StyledTextField
          required
          color="info"
          id="outlined-required"
          label="Login"

          sx={{
              margin: "0px 20px 0px 20px",
              "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
              "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
          }}
          variant="standard"
          size="small"
          fontcolor="#E4E4E4"
          value={login.login}
          onChange={(e:any)=>{dispatch(logActions.setLogin(e.target.value))}}
          />
          <StyledTextField
          required
          security=""
          color="info"
          id="outlined-required"
          label="Password"
          type={"password"}
          sx={{
              margin: "5%",
              "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
              "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
          }}
          variant="standard"
          size="small"
          fontcolor="#E4E4E4"
          value={login.password}
          onChange={(e:any)=>{dispatch(logActions.setPassword(e.target.value))}}
          />
           <Button sx={{margin:"5%", background:"#163661"}} onClick={()=>{logActions.login2()}} variant="contained" disableElevation>
Sing In
</Button>
  </FormControl>

</Box>
</Backdrop>)};

export default LoginPanel;
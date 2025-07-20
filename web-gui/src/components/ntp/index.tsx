
import React, { FC } from 'react';
import { Box, Typography, TextField, createTheme, MenuItem, InputLabel, styled, FormControl, Switch, FormControlLabel, InputAdornment, Chip, Stack } from '@mui/material';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { useDispatch, useSelector, shallowEqual } from 'react-redux';
import * as actions from '../../store/settings/actions';
import { IMonitoring } from '../../store/monitoring/types';

type props = {};

type Props = {
    fontcolor?: string ;
    backgroundcolor?:string;
};

const options = {
    shouldForwardProp: (prop: any) => prop !== 'fontcolor',
};
const options2 = {
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


const StyledChip = styled(
    Chip,
    options2,
  )<Props>(({ fontcolor, backgroundcolor }) => ({
    div: {
        color: fontcolor,
        backgroundColor: backgroundcolor,
  
  
    },
    span: {
        color: fontcolor,
        
  
    },

    color:fontcolor,
    backgroundColor: backgroundcolor,
  
  
  }));

const Android12Switch = styled(Switch)(({ theme }) => ({
    padding: 8,
    '& .MuiSwitch-track': {
        borderRadius: 22 / 2,
        '&:before, &:after': {
            content: '""',
            position: 'absolute',
            top: '50%',
            transform: 'translateY(-50%)',
            width: 16,
            height: 16,
        },
        '&:before': {
            backgroundImage: `url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 24 24"><path fill="${encodeURIComponent(
                theme.palette.getContrastText(theme.palette.primary.main),
            )}" d="M21,7L9,19L3.5,13.5L4.91,12.09L9,16.17L19.59,5.59L21,7Z"/></svg>')`,
            left: 12,
        },
        '&:after': {
            backgroundImage: `url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 24 24"><path fill="${encodeURIComponent(
                theme.palette.getContrastText(theme.palette.primary.main),
            )}" d="M19,13H5V11H19V13Z" /></svg>')`,
            right: 12,
        },
    },
    '& .MuiSwitch-thumb': {
        boxShadow: 'none',
        width: 16,
        height: 16,
        margin: 2,
    },
}));


const NTP: FC<props> = ({ }) => {
    let cycle: number = useSelector((state: any) => state.settings.ntp.cycle, shallowEqual);
    let enable: boolean = useSelector((state: any) => state.settings.ntp.enable, shallowEqual);
    let list: string[] = useSelector((state: any) => state.settings.ntp.server_list, shallowEqual);
    let monitoring: IMonitoring = useSelector((state: any) => state.monitoring, shallowEqual);
    const dispatch = useDispatch();
    return (<div>

        <Box sx={{ display: "flex", flexDirection: "column", justifyContent: "flex-start", width: "200px", background: "linear-gradient(130deg,#05101efe, #061834f0)", borderRadius: "10px", padding: "10px", margin: "5px", boxShadow: '8px 8px 3px -3px rgba(0,0,0,.09)' }}>
            <Typography variant="subtitle1" component="div" sx={{ color: "#E4E4E4", }}>
                NTP клиент
            </Typography>
            
            <FormControlLabel control={<Android12Switch
                checked={enable}
                onChange={(e) => dispatch(actions.ntpEnable())}
                inputProps={{ 'aria-label': 'controlled' }}
            
            />} label="Активен" sx={{ color: "#E4E4E4", paddingLeft: 1 }} />
            <FormControl >
                <StyledTextField
                    required
                    className='textfield'
                    id="outlined-disabled"
                    label="Интервал обновления"
                    type="number"
                    value={cycle/1000}
                    sx={{
                        margin: "5%",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    color="info"
                    fontcolor="#E4E4E4"
                    InputProps={{
                        endAdornment: <InputAdornment position="end" sx={{ color: "#E4E4E4" }}  >сек.</InputAdornment>

                      }}
            
                 onChange={(e)=>{dispatch(actions.setNtpCycle(Number(e.target.value)*1000))}}
                 
                />
            </FormControl>
            


                    <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.1, flexDirection: 'column', }}>
                    
                          <Stack direction="column" spacing={1}>
                            {list.map((value) => (
                              <StyledChip
                                key={value}
                                label={value}
                                fontcolor="#1E1E1E"
                                backgroundcolor="#E2E2E2"
                                onDelete={(e) => {
                                  console.log(value);
                                  dispatch(actions.removeNtpAddr(value));
                                }}

                            sx={{
                                width: '100%',
                                display: 'flex',
                                justifyContent: 'space-between',


                            }}
                              />
                            ))}
                           
                          </Stack>
                        
                       
                       <StyledTextField
        
                    required
                    className='textfield'
                    id="outlined-disabled"
                    label="Адрес NTP срвера"
                    type="string"
                    sx={{
                        margin: "5%",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    onKeyDown={(e:any)=>{if(e.nativeEvent.key==='Enter'){dispatch(actions.addNtpAddr(e.target.value));e.target.value=""}}}
                    variant="standard"
                    size="small"
                    color="info"
                    fontcolor="#E4E4E4"
                      />
                      <StyledTextField

                      className='textfield'
      
                      color="info"
                      id="outlined-read-only-input"
                      label="Последнее обновление"
                      type="string"
                      sx={{
                          margin: "0px 20px 0px 10px",
                          "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                          "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                      }}
                      variant="standard"
                      size="small"
                      fontcolor="#E4E4E4"
                      value={(monitoring.actial - monitoring.last_ntp).toString() + " секунд назад"}
                      InputProps={{
                          readOnly: true,
                      }}
                />
                    </Box>
                
        </Box>

    </div>
    );

}


export default NTP;
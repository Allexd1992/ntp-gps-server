
import React, { FC } from 'react';
import { Box, Typography, TextField, createTheme, MenuItem, InputLabel, styled, FormControl, Switch, FormControlLabel, InputAdornment, Chip, Stack } from '@mui/material';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { useDispatch, useSelector, shallowEqual } from 'react-redux';
import * as actions from '../../store/network/actions';
import { INetworck } from '../../store/network/types';
import { IMonitoring } from '../../store/monitoring/types';

type props = {};

type Props = {
    fontcolor?: string;
    backgroundcolor?: string;
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

    color: fontcolor,
    backgroundColor: backgroundcolor,


}));


const Monitoring: FC<props> = ({ }) => {
    let monitoring: IMonitoring = useSelector((state: any) => state.monitoring, shallowEqual);

    const dispatch = useDispatch();
    return (<div>

        <Box sx={{ display: "flex", flexDirection: "column",gap:"5px", justifyContent: "flex-start", width: "240px", background: "linear-gradient(130deg,#05101efe, #061834f0)", borderRadius: "10px", padding: "10px", margin: "5px", boxShadow: '8px 8px 3px -3px rgba(0,0,0,.09)' }}>
            <Typography variant="subtitle1" component="div" sx={{ color: "#E4E4E4", }}>
                Диагностика NTP сервера
            </Typography>

            <StyledTextField

                className='textfield'

                color="info"
                id="outlined-read-only-input"
                label="Актуальное время UTC"
                type="string"
                sx={{
                    fontSize:"100%",
                    margin: "0px 20px 0px 20px",
                    "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                    "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                }}
                variant="standard"
                size="small"
                fontcolor="#E4E4E4"
                value={new Date(monitoring.actial * 1000).toUTCString()}
                InputProps={{
                    readOnly: true,
                  }}
                onChange={(e: any) => { dispatch(actions.setAddres(e.target.value)) }}
            />
            <FormControl >
                <StyledTextField

                    className='textfield'


                    color="info"
                    id="outlined-read-only-input"
                    label="Обновление с GPS"
                    type="string"
                    sx={{
                        color:"white",
                        margin: "0px 20px 0px 20px",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    fontcolor="#E4E4E4"
                    backgroundcolor='white'
                    value={(monitoring.actial - monitoring.last_gps).toString() + " секунд назад"}
                    InputProps={{
                        readOnly: true,
                      }}
                />
                <StyledTextField

                    className='textfield'

                    color="info"
                    id="outlined-read-only-input"
                    label="Обновление с NTP"
                    type="string"
                    sx={{
                        margin: "0px 20px 0px 20px",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    fontcolor="#E4E4E4"
                    InputProps={{
                        readOnly: true,
                      }}
                    value={(monitoring.actial - monitoring.last_ntp).toString() + " секунд назад"}

                />
            </FormControl>

        </Box>

    </div>
    );

}


export default Monitoring;
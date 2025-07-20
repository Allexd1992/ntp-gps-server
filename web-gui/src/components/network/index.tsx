
import React, { FC } from 'react';
import { Box, Typography, TextField, createTheme, MenuItem, InputLabel, styled, FormControl, Switch, FormControlLabel, InputAdornment, Chip, Stack } from '@mui/material';
import Select, { SelectChangeEvent } from '@mui/material/Select';
import { useDispatch, useSelector, shallowEqual } from 'react-redux';
import * as actions from '../../store/network/actions';
import { INetworck } from '../../store/network/types';

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


const Network: FC<props> = ({ }) => {
    let networck: INetworck = useSelector((state: any) => state.network, shallowEqual);

    const dispatch = useDispatch();
    return (<div>

        <Box sx={{ display: "flex", flexDirection: "column",gap:"5px", justifyContent: "flex-start", width: "200px", background: "linear-gradient(130deg,#05101efe, #061834f0)", borderRadius: "10px", padding: "10px", margin: "5px", boxShadow: '8px 8px 3px -3px rgba(0,0,0,.09)' }}>
            <Typography variant="subtitle1" component="div" sx={{ color: "#E4E4E4", }}>
                Настройки сети
            </Typography>


            <FormControl >
                
                <StyledTextField

                    className='textfield'

                    color="info"
                    id="outlined-disabled"
                    label="IPV4 адрес"
                    type="string"
                    sx={{
                        margin: "0px 20px 0px 20px",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    fontcolor="#E4E4E4"
                    value={networck.address}
                    onChange={(e: any) => { dispatch(actions.setAddres(e.target.value)) }}
                />
                <StyledTextField

                    color="info"
                    id=""
                    label="Адрес шлюза"

                    sx={{
                        margin: "0px 20px 0px 20px",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    fontcolor="#E4E4E4"
                    value={networck.gateway}
                    onChange={(e: any) => { dispatch(actions.setGateway(e.target.value)) }}
                />
                <StyledTextField

                    color="info"
                    id="outlined-required"
                    label="DNS"

                    sx={{
                        margin: "0px 20px 0px 20px",
                        "& .MuiInput-underline:before": { borderBottom: "1px solid #E2E2E2" },
                        "& .MuiInput-underline:hover:before": { borderBottom: "1px solid #9292E2" }
                    }}
                    variant="standard"
                    size="small"
                    fontcolor="#E4E4E4"
                    value={networck.dns}
                    onChange={(e: any) => { dispatch(actions.setDns(e.target.value)) }}
                />
            </FormControl>

        </Box>

    </div>
    );

}


export default Network;

import React, { FC } from 'react';
import HeadPanel from '../../components/head-panel/headPanel';

import ButtonPanel from '../../components/button-panel/buttonPanel';
import NTP from '../ntp';
import GPS from '../gps';
import Display from '../display';
import RTC from '../rtc';
import Network from '../network';
import Monitoring from '../monitoring';
import InfoPanel from '../info-panel/infoPanel';



type props = {};




const MainPanel: FC<props> = ({ }) => {

  return (
    <div>
      <HeadPanel />
      <div style={{ display: "flex", flexWrap: "wrap", justifyContent: "center" }}>

        <div>
          <RTC />
          <GPS />



        </div><NTP />
        <div>
          <Network />
          <Display />
        </div>
        <InfoPanel />


      </div >
      <div style={{ position: "sticky", bottom: "0vh", transform: 'translateZ(0px)', flexGrow: 1 }}>
        <ButtonPanel />
      </div>
    </div>
  );
}
export default MainPanel;
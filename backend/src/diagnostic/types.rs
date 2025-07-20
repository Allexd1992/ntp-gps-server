use serde::{Deserialize, Serialize};

use sysinfo::{CpuExt, NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use utoipa::ToSchema;
pub struct MonitoringPacket {
    pub UsageRam: u64,
    pub TotalRam: u64,
    pub UsageCPU: f32,
    pub UplinkSpeed: u64,
    pub UplinkData: u64,
    pub DownlinkSpeed: u64,
    pub DownlinkData: u64,
    pub Uptime: u64,
    _sys: System,
}
#[derive(Debug, Clone, Deserialize, Serialize, ToSchema)]
pub struct DiagnosticPacket {
    pub UsageRam: u64,
    pub TotalRam: u64,
    pub UsageCPU: f32,
    pub UplinkSpeed: u64,
    pub UplinkData: u64,
    pub DownlinkSpeed: u64,
    pub DownlinkData: u64,
    pub Uptime: u64,
}

impl MonitoringPacket {
    pub fn new() -> Self {
        MonitoringPacket {
            UsageRam: 0,
            UsageCPU: 0.0,
            UplinkSpeed: 0,
            UplinkData: 0,
            DownlinkSpeed: 0,
            Uptime: 0,
            DownlinkData: 0,
            TotalRam: 0,
            _sys: System::new_all(),
        }
    }
    pub fn refresh(&mut self) {
        self._sys.refresh_all();
        let mut received = 0;
        let mut transmitted = 0;
        let mut totalreceived = 0;
        let mut totaltransmitted = 0;
        for (interface_name, data) in self._sys.networks() {
            received += data.received();
            transmitted += data.transmitted();
            totalreceived += data.total_received();
            totaltransmitted += data.total_transmitted();
        }
        self.DownlinkSpeed = received;
        self.DownlinkData = totalreceived;
        self.UplinkSpeed = transmitted;
        self.UplinkData = totaltransmitted;
        self.TotalRam = self._sys.total_memory();

        for (pid, process) in self._sys.processes() {
            if (process.name().contains("backend")) {
                self.Uptime = get_timestamp() - process.start_time() * 1000;
                self.UsageCPU = process.cpu_usage();
                self.UsageRam = process.memory()
            }
        }
    }
}

fn get_timestamp() -> u64 {
    let current_time = time::get_time();
    let milliseconds = (current_time.sec as u64 * 1000) + (current_time.nsec as u64 / 1000 / 1000);
    return milliseconds;
}

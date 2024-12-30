use std::{fmt::Display, net::Ipv4Addr, str::FromStr};

use chrono::{DateTime, TimeZone, Utc};
use log::{error, warn};
use pnet::util::MacAddr;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct XmlDevice {
    mac_addr: String,
    ip_addr: String,
    os: Option<String>,
    scan: Option<bool>
}

impl XmlDevice {
    fn to_soft_device(&self) -> Result<SoftDevice, ()> {
        let mac_addr = match MacAddr::from_str(&self.mac_addr) {
            Ok(m) => m,
            Err(_) => {
                warn!("Failed to read MAC Address {}, using default MAC Address", self.mac_addr);
                /* TODO: Add config option where process exits upon failed read */
                MacAddr::default()
            }
        };
        let ip_addr = match Ipv4Addr::from_str(&self.ip_addr) {
            Ok(ip) => ip,
            Err(_) => {
                warn!("Failed to read IP Address {}, using default IP Address (127.0.0.1)", self.ip_addr);

                Ipv4Addr::new(127, 0, 0, 1)
            }
        };
        let os = match &self.os {
            Some(os) => os.clone(),
            None => "N/A".to_string()
        };
        let scan = match self.scan {
            Some(b) => b,
            None => true
        };
        Ok(SoftDevice {
            mac_addr,
            ip_addr,
            os,
            scan
        })
    }
}

impl Display for XmlDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_sys = match &self.os {
            Some(s) => s.clone(),
            None => "N/A".to_string()
        };
        let scan = match self.scan {
            Some(b) => b,
            None => true
        };
        write!(f, "({}, {}, {}, {})", self.mac_addr, self.ip_addr, op_sys, scan)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct SoftDevice {
    mac_addr: MacAddr,
    ip_addr: Ipv4Addr,
    os: String,
    scan: bool
}

#[derive(Serialize, Deserialize)]
pub struct MachineDevice {
    ts: DateTime<Utc>,
    mac_addr: MacAddr,
    ip_addr: Ipv4Addr,
    os: Option<String>,
    pas: Option<PortsAndServices>
}

#[derive(Serialize, Deserialize)]
pub struct PortsAndServices {
    ports: Vec<u16>,
    services: Vec<String>
}

#[test]
fn test_xml_to_softdevice() {
    let xmdev = XmlDevice {
        mac_addr: "12:34:56:78:9a:bc".to_string(),
        ip_addr: "127.0.0.1".to_string(),
        os: None,
        scan: Some(true)
    };

    let softdevice = xmdev.to_soft_device().unwrap();

    assert_eq!(softdevice, SoftDevice{
        mac_addr: MacAddr::new(0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc),
        ip_addr: Ipv4Addr::new(127, 0, 0, 1),
        os: "N/A".to_string(),
        scan: true
    })
}

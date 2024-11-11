use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct APIResponse {
    pub success: bool,
    pub datacenters: Vec<DataCenter>,
    pub shadowsocks: ShadowSocks,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct DataCenter {
    pub slug: String,
    pub city: String,
    pub country: String,
    pub country_name: String,
    pub pools: Vec<String>,
    pub ping_address: Ipv4Addr,
    pub servers: Vec<Server>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct Server {
    pub ip: Ipv4Addr,
    pub ptr: String,
    pub name: String,
    pub online: bool,
    pub load: u8,
    pub public_key: String,
    pub public_key_ipv4: String,
    pub wireguard_ports: Vec<u16>,
    pub multihop_openvpn_port: u16,
    pub multihop_wireguard_port: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ShadowSocks {
    pub mode: String,
    pub method: String,
    pub password: String,
    pub ports: Vec<u16>,
}

// per city
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct StatusResponse {
    pub data: Vec<ServerInfo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ServerInfo {
    pub online: bool,
    pub uptime: String,
    pub bandwidth: u16,
    pub bandwidth_usage: u8,
    pub port_speed: u16,
    pub name: String,
    pub ip: Ipv4Addr,
}

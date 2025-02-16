use colored::{
    Color::{Green, Red},
    Colorize,
};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{error::Error, net::Ipv4Addr};

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let slugs = client
        .get("https://www.ovpn.com/v2/api/client/entry")
        .send()?
        .json::<APIResponse>()?
        .datacenters
        .into_iter()
        .map(|dc| dc.slug)
        .collect::<Vec<String>>();

    let mut servers: Vec<(String, ServerInfo)> = vec![];

    for slug in slugs {
        client
            .get(format!(
                "https://status.ovpn.com/datacenters/{slug}/servers"
            ))
            .send()?
            .json::<StatusResponse>()?
            .data
            .into_iter()
            .for_each(|server| {
                servers.push((
                    format!("{}{}", slug[..1].to_uppercase(), &slug[1..]),
                    server,
                ));
            });
    }

    let mut previous_city = String::new();

    for (city, server) in servers {
        if city != previous_city {
            print!("\n{}", format!("{city:<11}:").green());

            previous_city = city;
        }

        print!(
            " {}",
            server.name[3..].color(if server.online { Green } else { Red })
        );
    }

    println!();

    Ok(())
}

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

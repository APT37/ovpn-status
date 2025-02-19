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
struct APIResponse {
    success: bool,
    datacenters: Vec<DataCenter>,
    shadowsocks: ShadowSocks,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct DataCenter {
    slug: String,
    city: String,
    country: String,
    country_name: String,
    pools: Vec<String>,
    ping_address: Ipv4Addr,
    servers: Vec<Server>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct Server {
    ip: Ipv4Addr,
    ptr: String,
    name: String,
    online: bool,
    load: u8,
    public_key: String,
    public_key_ipv4: String,
    wireguard_ports: Vec<u16>,
    multihop_openvpn_port: u16,
    multihop_wireguard_port: u16,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct ShadowSocks {
    mode: String,
    method: String,
    password: String,
    ports: Vec<u16>,
}

// per city
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct StatusResponse {
    data: Vec<ServerInfo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord)]
struct ServerInfo {
    online: bool,
    uptime: String,
    bandwidth: u16,
    bandwidth_usage: u8,
    port_speed: u16,
    name: String,
    ip: Ipv4Addr,
}

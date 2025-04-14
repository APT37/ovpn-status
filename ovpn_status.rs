use colored::{ Color::{ Green, Red }, Colorize };
use reqwest::blocking::Client;
use serde::Deserialize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let slugs = client
        .get("https://www.ovpn.com/v2/api/client/entry")
        .send()?
        .json::<Datacenters>()?
        .datacenters.into_iter()
        .map(|dc| dc.slug)
        .collect::<Vec<String>>();

    let width = slugs
        .iter()
        .max_by_key(|slug| slug.len())
        .expect("determine longest slug")
        .len();

    let mut servers = vec![];

    for slug in slugs {
        client
            .get(format!("https://status.ovpn.com/datacenters/{slug}/servers"))
            .send()?
            .json::<StatusResponse>()?
            .data.into_iter()
            .for_each(|si| {
                servers.push((slug[..1].to_uppercase() + &slug[1..], si));
            });
    }

    let mut previous_city = String::new();

    for (city, si) in servers {
        if city != previous_city {
            print!("\n{}", format!(" {city:<width$} |").green());

            previous_city = city;
        }

        print!(" {}", si.name[3..].color(if si.online { Green } else { Red }));
    }

    println!();

    Ok(())
}

#[derive(Deserialize)]
struct Datacenters {
    datacenters: Vec<DataCenter>,
}

#[derive(Deserialize)]
struct DataCenter {
    slug: String,
}

#[derive(Deserialize)]
struct StatusResponse {
    data: Vec<ServerInfo>,
}

#[derive(Deserialize)]
struct ServerInfo {
    online: bool,
    name: String,
}

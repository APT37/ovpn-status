use colored::{ Color::{ Green, Red }, Colorize };
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{ error::Error, fmt, sync::OnceLock };

static WIDTH: OnceLock<usize> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    let slugs = client
        .get("https://www.ovpn.com/v2/api/client/entry")
        .send()?
        .json::<Cities>()?
        .datacenters.into_iter()
        .collect::<Vec<City>>();

    WIDTH.get_or_init(||
        slugs
            .iter()
            .max_by_key(|city| city.slug.len())
            .expect("determine longest slug")
            .slug.len()
    );

    let mut servers = vec![];

    for city in slugs {
        client
            .get(format!("https://status.ovpn.com/datacenters/{}/servers", city.slug))
            .send()?
            .json::<StatusReport>()?
            .data.into_iter()
            .for_each(|server| {
                servers.push((city.clone(), server));
            });
    }

    let mut previous_city = City::default();

    for (city, server) in servers {
        if city != previous_city {
            print!("{city}");

            previous_city = city;
        }

        print!("{server}");
    }

    println!();

    Ok(())
}

#[derive(Deserialize)]
struct Cities {
    datacenters: Vec<City>,
}

#[derive(Deserialize, Default, Eq, PartialEq, Clone)]
struct City {
    slug: String,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let city = self.slug[..1].to_uppercase() + &self.slug[1..];

        let width = WIDTH.get().expect("get width from OnceLock");

        write!(f, "\n{}", format!("{city:<width$} |").green())
    }
}

#[derive(Deserialize)]
struct StatusReport {
    data: Vec<Server>,
}

#[derive(Deserialize)]
struct Server {
    online: bool,
    name: String,
}

impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " {}", self.name[3..].color(if self.online { Green } else { Red }))
    }
}

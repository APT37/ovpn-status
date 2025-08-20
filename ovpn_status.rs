use colored::{ Color::{ Green, Red }, Colorize };
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{ error::Error, fmt, sync::{ LazyLock, OnceLock } };

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

static WIDTH: OnceLock<usize> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let cities = Cities::new()?;

    WIDTH.get_or_init(|| cities.width());

    let mut previous_city = City::default();

    for (city, server) in cities.servers() {
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

impl Cities {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(CLIENT.get("https://www.ovpn.com/v2/api/client/entry").send()?.json::<Cities>()?)
    }

    fn width(&self) -> usize {
        self.datacenters
            .iter()
            .max_by_key(|city| city.slug.len())
            .expect("determine longest slug")
            .slug.len()
    }

    fn servers(&self) -> Vec<(City, Server)> {
        self.datacenters
            .iter()
            .flat_map(|city| {
                city.servers().unwrap_or_else(|_| panic!("get servers for {}", city.slug))
            })
            .collect()
    }
}

#[derive(Deserialize, Default, Eq, PartialEq, Clone)]
struct City {
    slug: String,
}

impl City {
    fn servers(&self) -> Result<Vec<(City, Server)>, Box<dyn Error>> {
        Ok(
            CLIENT.get(format!("https://status.ovpn.com/datacenters/{}/servers", self.slug))
                .send()?
                .json::<StatusReport>()?
                .data.into_iter()
                .map(|server| (self.clone(), server))
                .collect()
        )
    }
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

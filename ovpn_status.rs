use colored::{ Color::{ Green, Red }, Colorize };
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{
    error::Error,
    fmt::{ self, Display, Formatter },
    sync::{ LazyLock, atomic::{ AtomicUsize, Ordering::Relaxed } },
};

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);
static WIDTH: AtomicUsize = AtomicUsize::new(0);

fn main() -> Result<(), Box<dyn Error>> {
    let datacenters = Datacenters::fetch()?;

    WIDTH.store(datacenters.max_width(), Relaxed);

    let mut previous_datacenter = Datacenter::default();

    for (datacenter, server) in datacenters.servers() {
        if datacenter != previous_datacenter {
            print!("{datacenter}");

            previous_datacenter = datacenter;
        }

        print!("{server}");
    }

    println!();

    Ok(())
}

#[derive(Deserialize)]
struct Datacenters {
    datacenters: Vec<Datacenter>,
}

impl Datacenters {
    fn fetch() -> Result<Self, Box<dyn Error>> {
        Ok(CLIENT.get("https://www.ovpn.com/v2/api/client/entry").send()?.json()?)
    }

    fn max_width(&self) -> usize {
        self.datacenters
            .iter()
            .map(|datacenter| datacenter.slug.len())
            .max()
            .expect("find maximum slug length")
    }

    fn servers(&self) -> Vec<(Datacenter, Server)> {
        self.datacenters
            .iter()
            .flat_map(|dc| {
                dc.servers().unwrap_or_else(|_| panic!("get servers from {}", dc.slug))
            })
            .collect()
    }
}

#[derive(Deserialize, Default, PartialEq, Clone)]
struct Datacenter {
    slug: String,
}

impl Datacenter {
    fn servers(&self) -> Result<Vec<(Datacenter, Server)>, Box<dyn Error>> {
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

impl Display for Datacenter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let dc_slug = self.slug[..1].to_uppercase() + &self.slug[1..];

        write!(f, "\n{}", format!("{dc_slug:<0$} |", WIDTH.load(Relaxed)).green())
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

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, " {}", self.name[3..].color(if self.online { Green } else { Red }))
    }
}

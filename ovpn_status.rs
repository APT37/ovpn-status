use colored::{ Color::{ Green, Red }, Colorize };
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{ error::Error, fmt, sync::{ LazyLock, OnceLock } };

static CLIENT: LazyLock<Client> = LazyLock::new(Client::new);

static WIDTH: OnceLock<usize> = OnceLock::new();

fn main() -> Result<(), Box<dyn Error>> {
    let datacenters = Datacenters::new()?;

    WIDTH.get_or_init(|| datacenters.width());

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
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(CLIENT.get("https://www.ovpn.com/v2/api/client/entry").send()?.json::<Datacenters>()?)
    }

    fn width(&self) -> usize {
        self.datacenters
            .iter()
            .max_by_key(|datacenter| datacenter.slug.len())
            .expect("find longest slug")
            .slug.len()
    }

    fn servers(&self) -> Vec<(Datacenter, Server)> {
        self.datacenters
            .iter()
            .flat_map(|datacenter| {
                datacenter
                    .servers()
                    .unwrap_or_else(|_| panic!("get servers for {}", datacenter.slug))
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

impl fmt::Display for Datacenter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datacenter = self.slug[..1].to_uppercase() + &self.slug[1..];

        let width = WIDTH.get().expect("get width from OnceLock");

        write!(f, "\n{}", format!("{datacenter:<width$} |").green())
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

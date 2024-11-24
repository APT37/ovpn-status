use colored::{
    Color::{Green, Red},
    Colorize,
};
use ovpn_status::{APIResponse, ServerInfo, StatusResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();

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
            .for_each(|server| servers.push((slug.to_uppercase(), server)));
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

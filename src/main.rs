use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Station {
    #[serde(alias = "type", default)]
    typ: String,

    #[serde(default)]
    latitude: f32,

    #[serde(default)]
    longitude: f32,

    #[serde(default)]
    id: String,

    #[serde(default)]
    diva_id: u32,

    #[serde(default)]
    name: String,

    #[serde(default)]
    has_live_data: bool,

    #[serde(default)]
    products: Vec<String>,

    #[serde(flatten)]
    extra_fields: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct StationResponse {
    locations: Vec<Location>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Address {
    #[serde(alias = "type", default)]
    typ: String,

    #[serde(default)]
    poi: bool,

    #[serde(default)]
    has_live_data: bool,

    #[serde(default)]
    street: String,

    #[serde(flatten)]
    extra_fields: Option<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
enum Location {
    #[serde(rename = "address")]
    Address(Address),
    #[serde(rename = "station")]
    Station(Station),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = format!(
        "https://www.mvg.de/api/fahrinfo/location/queryWeb?q={query}",
        query = "Freiheit"
    );
    println!("{}", request_url);

    let client = reqwest::Client::new();
    let response = client
        .get(&request_url)
        .header(USER_AGENT, "My fancy Rust program")
        .send()
        .await?;

    println!("{}", response.status());

    let station: StationResponse = response.json().await?;
    println!("{:?}", station);
    Ok(())
}

const JSON_URL: &'static str = "https://api.zippopotam.us/us/";

#[derive(Debug)]
#[allow(dead_code)]
struct ZipResult {
    post_code: String,
    country: String,
    country_abbr: String,
    places: Vec<Place>,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Place {
    name: String,
    latitude: f64,
    longitude: f64,
    state: String,
    state_abbr: String,
    google_map_url: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut zip_code = String::from("96943");
    if args.len() > 1 {
        zip_code = args[1].clone();
    }
    let url_str = format!("{JSON_URL}{zip_code}");
    let json_res = get_json(url_str.as_str());
    let zip_result = match json_res {
        Ok(json_value) => parse_zipcode(json_value),
        Err(err) => Err(anyhow::anyhow!(err)),
    };
    match zip_result {
        Ok(zip) => {
            println!("parsed zipcode information: {zip:?}")
        }
        Err(err) => {
            println!("Could not parse zipcode information: {err:?}")
        }
    }
}

fn parse_zipcode(json_value: json::JsonValue) -> anyhow::Result<ZipResult> {
    let mut places_arr: Vec<Place> = Vec::new();
    for i in 0..json_value["places"].len() {
        let pl = &json_value["places"][i];
        // Could easily make this a Geoutils::Location
        // https://docs.rs/geoutils/latest/geoutils/
        let latitude = pl["latitude"].to_string().parse::<f64>()?;
        let longitude = pl["longitude"].to_string().parse::<f64>()?;
        let map_url: String =
            format!("https://www.google.com/maps/search/?api=1&query={latitude}%2C{longitude}",);
        places_arr.push(Place {
            name: pl["place name"].to_string(),
            latitude: latitude,
            longitude: longitude,
            state: pl["state"].to_string(),
            state_abbr: pl["state abbreviation"].to_string(),
            google_map_url: map_url,
        });
    }
    let res = ZipResult {
        post_code: json_value["post code"].to_string(),
        country: json_value["country"].to_string(),
        country_abbr: json_value["country abbreviation"].to_string(),
        places: places_arr,
    };
    Ok(res)
}

fn get_json(url: &str) -> anyhow::Result<json::JsonValue> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(json::parse(&body)?)
}

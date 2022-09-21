const JSON_URL: &'static str = "http://localhost:8080/version.json";

fn main() {
    let json_res = get_json(JSON_URL);
    let version_result = match json_res {
        Ok(json_value) => parse_version(json_value),
        Err(err) => Err(anyhow::anyhow!(err)),
    };
    match version_result {
        Ok(vers) => {
            println!("parsed version: {}", vers)
        }
        Err(err) => {
            println!("Could not parse version {:?}", err)
        }
    }
}

fn parse_version(json_value: json::JsonValue) -> anyhow::Result<f32> {
    Ok(json_value["version"].to_string().parse::<f32>()?)
}

fn get_json(url: &str) -> anyhow::Result<json::JsonValue> {
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(json::parse(&body)?)
}

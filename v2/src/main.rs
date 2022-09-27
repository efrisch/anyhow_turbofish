const JSON_BODY: &'static str = "{ \"version\": \"\" }";
// https://github.com/dtolnay/anyhow

fn main() {
    let json_res = json::parse(JSON_BODY);
    let version_result = match json_res {
        Ok(json_value) => parse_version(json_value),
        Err(err) => Err(anyhow::anyhow!(err)),
    };
    match version_result {
        Ok(vers) => {
            println!("parsed version: {vers}")
        }
        Err(err) => {
            println!("Could not parse version {err:?}")
        }
    }
}

fn parse_version(json_value: json::JsonValue) -> anyhow::Result<f32> {
    Ok(json_value["version"].to_string().parse::<f32>()?)
}

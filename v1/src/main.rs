const JSON_BODY: &'static str = "{ \"version\": \"0.8\" }";

fn main() {
    let json_res = json::parse(JSON_BODY);
    let version_result = match json_res {
        Ok(json_value) => parse_version(json_value),
        Err(err) => Err(format!("Parse error {:?}", err)),
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

fn parse_version(json_value: json::JsonValue) -> Result<f32, String> {
    match &json_value["version"] {
        json::JsonValue::String(str) => {
            // This is a turbofish.
            return match str.parse::<f32>() {
                Ok(v) => Ok(v),
                Err(err) => Err(format!("Parse error {:?}", err)),
            };
        }
        json::JsonValue::Number(numb) => {
            // Turbofish.
            return match numb.to_string().parse::<f32>() {
                Ok(v) => Ok(v),
                Err(err) => Err(format!("Parse error {:?}", err)),
            }
        }
        json::JsonValue::Short(numb) => {
            // Turbofish.
            return match numb.to_string().parse::<f32>() {
                Ok(v) => Ok(v),
                Err(err) => Err(format!("Parse error {:?}", err)),
            }
        }
        _ => Err(format!("Could not parse version from {:?}", json_value)),
    }
}

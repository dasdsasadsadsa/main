use serde::Serialize;

pub fn print_json<T: Serialize>(data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    println!("{}", json);
    Ok(())
}

pub fn to_json<T: Serialize>(data: &T) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    Ok(json)
}

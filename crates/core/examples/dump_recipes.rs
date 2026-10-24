// Dump every recipe as JSON, for designing the markdown source format.
fn main() {
    let recipes: Vec<serde_json::Value> = recibase_core::recipes::recipes()
        .iter()
        .map(|r| r.to_json_with_usage(&[]))
        .collect();
    println!("{}", serde_json::to_string_pretty(&recipes).unwrap());
}

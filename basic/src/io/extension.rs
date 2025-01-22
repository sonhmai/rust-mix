use std::collections::HashMap;

// converting btw structs and bytes
type RoomId = String;
type RoomExists = Vec<(char, RoomId)>;
type RoomMap = HashMap<RoomId, RoomExists>;

fn main() -> std::io::Result<()> {
    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(),
               vec![('W', "Debris Room".to_string())]);
    map.insert("Debris Room".to_string(),
               vec![('E', "Cobble Crawl".to_string()),
                    ('W', "Sloping Canyon".to_string())]);

    serde_json::to_writer(&mut std::io::stdout(), &map)?;

    Ok(())
}
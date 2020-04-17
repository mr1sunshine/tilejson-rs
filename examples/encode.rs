use tilejson::{TileJson, encode};

fn main() {
    let mut tilejson = TileJson::default();
    tilejson.name = Some("TileSet Name".to_owned());
    tilejson.description = Some("TileSet description".to_owned());
    let json = encode(&tilejson);
    println!("{:?}", json);
}
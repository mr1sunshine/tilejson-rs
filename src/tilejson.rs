use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct TileJson {
    /// REQUIRED. A semver.org style version number. Describes the version of
    /// the TileJSON spec that is implemented by this JSON object.
    pub tilejson: String,

    /// OPTIONAL. Default: null. A name describing the tileset. The name can
    /// contain any legal character. Implementations SHOULD NOT interpret the
    /// name as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// OPTIONAL. Default: null. A text description of the tileset. The
    /// description can contain any legal character. Implementations SHOULD NOT
    /// interpret the description as HTML.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// OPTIONAL. Default: "1.0.0". A semver.org style version number. When
    /// changes across tiles are introduced, the minor version MUST change.
    /// This may lead to cut off labels. Therefore, implementors can decide to
    /// clean their cache when the minor version changes. Changes to the patch
    /// level MUST only have changes to tiles that are contained within one tile.
    /// When tiles change significantly, the major version MUST be increased.
    /// Implementations MUST NOT use tiles with different major versions.
    #[serde(default = "default_version")]
    pub version: String,

    /// OPTIONAL. Default: null. Contains an attribution to be displayed
    /// when the map is shown to a user. Implementations MAY decide to treat this
    /// as HTML or literal text. For security reasons, make absolutely sure that
    /// this field can't be abused as a vector for XSS or beacon tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribution: Option<String>,

    /// OPTIONAL. Default: null. Contains a mustache template to be used to
    /// format data from grids for interaction.
    /// See https:///github.com/mapbox/utfgrid-spec/tree/master/1.2
    /// for the interactivity specification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,

    /// OPTIONAL. Default: null. Contains a legend to be displayed with the map.
    /// Implementations MAY decide to treat this as HTML or literal text.
    /// For security reasons, make absolutely sure that this field can't be
    /// abused as a vector for XSS or beacon tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend: Option<String>,

    /// OPTIONAL. Default: "xyz". Either "xyz" or "tms". Influences the y
    /// direction of the tile coordinates.
    /// The global-mercator (aka Spherical Mercator) profile is assumed.
    #[serde(default)]
    pub scheme: Scheme,

    /// REQUIRED. An array of tile endpoints. {z}, {x} and {y}, if present,
    /// are replaced with the corresponding integers. If multiple endpoints are specified, clients
    /// may use any combination of endpoints. All endpoints MUST return the same
    /// content for the same URL. The array MUST contain at least one endpoint.
    pub tiles: Vec<String>,

    /// OPTIONAL. Default: []. An array of interactivity endpoints. {z}, {x}
    /// and {y}, if present, are replaced with the corresponding integers. If multiple
    /// endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL.
    /// If the array doesn't contain any entries, interactivity is not supported
    /// for this tileset.
    /// See https:///github.com/mapbox/utfgrid-spec/tree/master/1.2
    /// for the interactivity specification.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub grids: Vec<String>,

    /// OPTIONAL. Default: []. An array of data files in GeoJSON format.
    /// {z}, {x} and {y}, if present,
    /// are replaced with the corresponding integers. If multiple
    /// endpoints are specified, clients may use any combination of endpoints.
    /// All endpoints MUST return the same content for the same URL.
    /// If the array doesn't contain any entries, then no data is present in
    /// the map.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<String>,

    /// OPTIONAL. Default: 0. >= 0, <= 30.
    /// An integer specifying the minimum zoom level.
    #[serde(default = "default_minzoom")]
    pub minzoom: u8,

    /// OPTIONAL. Default: 30. >= 0, <= 30.
    /// An integer specifying the maximum zoom level. MUST be >= minzoom.
    #[serde(default = "default_maxzoom")]
    pub maxzoom: u8,

    /// OPTIONAL. Default: [-180, -90, 180, 90].
    /// The maximum extent of available map tiles. Bounds MUST define an area
    /// covered by all zoom levels. The bounds are represented in WGS:84
    /// latitude and longitude values, in the order left, bottom, right, top.
    /// Values may be integers or floating point numbers.
    #[serde(default = "default_bounds")]
    pub bounds: Vec<f32>,

    /// OPTIONAL. Default: null.
    /// The first value is the longitude, the second is latitude (both in
    /// WGS:84 values), the third value is the zoom level as an integer.
    /// Longitude and latitude MUST be within the specified bounds.
    /// The zoom level MUST be between minzoom and maxzoom.
    /// Implementations can use this value to set the default location. If the
    /// value is null, implementations may use their own algorithm for
    /// determining a default location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub center: Option<Vec<f32>>
}

impl Default for TileJson {
    fn default() -> Self {
        Self {
            tilejson: default_tilejson(),
            name: Option::None,
            description: Option::None,
            version: default_version(),
            attribution: Option::None,
            template: Option::None,
            legend: Option::None,
            scheme : Default::default(),
            tiles: vec![],
            grids: vec![],
            data: vec![],
            minzoom: default_minzoom(),
            maxzoom: default_maxzoom(),
            bounds: default_bounds(),
            center: Option::None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub enum Scheme {
    #[serde(rename = "xyz")]
    XYZ,
    #[serde(rename = "tms")]
    TMS
}

impl Default for Scheme {
    fn default() -> Self { Scheme::XYZ }
}

fn default_tilejson() -> String {
    "2.2.0".to_owned()
}

fn default_version() -> String {
    "1.0.0".to_owned()
}

fn default_minzoom() -> u8 {
    0
}

fn default_maxzoom() -> u8 {
    30
}

fn default_bounds() -> Vec<f32> {
    vec![-180.0, -90.0, 180.0, 90.0]
}

pub fn decode(tilejson: &str) -> TileJson {
    serde_json::from_str(tilejson).unwrap()
}

pub fn encode(tilejson: &TileJson) -> String {
    serde_json::to_string(tilejson).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_default() {
        let encoded_str = r#"{"tilejson":"2.2.0","version":"1.0.0","scheme":"xyz","tiles":[],"minzoom":0,"maxzoom":30,"bounds":[-180.0,-90.0,180.0,90.0]}"#;
        let tilejson = TileJson::default();
        assert_eq!(encode(&tilejson), encoded_str);
    }

    #[test]
    fn test_decode_default() {
        let encoded_str = r#"{"tilejson":"2.2.0","version":"1.0.0","scheme":"xyz","tiles":[],"minzoom":0,"maxzoom":30,"bounds":[-180.0,-90.0,180.0,90.0]}"#;
        let tilejson = TileJson::default();
        assert_eq!(decode(&encoded_str), tilejson);
    }

    #[test]
    fn test_encode_example() {
        let encoded_str = r#"{"tilejson":"1.0.0","name":"OpenStreetMap","description":"A free editable map of the whole world.","version":"1.0.0","attribution":"(c) OpenStreetMap contributors, CC-BY-SA","scheme":"xyz","tiles":["https://a.tile.openstreetmap.org/{z}/{x}/{y}.png","https://b.tile.openstreetmap.org/{z}/{x}/{y}.png","https://c.tile.openstreetmap.org/{z}/{x}/{y}.png"],"minzoom":0,"maxzoom":18,"bounds":[-180.0,-85.0,180.0,85.0]}"#;
        let mut tilejson = TileJson::default();
        tilejson.tilejson = "1.0.0".to_owned();
        tilejson.name = Some("OpenStreetMap".to_owned());
        tilejson.description = Some("A free editable map of the whole world.".to_owned());
        tilejson.attribution = Some("(c) OpenStreetMap contributors, CC-BY-SA".to_owned());
        tilejson.tiles = vec![
            "https://a.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned(),
            "https://b.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned(),
            "https://c.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned()
        ];
        tilejson.maxzoom = 18;
        tilejson.bounds = vec![ -180.0, -85.0, 180.0, 85.0 ];

        assert_eq!(encode(&tilejson), encoded_str);
    }

    #[test]
    fn test_decode_example() {
        let encoded_str = r#"{
            "tilejson": "1.0.0",
            "name": "OpenStreetMap",
            "description": "A free editable map of the whole world.",
            "version": "1.0.0",
            "attribution": "(c) OpenStreetMap contributors, CC-BY-SA",
            "scheme": "xyz",
            "tiles": [
                "https://a.tile.openstreetmap.org/{z}/{x}/{y}.png",
                "https://b.tile.openstreetmap.org/{z}/{x}/{y}.png",
                "https://c.tile.openstreetmap.org/{z}/{x}/{y}.png"
            ],
            "minzoom": 0,
            "maxzoom": 18,
            "bounds": [ -180, -85, 180, 85 ]
        }"#;

        let mut tilejson = TileJson::default();
        tilejson.tilejson = "1.0.0".to_owned();
        tilejson.name = Some("OpenStreetMap".to_owned());
        tilejson.description = Some("A free editable map of the whole world.".to_owned());
        tilejson.attribution = Some("(c) OpenStreetMap contributors, CC-BY-SA".to_owned());
        tilejson.tiles = vec![
            "https://a.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned(),
            "https://b.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned(),
            "https://c.tile.openstreetmap.org/{z}/{x}/{y}.png".to_owned()
        ];
        tilejson.maxzoom = 18;
        tilejson.bounds = vec![ -180.0, -85.0, 180.0, 85.0 ];

        assert_eq!(decode(&encoded_str), tilejson);
    }
}
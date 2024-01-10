#[macro_use]
extern crate serde_derive;

extern crate serde_json;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct Obj {
    pub profile: Vec<Profile>,
    pub skills: Vec<Skills>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub city: String,
    pub profile_picture: String,
    pub bio: String,
    pub dev: String,
    pub gitlab: String,
    pub twitter: String,
    pub facebook: String,
    pub linkedin: String,
}

pub fn get_data() -> Result<Obj, Box<dyn std::error::Error>> {
    let file = File::open("./static/data/data.json")?;
    let reader = BufReader::new(file);
    let p = serde_json::from_reader(reader)?;
    Ok(p)
}

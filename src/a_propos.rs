use rocket_contrib::json::Json;
use std::io;
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::BufReader;

#[get("/a_propos")]
pub fn get_a_propos() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/a_propos/a_propos.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}
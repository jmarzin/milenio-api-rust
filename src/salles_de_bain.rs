use rocket_contrib::json::Json;
use std::io;
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::BufReader;

#[get("/salles_de_bain")]
pub fn get_services_salles_de_bain() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/services/salles_de_bain/salles_de_bain.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}
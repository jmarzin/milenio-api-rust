use rocket_contrib::json::Json;
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::BufReader;
use std::io;

#[get("/accueil/panneaux")]
pub fn get_accueil_panneaux() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/accueil/panneaux/panneaux.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}
use rocket_contrib::json::Json;
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::{BufReader, Write};
use std::io;
use crate::tools::Administrateur;

#[get("/accueil/panneaux")]
pub fn get_accueil_panneaux() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/accueil/panneaux.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Carte {
    titre: String,
    text: String,
    affiche: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostPanneaux {
    titre: String,
    colonnes: i32,
    cartes: Vec<Carte>
}

#[post("/accueil/panneaux", data = "<post_panneaux>")]
pub fn post_accueil_panneaux(_key: Administrateur, post_panneaux: Json<PostPanneaux>) -> Json<String> {
    let path = Path::new(unsafe { &BASE })
        .join("contenu/accueil/panneaux.json");
    let donnees = post_panneaux.clone();
    let chaine = ::serde_json::to_string_pretty(&donnees).unwrap();
    let mut file = File::create(path.as_path()).unwrap();
    let _result = file.write_all(chaine.as_bytes());
    Json("panneaux.json sauvegardé".to_string())
}
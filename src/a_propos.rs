use rocket_contrib::json::Json;
use std::{io, fs};
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::{BufReader, Write};
use crate::tools::Administrateur;
use crate::realisations::Photo;

#[get("/a_propos")]
pub fn get_a_propos() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/a_propos/a_propos.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DonneesActualites {
    repertoire: String,
    texte: String,
    photos: Vec<Photo>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostAPropos {
    donnees_actualites: DonneesActualites,
    menage: bool
}

#[post("/a_propos", data = "<post_a_propos>")]
pub fn post_a_propos(_key: Administrateur, post_a_propos: Json<PostAPropos>) -> Json<String> {
    let path = Path::new(unsafe { &BASE })
        .join(&post_a_propos.donnees_actualites.repertoire)
        .join("a_propos.json");
    let mut liste = Vec::new();
    for item in &post_a_propos.donnees_actualites.photos {
        if post_a_propos.menage && !item.affiche {
            let file = Path::new(unsafe { &BASE })
                .join(&post_a_propos.donnees_actualites.repertoire)
                .join(item.image.as_str());
            let _result = fs::remove_file(file.as_path());
        } else {
            liste.push(item.clone())
        }
    }
    let mut donnees = post_a_propos.donnees_actualites.clone();
    donnees.photos = liste;
    let chaine = ::serde_json::to_string_pretty(&donnees).unwrap();
    let mut file = File::create(path.as_path()).unwrap();
    let _result = file.write_all(chaine.as_bytes());
    Json("a_propos.json sauvegardé".to_string())
}
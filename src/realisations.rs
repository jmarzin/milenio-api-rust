use rocket_contrib::json::Json;
use std::{io, fs};
use serde_json::Value;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::{BufReader, Write};
use crate::tools::Administrateur;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    pub image: String,
    pub affiche: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemRealisations {
    repertoire: String,
    commentaire_prive: String,
    titre: String,
    sous_titre: String,
    texte: String,
    photos: Vec<Photo>,
    repertoires_lies: Vec<String>,
    affiche: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostRealisations {
    liste: Vec<ItemRealisations>,
    index: i32,
    menage: bool
}

#[get("/realisations")]
pub fn get_realisations() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/realisations/realisations.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}

#[post("/realisations", data = "<post_realisations>")]
pub fn post_realisations(_key: Administrateur, post_realisations: Json<PostRealisations>) -> Json<String> {
    let premier_rep = &post_realisations.liste[0].repertoire;
    let repertoire_parties: Vec<&str> = premier_rep.split("/").collect();
    let from = "/".to_string() + repertoire_parties[repertoire_parties.len() - 1];
    let repertoire_objets = premier_rep.replace(&from , "");
    let nom_fichier = repertoire_parties[repertoire_parties.len() - 2].to_string() + ".json";
    let mut liste = Vec::new();
    let mut i = 0;
    for item in &post_realisations.liste {
        if post_realisations.menage && post_realisations.index < 0 && !item.affiche {
            let dir_path = Path::new (unsafe { &BASE }).join(&item.repertoire);
            fs::remove_dir_all(dir_path).unwrap();
        } else {
            let mut item2 = item.clone();
            if post_realisations.menage && post_realisations.index == i {
                for photo in &item2.photos {
                    if !photo.affiche {
                        let path = Path::new (unsafe { &BASE }).join(&item.repertoire).join(&photo.image);
                        fs::remove_file(path).unwrap();
                    }
                }
                item2.photos = item2.photos.into_iter().filter(|el| el.affiche).collect();
            }
            liste.push(item2);
        }
        i += 1;
    }
    let chaine = ::serde_json::to_string_pretty(&liste).unwrap();
    let path = Path::new(unsafe { &BASE }).join(&repertoire_objets).join(&nom_fichier);
    let mut file = File::create(path.as_path()).unwrap();
    let _result = file.write_all(chaine.as_bytes());
    Json(nom_fichier + " sauvegardé")
}
use std::io::{BufReader, Write};
use std::path::Path;
use crate::BASE;
use rocket_contrib::json::Json;
use std::{fs, io};
use std::ffi::OsStr;
use std::fs::File;
use crate::tools::Administrateur;

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemCarousel {
    caption: String,
    photo: String,
    affiche: bool
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostCarousel {
    liste: Vec<ItemCarousel>,
    menage: bool
}

fn est_inconnu(reponse: &Vec<ItemCarousel>, fichier: &Option<&OsStr>) -> bool {
    for item in reponse {
        if Path::new(&item.photo).file_name() == *fichier {
            return false;
        }
    }
    true
}

#[get("/accueil/carousel")]
pub fn get_accueil_carousel() -> Result<Json<Vec<ItemCarousel>>, io::Error> {
    let dir_path = Path::new(unsafe { &BASE }).join("contenu/accueil/carousel");
    let path = Path::new(unsafe { &BASE }).join("contenu/accueil/carousel/carousel.json");
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut reponse: Vec<ItemCarousel> = serde_json::from_reader(reader).unwrap();
    for entry in fs::read_dir(dir_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            match path.extension().and_then(OsStr::to_str) {
                Some("jpg")|Some("JPG") => {
                    print!("{:?}", path.file_name().unwrap());
                    if est_inconnu(&reponse, &path.file_name()) {
                        reponse.push(ItemCarousel {caption: "".to_string(),
                            photo: "contenu/accueil/carousel/".to_string() + path.file_name().unwrap().to_str().unwrap(),
                            affiche: false})
                    }
                },
                _ => {}
            }
        }
    }
    Ok(Json(reponse))
}

#[post("/accueil/carousel", data = "<post_carousel>")]
pub fn post_accueil_carousel(_key: Administrateur, post_carousel: Json<PostCarousel>) -> Json<String> {
    let path = Path::new(unsafe { &BASE }).join("contenu/accueil/carousel/carousel.json");
    let mut liste = Vec::new();
    for item in &post_carousel.liste {
        if post_carousel.menage && !item.affiche {
            let file = Path::new(unsafe { &BASE }).join(item.photo.as_str());
            let _result = fs::remove_file(file.as_path());
        } else {
            liste.push(item)
        }
    }
    let chaine = ::serde_json::to_string(&liste).unwrap();
    let mut file = File::create(path.as_path()).unwrap();
    let _result = file.write_all(chaine.as_bytes());
    Json("carousel.json sauvegardé".to_string())
}

use rocket::{Outcome, Request, Data};
use rocket::request::{self, FromRequest};
use rocket::http::Status;
use rocket::http::ContentType;
use rocket_contrib::json::Json;
use rocket_multipart_form_data::{mime, MultipartFormDataOptions, MultipartFormDataField, MultipartFormData, FileField, TextField};
use std::path::Path;
use crate::BASE;
use std::fs::copy;
use std::{env, fs};

#[get("/")]
pub fn index() -> String {
    format!("api is ok wd : {} base : {}", env::current_dir().unwrap().to_str().unwrap(), unsafe { &BASE })
}

pub struct Administrateur(String);

#[derive(Debug)]
pub enum AdmError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for Administrateur {
    type Error = AdmError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, AdmError::Missing)),
            1 if is_valid_admin(keys[0]) => Outcome::Success(Administrateur(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, AdmError::Invalid)),
            _ => Outcome::Failure((Status::Unauthorized, AdmError::BadCount)),
        }
    }
}

fn is_valid_admin(key: &str) -> bool {
    key == "Basic YWRtaW46dG9uaW82MzI="
}

pub struct Utilisateur(String);

impl<'a, 'r> FromRequest<'a, 'r> for Utilisateur {
    type Error = AdmError;
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        match keys.len() {
            0 => Outcome::Failure((Status::Unauthorized, AdmError::Missing)),
            1 if is_valid_util(keys[0]) => Outcome::Success(Utilisateur(keys[0].to_string())),
            1 => Outcome::Failure((Status::Unauthorized, AdmError::Invalid)),
            _ => Outcome::Failure((Status::Unauthorized, AdmError::BadCount)),
        }
    }
}

fn is_valid_util(key: &str) -> bool {
    key == "Basic dXRpbGlzYXRldXI6bWVzc2FnZQ=="
}

#[get("/sensitive")]
pub fn sensitive(_key: Administrateur) -> Json<String> {
    Json("You are an administrator".to_string())
}

#[derive(Serialize)]
pub struct Resultat {
    resultat: String,
    fichier: String
}

#[post("/photoupload", data = "<data>")]
pub fn photo_upload(_key: Administrateur, content_type: &ContentType, data: Data) -> Json<Resultat> {
    let mut options = MultipartFormDataOptions::new();
    options.allowed_fields.push(MultipartFormDataField::file("photo").content_type_by_string(Some(mime::IMAGE_STAR)).unwrap());
    options.allowed_fields.push(MultipartFormDataField::text("destination"));

    let multipart_form_data = MultipartFormData::parse(content_type, data, options).unwrap();

    let photo = multipart_form_data.files.get("photo");
    let destination = multipart_form_data.texts.get("destination");

    let mut destination_fichier = "";
    if let Some(destination) = destination {
        match destination {
            TextField::Single(text_field) => {
                destination_fichier = &text_field.text;
            }
            _ => {}
        }
    }
    if let Some(photo) = photo {
        match photo {
            FileField::Single(file) => {
                let file_name = file.file_name.as_ref().unwrap();
                let path = &file.path;
                let dest_name = match destination_fichier {
                    "carousel_accueil" => "contenu/accueil/carousel/".to_string() + file_name,
                    _ => {
                        let dest_dir = Path::new(unsafe { &BASE }).join(destination_fichier);
                        if !dest_dir.exists() {
                            fs::create_dir(dest_dir).unwrap();
                        }
                        destination_fichier.to_string() + "/" + file_name
                    }
                };
                let dest_path = Path::new( unsafe { &BASE }).join(&dest_name);
                let result = copy(&path, &dest_path);
                if result.is_ok() {
                    return Json(Resultat {resultat: "OK".to_string(), fichier: dest_name});
                }
            }
            _ => {}
        }
    }
    return Json(Resultat {resultat: "KO".to_string(), fichier: "".to_string()});
}
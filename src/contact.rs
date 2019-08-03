use rocket_contrib::json::Json;
use serde_json::Value;
use std::io;
use std::path::Path;
use crate::BASE;
use std::fs::File;
use std::io::BufReader;
use crate::tools::Administrateur;
use lettre_email::Email;
use lettre::{SmtpClient, Transport, ClientSecurity};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ConnectionReuseParameters;

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    nom: String,
    telephone: String,
    email: String,
    creneau: String,
    message: String
}

#[get("/accueil/contact")]
pub fn get_accueil_contact() -> Result<Json<Value>, io::Error> {
    let path = Path::new(unsafe { &BASE }).join("contenu/accueil/contact.json");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let reponse : Value = serde_json::from_reader(reader)?;
    Ok(Json(reponse))
}

#[post("/accueil/message", data = "<contact>")]
pub fn post_accueil_message(_key: Administrateur, contact: Json<Contact>) -> Result<Json<String>, lettre::smtp::error::Error> {

    let texte = format!("{}\nTéléphone : {}\nMail: {}\n Rendez-vous demandé: {}\n{}", contact.nom, contact.telephone, contact.email,
        if contact.creneau.is_empty() {"aucun".to_string()} else { contact.creneau.to_string() }, contact.message);
    let email = Email::builder()
        .to("jmarzin@gmail.com")
        .from(String::from(&contact.email))
        .subject("Prise de contact")
        .text(texte)
        .build().unwrap().into();
    let mut mailer = SmtpClient::new(("SSL0.OVH.NET", 587), ClientSecurity::None).unwrap()
        // Add credentials for authentication
        .credentials(Credentials::new("contact@milenioconcept.fr".to_string(), "mimilove632".to_string()))
        // Configure expected authentication mechanism
        .authentication_mechanism(Mechanism::Plain)
        // Disable connection reuse
        .connection_reuse(ConnectionReuseParameters::NoReuse).transport();
    mailer.send(email)?;
    mailer.close();
    Ok(Json("Le message a bien été envoyé".to_string()))
}
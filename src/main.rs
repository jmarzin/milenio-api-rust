#![feature(proc_macro_hygiene, decl_macro)]
#![feature(const_string_new)]

#[macro_use] extern crate rocket;
extern crate rocket_multipart_form_data;

extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

extern crate lettre;
extern crate lettre_email;
extern crate rocket_cors;

use std::fs;
use rocket_contrib::serve::StaticFiles;
use std::path::Path;

mod carousel;
mod panneaux;
mod contact;
mod realisations;
mod cuisines;
mod rangements;
mod salles_de_bain;
mod a_propos;
mod tools;

static mut BASE: String = String::new();

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let base_path = Path::new("base.param");
    if base_path.exists() {
        unsafe { BASE = fs::read_to_string("base.param")?.parse()? };
    } else {
        unsafe { BASE = "/home/deploy/milenio/milenioapi/public".to_string()}
    }
    let cors = rocket_cors::CorsOptions {
        ..Default::default()
    }
        .to_cors()?;

    rocket::ignite().mount("/",
                           routes![tools::index,
                                          tools::sensitive,
                                          tools::photo_upload,
                                          carousel::get_accueil_carousel,
                                          carousel::post_accueil_carousel,
                                          panneaux::get_accueil_panneaux,
                                          contact::get_accueil_contact,
                                          contact::post_accueil_message,
                                          realisations::get_realisations,
                                          cuisines::get_services_cuisines,
                                          rangements::get_services_rangements,
                                          salles_de_bain::get_services_salles_de_bain,
                                          a_propos::get_a_propos])
                    .mount("/contenu", StaticFiles::from( Path::new(unsafe { &BASE }).join("contenu")))
                    .attach(cors)
                    .launch();
    Ok(())
}

use lazy_static::lazy_static;
use std::{io, process::exit};
use tera::{Context as TeraContext, Tera};

lazy_static! {
    pub static ref HULL_TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Hull: template parsing error(s): {}", e);
                exit(1)
            }
        };
        tera.autoescape_on(vec!["atom", "html", "xml"]);
        tera
    };
}

pub fn render(template: &str, ctx: &TeraContext) -> Result<String, io::Error> {
    match HULL_TEMPLATES.render(template, &ctx) {
        Ok(x) => Ok(x),
        Err(err) => {
            println!("Hull: template error: {:#?}", err);
            exit(1)
        }
    }
}

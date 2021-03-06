use lazy_static::lazy_static;
use serde::Serialize;
use std::{io, process::exit};
use tera::{Context as TeraContext, Tera};

lazy_static! {
    pub static ref HULL_TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Hull: template parsing error(s): {}", e);
                exit(1)
            }
        }
    };
}

pub fn render<A: Serialize>(template: &str, data: Vec<(&str, A)>) -> Result<String, io::Error> {
    let mut ctx = TeraContext::new();

    for datum in data {
        ctx.insert(datum.0, &datum.1);
    }

    match HULL_TEMPLATES.render(template, &ctx) {
        Ok(x) => Ok(x),
        Err(err) => {
            println!("Hull: template error: {:#?}", err);
            exit(1)
        }
    }
}

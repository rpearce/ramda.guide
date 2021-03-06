use minify_html::{with_friendly_error, Cfg, FriendlyError};
use std::{io, process::exit, str::from_utf8};

pub fn html(html: String) -> Result<String, io::Error> {
    let cfg = &Cfg {
        minify_css: false,
        minify_js: false,
    };

    let mut input = html.as_bytes().to_vec();

    match with_friendly_error(&mut input, cfg) {
        Ok(len) => match from_utf8(&input) {
            Ok(res) => Ok(res[..len].to_string()),
            Err(err) => {
                println!(
                    "Hull failed to convert minified HTML bytes to string: {}",
                    err
                );
                exit(1)
            }
        },
        Err(FriendlyError {
            position,
            message,
            code_context,
        }) => {
            println!(
                "Hull failed to minify HTML at {}: {}\n{}",
                position, message, code_context
            );
            exit(1)
        }
    }
}

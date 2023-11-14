use actix_web::Error;
use serde::Deserialize;

use crate::classes::err::create_err;

pub fn convo_json<'a, T: Deserialize<'a>>(test: &'a str) -> Result<T, Error> {
    let temp_body = serde_json::from_str::<T>(test);

    let body = match temp_body {
        Ok(gut) => gut,
        Err(er) => match er.classify() {
            serde_json::error::Category::Syntax => {
                println!("{}", er);
                let err = create_err(String::from("Missing data"), 400);
                return Err(err.into());
            }

            _ => {
                let nice = er.to_string();
                let index_2: Vec<(usize, char)> = nice
                    .chars()
                    .enumerate()
                    .filter(|(_, e)| e == &'`')
                    .collect();

                let missing_field = {
                    let i1 = index_2[0].0 + 1;
                    let i2 = index_2[1].0;
                    &nice[i1..i2]
                };

                let err = create_err(
                    String::from("Missing field: [") + missing_field + "] from message",
                    400,
                );
                return Err(err.into());
            }
        },
    };
    Ok(body)
}

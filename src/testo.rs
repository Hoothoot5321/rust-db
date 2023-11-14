use serde_json::Value;

pub fn testo() -> String {
    let damn = Value::String("Nice".to_owned());

    let fisk = match damn.as_str() {
        Some(nice) => nice,
        None => "",
    };

    println!("{}", fisk);

    "".to_owned()
}

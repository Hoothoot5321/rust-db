use std::fs;

use actix_web::Error;

fn get_base_path(start_path: &str) -> String {
    String::from("./app/") + start_path + "/"
}

fn file_to_string(start_path: &str, file_name: &str) -> String {
    let file_path = get_base_path(start_path) + file_name;

    let file_content = fs::read_to_string(file_path).unwrap();

    file_content
}

fn get_js(dir_path: &str) -> String {
    let dir = fs::read_dir(dir_path).unwrap();

    let mut js = String::new();

    for temp_file in dir {
        let file = temp_file.unwrap();

        let content = fs::read_to_string(file.path()).unwrap();
        js = js + &content + "\n";
    }
    js
}

pub fn get_frontend(path: &str) -> Result<String, Error> {
    let html_content = file_to_string(path, "index.html");

    let style_content = file_to_string(path, "app.css");

    let js_content = get_js(&(get_base_path(path) + "js"));

    let mut new_content = html_content.replace("/*@css*/", &style_content);

    new_content = new_content.replace("//js", &js_content);

    Ok(new_content)
}

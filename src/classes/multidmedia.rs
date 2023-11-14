use std::io::Cursor;

#[allow(dead_code)]
pub struct MultiMediaEncoder {}

#[allow(dead_code)]
impl MultiMediaEncoder {
    pub fn encode_img(img_name: &str) -> String {
        let img = image::open(String::from("./") + &img_name).unwrap();

        let temp_name = img_name.replace("./", "");

        let dot_index = match temp_name.find(".") {
            Some(nice) => nice,
            None => 0,
        };

        let mut buf: Vec<u8> = vec![];

        img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
            .unwrap();

        let img_name_length = temp_name.len();

        let img_type = &temp_name[dot_index..img_name_length];

        let coverted = base64::encode(buf);

        let base64_img =
            String::from("data:image/") + &img_type.replace(".", "") + ";base64, " + &coverted;

        base64_img
    }

    pub fn decode_base64_image(base64_str: String) {
        let pure_base64 = base64_str.replace("data:image/png;base64, ", "");
        let nice = base64::decode(pure_base64).unwrap();

        match image::load_from_memory_with_format(&nice, image::ImageFormat::Png) {
            Ok(_) => std::fs::write("output.png", nice).unwrap(),
            Err(_) => (),
        }
    }
}

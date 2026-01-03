use std::fs::File;
use std::path::Path;
use std::io::{
    self,
    Write
};



fn create_html_file(name: &str) -> io::Result<File> {

    let file_name: String = format!("{name}.html");

    let mut file = File::create(file_name)?;

    write!(file, "<!DOCTYPE html>\n\n");
    write!(file, "<html>\n\n");

    return Ok(file);


} 


fn create_html_head(html_file: &File, html_name: &str) {

}


fn create_html_body(html_file: &File) {

}

pub fn convert_to_html(file_path: &str) {
    let path = Path::new(&file_path);
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

    let mut html_file = create_html_file(&file_name.as_str()).unwrap();

    create_html_head(&html_file, &file_name);

    


    write!(html_file, "</html>");
}
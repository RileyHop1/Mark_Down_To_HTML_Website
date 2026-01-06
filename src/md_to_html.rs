use std::fs::File;
use std::path::Path;
use std::io::{
    self,
    Write
};

enum TextState {
    NORMAL,
    BOLD,
    ITALICS,
    CODE, 

}

fn create_html_file(name: &str) -> io::Result<File> {

    let file_name: String = format!("{name}.html");

    let mut file = File::create(file_name)?;

    write!(file, "<!DOCTYPE html>\n\n").expect("Could not write to file");
    write!(file, "<html>\n\n").expect("Could not write to file");

    return Ok(file);


} 


/// Add more meta data as needed.
fn create_html_head(html_file: &mut File, html_name: &str) {
    write!(html_file, "<head>\n").expect("Could not write to file");

    let title_tag: String = format!("<title>{html_name}</title>\n");

    write!(html_file,"{}" ,title_tag.as_str()).expect("Could not write to file");
    write!(html_file, "</head>\n").expect("Could not write to file");
}


fn create_html_body(html_file: &mut File, md_file_path: &str) {

    write!(html_file, "<body>\n").expect("Could not write to file");

    let mut contents = std::fs::read_to_string(md_file_path).unwrap();

    //Controls the state of the text we are reading.
    let mut text_state: TextState = TextState::NORMAL;


    //This will iterate through each line and do some stuff.
    for line in contents.lines() {
        let line_start: char = line.chars().next().expect("Couldn't read first character");

    }


    write!(html_file, "</body>\n").expect("Could not write to file");
    





}

pub fn convert_to_html(file_path: &str) {
    let path = Path::new(&file_path);
    let file_name = path.file_stem().unwrap().to_string_lossy().to_string();

    let mut html_file = create_html_file(&file_name.as_str()).unwrap();

    create_html_head(&mut html_file, &file_name);
    create_html_body(&mut html_file, &file_path);

    


    write!(html_file, "</html>").expect("Could not write to file");
}
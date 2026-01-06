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

enum BlockType {

    PARAGRAPH,
    LIST,
    HEADING,
    HORIZONTALRULE,
}

struct Block {
    block_type: BlockType,
    key_item: Vec<char>,
    text: String,
}


impl Block {
    pub fn new(line: &str) -> Block{
        let line_start: char = line.chars().next().expect("Couldn't read first character");

        match line_start {
            '#' => {

            },
            '-' => {

            },
            '0'..='9' => {

            },
            _=> { 

            }
        }

        return Block {
            block_type: BlockType::PARAGRAPH,
            key_item: Vec::new(),
            text: String::new(),
        }; 
    }
}
fn handle_header(header: &str, html_file: &mut File) {

}

fn handle_horizontal_rule(html_file: &mut File) {

}

fn handle_ordered_list(text: &str, html_file: &mut File) {

}

fn handle_unordered_list(text: &str, html_file: &mut File) {

}

fn handle_bold_text(text: &str, html_file: &mut File) {

}

fn handle_italics_text(text: &str, html_file: &mut File) {

}

fn handle_normal_text(text: &str, html_file: &mut File) {

}

fn handle_link(title: &str, link: &str, html_file: &mut File) {

} 

fn handle_image(alt_text: &str, image_path: &str, html_file: &mut File) {

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
        let body= Block::new(&line);

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
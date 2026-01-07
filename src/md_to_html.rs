use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
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
    block_type: BlockType, // This is the type of block.
    lines: Vec<String>, //This is all of the lines, induvidually that exist within a block.
    last_line: String, //This will keep track of the last line in the block so there isn't overlap.
    text: String, //This is the output.
}


impl Block {
    pub fn new(line: &str) -> Block {

    }

    ///This will figure out the type of block and what lines exist within it.
    ///Returning a tuple with the type and a vector of all the lines in the block.
    fn get_block_type(text: &str) -> (BlockType, vec<String>) {

    }

    ///This will take an inputed md vector and convert it to an html vector.
    fn convert_md_lines_to_html(lines: vec<String>) ->vec<String> {

    }

    fn get_block_as_html() -> String {

    }
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
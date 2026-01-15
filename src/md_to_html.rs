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
    block_type: BlockType, // This is the type of block.
    lines: Vec<String>, //This is all of the lines, induvidually that exist within a block.
    last_line: u32, //This will the last index that the iterator moved to.
    text: String, //This is the output.
}


//Working theory is to put each line into a vector, so we can keep better track
//Of the position within text we are at.
//This will allow us some flexiblity with how we move through the files.
impl Block {
    pub fn new(text: &Vec<&str>,starting_index: u32) -> Block {
        
        let type_and_vstring: (BlockType, Vec<String>) = get_block_type(&text, starting_index);

    }

    ///This will figure out the type of block and what lines exist within it.
    ///Returning a tuple with the type and a vector of all the lines in the block.
    fn get_block_type(text: &Vec<&str>,starting_index: usize) -> (BlockType, Vec<String>) {

        let mut current_index:usize = starting_index;

        //This will grab the current block type that we are within
        let current_block:BlockType = {

            let line: &str = text[starting_index];

            if line.starts_with("#") {

                let mut chars = line.chars();

                loop {
                    match chars.next() {
                        Some('#') => continue,            
                        Some(' ') => break BlockType::HEADING,   
                        Some(_) => break BlockType::PARAGRAPH,  
                        None => break BlockType::PARAGRAPH,     
                    }
                }

            } else if line.starts_with("- ") {

                BlockType::LIST
           } else if line == "---" {
               BlockType::HORIZONTALRULE
           } else {

            BlockType::PARAGRAPH
           }
        };
        
        //This will find how many lines complie with the block of the previous lines.
        loop {

            


        }
    }

    ///This will take an inputed md vector and convert it to an html vector.
    fn convert_md_lines_to_html(lines: Vec<String>) -> Vec<String> {

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
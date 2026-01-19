use core::num;
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

#[derive(PartialEq, Eq, Debug)]
enum BlockType {
    PARAGRAPH,
    LIST,
    HEADING,
    HORIZONTALRULE,
}

struct Block {
    block_type: BlockType, // This is the type of block.
    lines: Vec<MdLine>, //This is all of the lines, induvidually that exist within a block.
}


#[derive(PartialEq, Eq, Debug)]
enum LineType {
    PARAGRAPH,
    UNORDED_LIST,
    ORDERED_LIST(u32),
    HEADING(u32),
    HORIZONTALRULE,
}

struct MdLine {
    line: String,
    line_type: LineType,
}

//Working theory is to put each line into a vector, so we can keep better track
//Of the position within text we are at.
//This will allow us some flexiblity with how we move through the files.
impl Block {
    pub fn new(text: &Vec<&str>,starting_index: usize) -> Block {
        
        let type_and_vstring: (BlockType, Vec<MdLine>) = Block::get_block_type(&text, starting_index);

        return Block {
            block_type: type_and_vstring.0,
            lines: type_and_vstring.1
        }


    }

    fn line_type_to_block_type(the_type: &LineType) -> BlockType {
        match the_type {
            LineType::PARAGRAPH => BlockType::PARAGRAPH,

            LineType::UNORDED_LIST => BlockType::LIST,

            LineType::ORDERED_LIST(n) => BlockType::LIST,

            LineType::HEADING(level) => BlockType::HEADING,

            LineType::HORIZONTALRULE => BlockType::HORIZONTALRULE,
        }
    }

    fn line_block(line: &str) -> MdLine {


            if line.starts_with("#") {

                let mut chars = line.chars();
                let mut level: u32 = 0;
                let line_type = {
                    loop {
                        match chars.next() {
                            Some('#') => level += 1,            
                            Some(' ') => break LineType::HEADING(level),   
                            Some(_) => break LineType::PARAGRAPH,  
                            None => break LineType::PARAGRAPH,     
                        }
                    }

                };

                return match line_type {
                    LineType::HEADING(_) => {
                        // It's a heading, so we strip the prefix. 
                        // level + 1 accounts for the '#'s and the space.
                        let clean_text = line.chars().skip((level + 1) as usize).collect();
                        MdLine { 
                            line: clean_text, 
                            line_type 
                        }
                    }
                    _ => {
                        MdLine { 
                            line: line.to_string(), 
                            line_type: LineType::PARAGRAPH 
                        }
                    }
                };

            } else if line.chars().next()
                .map_or(false, |c| c.is_ascii_digit()) {

                let mut chars = line.chars();
                let mut level: u32 = 0;
                let mut has_dot= false;
                let line_type = {
                    loop {
                        match chars.next() {
                            Some(c) if c.is_ascii_digit() => {
                                if has_dot {
                                    break LineType::PARAGRAPH;
                                }

                                level += 1;
                            },            
                            Some('.') => {
                                if has_dot || level == 0{
                                    break LineType::PARAGRAPH;
                                }
                                has_dot = true;
                            },
                            Some(' ') => {

                                let num_str: String = line.chars().take(level as usize).collect();
                                let num = num_str.parse::<u32>().unwrap_or(1);
                                break LineType::ORDERED_LIST(num);
                            },   
                            Some(_) => break LineType::PARAGRAPH,  
                            None => break LineType::PARAGRAPH,     
                        }
                    }

                };

                return match line_type {
                    LineType::ORDERED_LIST(_) => {
                        // It's a heading, so we strip the prefix. 
                        // level + 1 accounts for the '#'s and the space.
                        let clean_text = line.chars().skip((level + 2) as usize).collect();
                        MdLine { 
                            line: clean_text, 
                            line_type 
                        }
                    }
                    _ => {
                        MdLine { 
                            line: line.to_string(), 
                            line_type: LineType::PARAGRAPH 
                        }
                    }
                };
            } else if line.starts_with("- ") {
                let clean_text = line.chars().skip(2).collect();
                MdLine { 
                    line: clean_text, 
                    line_type: LineType::UNORDED_LIST,
                }
           } else if line == "---" {
                MdLine { 
                    line: line.to_string(), 
                    line_type: LineType::HORIZONTALRULE,
                }
           } else {
                MdLine { 
                    line: line.to_string(), 
                    line_type: LineType::PARAGRAPH,
                }
           }
    }

    ///This will figure out the type of block and what lines exist within it.
    ///Returning a tuple with the type and a vector of all the lines in the block.
    fn get_block_type(text: &Vec<&str>,starting_index: usize) -> (BlockType, Vec<MdLine>) {

        let mut current_index:usize = starting_index;
        let mut lines_in_block:Vec<MdLine> = Vec::new();

        
        let first_line = Block::line_block(text[starting_index]);

        //This will grab the current block type that we are within
        let block:BlockType = Block::line_type_to_block_type(&first_line.line_type);

        lines_in_block.push(first_line);
        current_index = current_index + 1;
        //This will find how many lines complie with the block of the previous lines.
        loop {
            

            let next_line = match text.get(current_index).copied() {
                Some(line) => line,
                None => return (block, lines_in_block), 
            };

            let current_line = Block::line_block(next_line); 

            let current_block:BlockType =  Block::line_type_to_block_type(&current_line.line_type);
            
            if current_block != block {
                
                return (block, lines_in_block);
            }

            lines_in_block.push(current_line);
            current_index = current_index + 1;
            
        }

    }
}

fn render_block_to_html(html_file: &mut File, block_type: BlockType, md_block: Vec<MdLine>) {

    match block_type {
        BlockType::HEADING => {

            for head in md_block {
                if let LineType::HEADING(level)= head.line_type {

                    writeln!(html_file, "<h{}>{}</h{}>",level, head.line, level).unwrap();
                }
            }


        },
        BlockType::LIST => {
            let mut prev_tag = match md_block.get(0).unwrap().line_type {
                    LineType::ORDERED_LIST(n) => {

                        writeln!(html_file,"<ol start ={}>", n).unwrap();
                        "ol"
                    },
                    _=> {
                        writeln!(html_file,"<ul>").unwrap();
                        "ul"
                    }, //Should in theory always be one of the two options 
            };

            for item in md_block {
                let cur_tag = match item.line_type {
                    LineType::UNORDED_LIST => "ul",
                    LineType::ORDERED_LIST(_) => "ol",
                    _ => continue, // Should theoretically not happen
                };

                if prev_tag == cur_tag  {
                    match item.line_type {
                        LineType::ORDERED_LIST(_) => writeln!(html_file,"<li>{}</li>", item.line).unwrap(),   
                        _=> writeln!(html_file,"<li>{}</li>", item.line).unwrap(),
                    }
                } else {
                    writeln!(html_file,"</{}>", prev_tag).unwrap();
                    match item.line_type {
                        LineType::ORDERED_LIST(n) => {

                            writeln!(html_file,"<ol start ={}>", n).unwrap();
                        },
                        _=> {
                            writeln!(html_file,"<ul>").unwrap();
                        },  
                    };
                    match item.line_type {
                        LineType::ORDERED_LIST(_) => writeln!(html_file,"<li>{}</li>", item.line).unwrap(),   
                        _=> writeln!(html_file,"<li>{}</li>", item.line).unwrap(),
                    }
                    prev_tag = cur_tag;
                }

            }
            writeln!(html_file,"</{}>", prev_tag).unwrap();

        },
        BlockType::HORIZONTALRULE => {

            writeln!(html_file, "<hr />").unwrap();
        },
        BlockType::PARAGRAPH => {

            writeln!(html_file, "<p>").unwrap();
            for line in md_block {
                writeln!(html_file,"{}", line.line).unwrap();
            }
            writeln!(html_file, "</p>").unwrap();

        },

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

    let all_lines: Vec<&str> = contents.lines().collect();
    let mut current_index: usize = 0;


    //This will iterate through each line and do some stuff.
    while current_index < all_lines.len() {
        let body= Block::new(&all_lines, current_index);

        let md_block = body.lines;
        let block_type = body.block_type;
        current_index += md_block.len();

        render_block_to_html(html_file, block_type, md_block);



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
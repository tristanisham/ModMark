use std::io::Write;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use pulldown_cmark::{html, Options, Parser};

pub struct Config {
    pub conv_from: String,
    pub conv_to: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        //skips first arg which is file name
        args.next();

        let conv_from = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an input file name"),
        };
        let conv_to = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get an output file name"),
        };

        Ok(Config {
            conv_from,
            conv_to,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_file = fs::read_to_string(config.conv_from)?;
    let output_file: String = config.conv_to;
    
    let results = md_to_html(&input_file);
    
    if let Err(e) = writer(&results, output_file) {
        eprintln!("Appllication error: {}", e);

    }

    
    Ok(())
}


// MD converter
pub fn md_to_html<'a>(input_file: &'a str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input_file, options);

    let mut html_output: String = String::with_capacity(input_file.len() * 3 / 2);
    html::push_html(&mut html_output, parser);

    html_output
}

// Writes converted MD to output
pub fn writer(input: &String, output: String,) -> std::io::Result<()> {
    let mut file = File::create(output)?;
    let content = input.as_bytes();
    file.write_all(content)?;
    Ok(())
}



//Tests


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_str_and_back() {
            let input_file = "Hello world, this is a ~~complicated~~ *very simple* example.";


            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(input_file, options);
        
            let mut html_output: String = String::with_capacity(input_file.len() * 3 / 2);
            html::push_html(&mut html_output, parser);
        
            let string_conv = html_output.to_owned();
            let html_string_slice: &str = &string_conv[..];
        
            let results = vec![html_string_slice];
            assert_eq!(results[0], html_string_slice);
        
            println!("{:?}", results);
        
    }
    

}
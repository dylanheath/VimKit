use std::io;
use std::env;
use terminal_spinners::{SpinnerBuilder, DOTS};


//use and array to store urls


//colorschemes

const COLORSCHEMES: &'static [&'static str] =  &["PlaceHolder"];

//plugins
const PLUGINS: &'static [&'static str] = &["PlaceHolder"];

#[derive(Debug)]
struct repoUrl {
    url: String,
}

#[derive(Debug)]
struct customRepo {
    url: String,
    name: String,
    r#type: String,
}

#[derive(Debug)]
struct repoInfo {

        url:  String,
        name: String,
        r#type: String,        
}

impl repoUrl {
    fn checkLiteral() {


      
     
    }
}

fn custom() {

    let mut customInput = String::new();
    io::stdin().read_line(&mut customInput).expect("Could not read line");
    let customInput = customInput.trim();

    let customUrl =  repoUrl {

        url: customInput.to_string(),
    };



}


fn default() {

}

fn main() {
    
    println!("VimKit 0.0.1");
    
    

   


}

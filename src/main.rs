use std::io;
use std::env;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::process::Command;


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
    fn checkLiteral(&self) -> bool {

       if self.url.contains("http://github.com") {
           return true
        } else {
            return false
        }
     
    }

    fn checkRepo(&self) -> bool {

        //need to make url an api request to check status
        let output = Command::new("curl")
            .arg(&self.url)
            .output()
            .expect("Failed to execute process");

        //use to request data to determine status of repo
        let checkResult = String::from_utf8_lossy(&output.stdout);
        

        return true



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

use std::fmt::Error;
use std::io;
use std::env;
use std::ptr::null;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::process::Command;

//run command
//curl -fLo ~/.vim/autoload/plug.vim --create-dirs \
//https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim



//use and array to store urls


//colorschemes

const COLORSCHEMES: &'static [&'static str] =  &["morhetz/gruvbox"
, "Mangeshrex/uwu.vim", "joshdick/onedark.vim", "arcticicestudio/nord-vim",
"dracula/vim', { 'as': 'dracula' }" , "sainnhe/sonokai" , "sainnhe/gruvbox-material",
"sainnhe/everforest", "NLKNguyen/papercolor-theme"];


const PLUGINS: &'static [&'static str] = &["'preservim/nerdtree'" , "'easymotion/vim-easymotion'",
"'tpope/vim-fugitive'", "'neoclide/coc.nvim' {'branch': 'release'}", "'scrooloose/syntastic'",
"'turbio/bracey.vim'", "'xuyuanp/nerdtree-git-plugin'", "'neoclide/coc-snippets'"];

//add language specific plugins (will do this later on I cba)
//
const PYTHON_PLUGINS: &'static [&'static str] = &["'hdima/python-syntax'", ""];

const JAVA_PLUGINS: &'static [&'static str] = &["'neoclide/coc-java'" , ""];

const C_PLUGINS: &'static [&'static str] = &["PlaceHolder"];

const CPP_PLUGINS: &'static [&'static str] = &["PlaceHolder"];

const RUST_PLUGINS: &'static [&'static str] = &["PlaceHolder"];

const TYPESCRIPT_PLUGINS: &'static [&'static str] = &["'neoclide/coc-tslint'", "'neoclide/coc-tsserver'"];

const GO_PLUGINS: &'static [&'static str] = &["PlaceHolder"];

//configuration
const CONFIGURATION: &'static [&'static str] = &["syntax on","smartindent",
"noerrorbells", "nowrap", "backspace=2", "noswapfile", "smartcase", "nobackup"
, "incsearch", "nocompatible", "filetype plugin indent on", "syntax enable"];

#[derive(Debug)]
struct repoUrl {
    url: String,
    valid: bool,
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

    fn checkRepo(&self) -> Result<(), &str>{


        if self.url.contains("https://github.com") {
                 let handle = SpinnerBuilder::new().spinner(&DOTS).text("    Checking Repository Status").start();  

                 let urlLength = *&self.url.len();
                 let urlToApi = &self.url[18..urlLength];

                 let apiUrl = ["https://api.github.com/repos", urlToApi].join("");

                 //need to make url an api request to check status
                 let output = Command::new("curl")
                                     .arg(apiUrl)
                                     .output()
                                     .expect("Failed to execute process");

                //use to request data to determine status of repo
                 let checkResult = String::from_utf8_lossy(&output.stdout);

                 if checkResult.contains("id") {
                     &self.valid == &true;
                 } else if  checkResult.contains("message") {
                     &self.valid == &false;
                     return Err("repository is either invalid or Private");
                     
                 }

                 std::thread::sleep(std::time::Duration::from_secs(3));
                 handle.done();
    

                 Ok(())
                
        } else {

            &self.valid == &false;
            return Err("Entered repository isn't Valid or is Private");


        }





    }

    fn getRepo(&self) {

        //pass through url display error/success message

    }
}

fn custom() {

    let mut customInput = String::new();
    io::stdin().read_line(&mut customInput).expect("Could not read line");
    let customInput = customInput.trim();
    let customUrl =  repoUrl {

        url: customInput.to_string(),
        valid: false,
        
    };

    customUrl.checkRepo();

    if customUrl.valid == true {
        //getRepo function
    } else if customUrl.valid == false {
        custom()
    }


}


fn default() {

}

fn main() {
    
    println!("VimKit 0.0.1");
    
    

   


}

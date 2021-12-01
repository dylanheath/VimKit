use std::fmt::Error;
use std::io;
use std::os;
use std::env;
use std::path::PathBuf;
use std::ptr::null;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::process::Command;
use std::path::Path;
use directories::{BaseDirs, UserDirs, ProjectDirs};

extern crate dirs;

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
"'turbio/bracey.vim'", "'xuyuanp/nerdtree-git-plugin'", "'neoclide/coc-snippets'" , "'benmills/vimux'", 
"'christoomey/vim-tmux-navigator'",  "'yuttie/comfortable-motion.vim'"];

//add language specific plugins (will do this later on I cba)
//
const PYTHON_PLUGINS: &'static [&'static str] = &["'hdima/python-syntax'", "'davidhalter/jedi-vim'" ,
""];

const JAVA_PLUGINS: &'static [&'static str] = &["'neoclide/coc-java'" , ""];

const C_and_CPP_PLUGINS: &'static [&'static str] = &["'vim-scripts/c.vim'",  "'octol/vim-cpp-enhanced-highlight'"];

const RUST_PLUGINS: &'static [&'static str] = &["'neoclide/coc-rls'" , "'timonv/vim-cargo'"];

const TYPESCRIPT_PLUGINS: &'static [&'static str] = &["'neoclide/coc-tslint'", "'neoclide/coc-tsserver'"];

const GO_PLUGINS: &'static [&'static str] = &["'fatih/vim-go'"];

//configuration
const CONFIGURATION: &'static [&'static str] = &["syntax on","smartindent",
"noerrorbells", "nowrap", "backspace=2", "noswapfile", "smartcase", "nobackup"
, "incsearch", "nocompatible", "filetype plugin indent on", "syntax enable"];

//Plug variables
const PLUG: &'static [&'static str] = &["PlaceHolder"];

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


        //write to vimrc file "HOME" then PlugInstall or download packages then source % write to
        //vimrc

        //pass through url display error/success message

}

#[derive(Debug)]
struct vimrc {

    path: PathBuf,
    exists: bool,

}

impl vimrc {


    fn checkVimrc(&self) -> Result<() , &str> {

        if self.path.exists() {
            &self.exists == &true;
            Ok(())
        } else {
            &self.exists == &false;
            return Err("Couldn't find vimrc file")
            
        } 
        

    }
}





fn custom() {
    
    //allow multiple entries

    let mut customInput = String::new();
    io::stdin().read_line(&mut customInput).expect("Could not read line");
    let customInput = customInput.trim();
    let customUrl =  repoUrl {

        url: customInput.to_string(),
        valid: false,
        
    };

    customUrl.checkRepo();

    if customUrl.valid == true {
        //check if repo is valid
        //fron array write to vimrc file
    } else if customUrl.valid == false {
        custom()
    }


}


fn default() {




}





//be able to import and export config file / plugin list
//check for vimrc file, either create or edit one
//Run commands for package managers such as Vundle, Plug etc

fn main() {
    
    println!("VimKit 0.0.1");

    //random loading sentences
    let handle =  SpinnerBuilder::new().spinner(&DOTS).text("  ").start();

    //get current user
    let output = Command::new("echo")
                            .arg("$USER")
                            .output()
                            .expect("Failed to get current user");

    let currentUser = String::from_utf8_lossy(&output.stdout);

    let vimrcPath = ["/Users/", &currentUser, "/.vimrc"].join("");

    let path = Path::new(&vimrcPath);

    let currentPath = vimrc {
       path: PathBuf::from(path),
       exists: true,
    };
    
    currentPath.checkVimrc(); 


    let mut menu = String::new();
    io::stdin().read_line(&mut menu).expect("Failed to read line");
    let menu = menu.trim();




    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();
    
    

}

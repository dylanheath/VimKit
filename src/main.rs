use std::fmt::Error;
use std::io;
use std::os;
use std::env;
use std::fs;
use std::fs::File;
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

//make the arrays not static

//colorschemes

const COLORSCHEMES: &'static [&'static str] =  &["morhetz/gruvbox"
, "Mangeshrex/uwu.vim", "joshdick/onedark.vim", "arcticicestudio/nord-vim",
"dracula/vim', { 'as': 'dracula' }" , "sainnhe/sonokai" , "sainnhe/gruvbox-material",
"sainnhe/everforest", "NLKNguyen/papercolor-theme"];


const PLUGINS: &'static [&'static str] = &["preservim/nerdtree" , "easymotion/vim-easymotion",
"tpope/vim-fugitive", "neoclide/coc.nvim {'branch': 'release'}", "scrooloose/syntastic",
"turbio/bracey.vim", "xuyuanp/nerdtree-git-plugin", "neoclide/coc-snippets" , "benmills/vimux", 
"christoomey/vim-tmux-navigator",  "yuttie/comfortable-motion.vim"];

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

#[derive(Debug)]
struct defaultRepo {
    url: String,
    valid: bool,
}

impl defaultRepo{
    fn checkRepo(&self) -> Result<(), &str> {

        let urlLength = *&self.url.len();
        
        let apiUrl = ["https://api.github.com/repos/" , &self.url].join("");

        let output = Command::new("curl")
                                .arg(apiUrl)
                                .output()
                                .expect("Failed to execute command");

        let repoResult = String::from_utf8_lossy(&output.stdout);
        
        if repoResult.contains("id") {
            &self.valid == &true;
        } else if repoResult.contains("message") {
            &self.valid == &false;
            return Err("repository is either invalid or Private");
        } else {
            &self.valid == &false;
            return Err("Couldn't get repository information");
        }

        Ok(())


    }
}



fn custom(currentUser: vimrc) {
    
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
        custom(currentUser);
    }


}


fn default(currentPath: vimrc) {
//go through default plugins and check if they're public or deprecated
    
    let handle =  SpinnerBuilder::new().spinner(&DOTS).text("Writing to vimrc").start();

    let path = Path::new(&currentPath.path);

    let Configuration = vec!["syntax on","smartindent",
    "noerrorbells", "nowrap", "backspace=2", "noswapfile", "smartcase", "nobackup"
    , "incsearch", "nocompatible", "filetype plugin indent on", "syntax enable"];

    let mut RepoPlugin = vec!["preservim/nerdtree" , "easymotion/vim-easymotion",
    "tpope/vim-fugitive", "neoclide/coc.nvim {'branch': 'release'}", "scrooloose/syntastic",
    "turbio/bracey.vim", "xuyuanp/nerdtree-git-plugin", "neoclide/coc-snippets" , "benmills/vimux", 
    "christoomey/vim-tmux-navigator",  "yuttie/comfortable-motion.vim"];

    let mut RepoColorscheme = vec!["morhetz/gruvbox"
    , "Mangeshrex/uwu.vim", "joshdick/onedark.vim", "arcticicestudio/nord-vim",
    "dracula/vim', { 'as': 'dracula' }" , "sainnhe/sonokai" , "sainnhe/gruvbox-material",
    "sainnhe/everforest", "NLKNguyen/papercolor-theme"];



    for i in 0..Configuration.len() {
        fs::write(path, Configuration[i]).expect("Failed to write to vimrc");
    }
     
    //plug variables

    fs::write(path, "call plug#begin('~/.vim/plugged')");

    for i in 0..RepoPlugin.len() {

        let currentPlugin = defaultRepo {
            url: RepoPlugin[i].to_string(),
            valid: false,
        }; 
        currentPlugin.checkRepo();

        if currentPlugin.valid == true {
            //write to vimrc
            
        } else if currentPlugin.valid == false {
            RepoPlugin.retain(|&x| x != currentPlugin.url)

        }

    }

    for i in 0..RepoColorscheme.len() {
        let currentPlugin = defaultRepo {
            url: RepoColorscheme[i].to_string(),
            valid: false,
        };
        currentPlugin.checkRepo();

        if currentPlugin.valid == true {
            //write to vimrc
        
        } else if currentPlugin.valid == false {
            RepoColorscheme.retain(|&x| x != currentPlugin.url)
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(3));
    handle.done();

    //continue

}

fn menu(currentPath: vimrc ) {
    println!("[*] installation\n");
    println!("[1] default\n");
    println!("[2] custom");

    let mut menuInput = String::new();
    io::stdin().read_line(&mut menuInput).expect("Failed to get line");
    let menuInput =  menuInput.trim();

    if menuInput == "1" {
        default(currentPath);
    } else if menuInput == "2" {
        custom(currentPath);
    } else {
        menu(currentPath);
    }

}



//be able to import and export config file / plugin list
//check for vimrc file, either create or edit one
//Run commands for package managers such as Vundle, Plug etc

fn main() {
    
    println!("VimKit\n");

    //random loading sentences
    let handle =  SpinnerBuilder::new().spinner(&DOTS).text("Creating Vimrc").start();

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

    if currentPath.exists == true {
        let plug = Command::new("curl")
                                .arg("-fLo")
                                .arg("~/.vim/autoload/plug.vim")
                                .arg("--create-dirs")
                                .arg(r#"\"#)
                                .arg("https://raw.githubusercontent.com/junegunn/vim-plug/master/plug.vim")
                                .output()
                                .expect("Failed to download vim plug");
        
        let plug = ["/Users" , &currentUser , "/.vim/autoload/plug.vim"].join(""); 

        let plugPath  = Path::new(&plug);

        std::thread::sleep(std::time::Duration::from_secs(3));
        handle.done();

        if PathBuf::from(plugPath).exists() {

            menu(currentPath)

        } else {
            println!("Failed to download vim plug")
        }


        


    } else if currentPath.exists == false {
        println!("Failed to get Vimrc path");
    } 

}

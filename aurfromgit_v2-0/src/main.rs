use std::io;
use std::process;
use colored::Colorize;
use std::fs;

const VERSION: &str = "v2.0";

fn runbash(argument: String) -> process::ExitStatus{
    process::Command::new("sh").arg("-c").arg(argument).status().expect("Error occured while running a command.")
}

fn check_path(pathdir: String) -> bool {
    fs::metadata(pathdir).is_ok()
}

fn main(){
    let rcode: process::ExitStatus = runbash("pacman -Qq git".to_string());
    if !(rcode.code() == Some(0)){
        panic!("Git probably not installed on system, status code: {:?}", rcode);
    }
    let _cleancode: process::ExitStatus = runbash("clear".to_string());
    println!("\n\n");
    println!("{} {} \n", "AurFromGit".green().bold(), VERSION.blue());
    println!("Type in the {} package name you want to download\n", "AUR".bold().blue());
    let mut packagename: String = String::new();
    io::stdin().read_line(&mut packagename).expect("Could not take input!!!");
    let packagename = packagename.trim();
    runbash("clear".to_string());
    let readybash = format!("cd ~/ && git clone --branch {} --single-branch https://github.com/archlinux/aur.git {}", &packagename, &packagename);
    let clonecode: process::ExitStatus = runbash(readybash.to_string());
    if !(clonecode.code() == Some(0)){
        panic!("could not download {} package, {} {:?}", "AUR".bold().blue(), "Error code:".bold().red(), clonecode);
    }
    println!("\n\nWould you like to build package immediately using {}\n[y/n]", "makepkg -si".bold().bright_yellow());
    let mut agreement: String = String::new();
    io::stdin().read_line(&mut agreement).expect("Could not take input!!!");
    let agreement = agreement.trim();
    if &agreement.to_lowercase() == "y" || &agreement.to_lowercase() == "yes" {
        let homepath: String = std::env::var("HOME").expect("Could not fetch home directory!!!");
        let probablepath: String = format!("{}/{}", homepath, &packagename);
        if check_path(probablepath) == true {
            let buildcode: process::ExitStatus = runbash(format!("cd ~/ && cd {} && makepkg -si", &packagename));
            if !(buildcode.code() == Some(0)){
                panic!("{} {:?}", "Could not build package!!! Error status code:".bold().red(), buildcode);
            }
        }
        else {
            println!("Could not guess probably directory to build from, manual building required, though {} is successfully installed in {} directory", &packagename.bright_green(), "home".bright_yellow());
        }
    }
    else {
        println!("\n\n{} installed {} to {} directory! Note that you'll need to build package yourself.", "Successfully".bright_green(), &packagename, "home".bright_yellow());
    }
    process::exit(0);
}
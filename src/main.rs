use std::io;
use std::process;
use colored::Colorize;
use std::fs;
use std::collections::HashMap;

fn runbash(argument: String) -> process::ExitStatus{
    process::Command::new("sh").arg("-c").arg(argument).status().expect("Error occured while running a command.")
}

fn safebashinstall(package_argument: &String) -> process::ExitStatus{
    let hmvar = std::env::var("HOME").unwrap();
    process::Command::new("git")
        .current_dir(hmvar)
        .arg("clone")
        .arg("--branch")
        .arg(package_argument.to_string())
        .arg("--single-branch")
        .arg("https://github.com/archlinux/aur.git")
        .arg(package_argument.to_string())
        .status()
        .expect("Could not execute a command")
}

fn emergencystop(code: String){
    let _cln = runbash("clear".to_string());
    println!("\n\n Critical error: {}", code);
    process::exit(1);
}

fn check_path(pathdir: String) -> bool {
    fs::metadata(pathdir).is_ok()
}

fn buildbash(buildname: String) -> bool{
    let homedir: String = std::env::var("HOME").unwrap();
    let execpath: String = format!("{}/{}", homedir, &buildname);
    if check_path(execpath.clone()) == true {
        let runbuild: process::ExitStatus = process::Command::new("makepkg").arg("-si").current_dir(execpath).status().expect("Error occured while building a package");
        if !(runbuild.code() == Some(0)) {
            false
        }else {
            true
        }
    }else {
        false
    }
}


fn main(){
    let rcode: process::ExitStatus = runbash("pacman -Q git".to_string());
    if !(rcode.code() == Some(0)){
       emergencystop("Git is not installed".to_string());
    }
    let afg_args: Vec<String> = std::env::args().collect();
    let sz = afg_args.len();
    if sz < 2 || afg_args[1] == "--help" || afg_args[1] == "-h" {
        println!("Usage: aurfromgit <package1> [package2 ...]");
        process::exit(0);
    }
    if sz > 2 as usize {
        let mut resultdata: HashMap<String, u8> = HashMap::new();
        let mut userchoice: String = String::new();
        println!("\n\n Do you want to automatically build those packages after cloning them? {} \n [y/n]\n", "(by trying to use makepkg -si)".bold().bright_yellow());
        io::stdin().read_line(&mut userchoice).expect("Could not read user input!!!");
        let userchoice: &str = userchoice.trim();
        if &userchoice.to_lowercase() == "y" || &userchoice.to_lowercase() == "yes" {
            for i in 1..sz {
                let pkgarg: &String = &afg_args[i];
                let _cleancode: process::ExitStatus = runbash("clear".to_string());
                println!("\n\n Installing {}... - {}/{}", pkgarg.bright_green(), i, (sz - 1));
                let cloneproccess: process::ExitStatus = safebashinstall(pkgarg);
                if !(cloneproccess.code() == Some(0)){
                    resultdata.insert(pkgarg.to_string(), 2);
                    continue;
                }
                let success_build_code: bool = buildbash(pkgarg.to_string());
                if success_build_code == true {
                    resultdata.insert(pkgarg.to_string(), 0);
                }
                else {
                    resultdata.insert(pkgarg.to_string(), 1);
                }
            }
            let _cleancode: process::ExitStatus = runbash("clear".to_string());
            println!("\n\n Result of installing {} packages:\n", (sz - 1));
            for (key, value) in &resultdata {
                match *value {
                    0 => println!(" * {} - successfully cloned!", key.bright_green()),
                    1 => println!(" * {} - could not build, would require manual building", key.yellow()),
                    2 => println!(" * {} - could not clone, package folder probably exists in /home", key.bright_red()),
                    _ => unimplemented!()
                };
            }
            process::exit(0);
        }
        else {
            for i in 1..sz {
                let pkgarg: &String = &afg_args[i];
                let _cleancode: process::ExitStatus = runbash("clear".to_string());
                println!("\n\n Installing {}... - no building - {}/{}", pkgarg.bright_green(), i, (sz - 1));
                let cloneproccess: process::ExitStatus = safebashinstall(pkgarg);
                if !(cloneproccess.code() == Some(0)){
                    resultdata.insert(pkgarg.to_string(), 2);
                    continue;
                } else {
                    resultdata.insert(pkgarg.to_string(), 0);
                }
            }
            let _clrcode: process::ExitStatus = runbash("clear".to_string());
            println!("\n\n The result of cloning {} packages:\n", (sz - 1));
            for (key, value) in &resultdata {
                match *value {
                    0 => println!(" * {} - successfully cloned!", key.bright_green()),
                    1 => println!(" * {} - could not build, would require manual building", key.yellow()),
                    2 => println!(" * {} - could not clone, package folder probably exists in /home", key.bright_red()),
                    _ => unimplemented!()
                };
            }
            process::exit(0);
        }
    }
    else {
        let pkgname: &String = &afg_args[1];
        let _cleancode: process::ExitStatus = runbash("clear".to_string());
        println!("\n\n Installing {}...", pkgname.green());
        let cloneproccess: process::ExitStatus = safebashinstall(pkgname);
        if !(cloneproccess.code() == Some(0)){
            emergencystop(format!("Could not successfully clone {} package.", pkgname.red()));
        }
        let mut userchoice: String = String::new();
        println!("Do you want to build package immediately after cloning it? {}\n[y/n]\n", "(by trying to use makepkg -si)".bold().bright_yellow());
        io::stdin().read_line(&mut userchoice).expect("Could not read input!!!");
        let userchoice: &str = userchoice.trim();
        if &userchoice.to_lowercase() == "y" || &userchoice.to_lowercase() == "yes" {
            let buildcode: bool = buildbash(pkgname.to_string());
            if buildcode == true {
                println!("Result of installing {} - {}", pkgname, "Success".bright_green());
            }
            else {
                println!("Result of installing {} - {}", pkgname, "Couln't start building, package cloned".yellow());
            }
            process::exit(0);
        }
        else{
            println!("Successfully cloned {} - would require manual building.", pkgname.bright_green());
            process::exit(0);
        }
    }
}
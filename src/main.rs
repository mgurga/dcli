use clap::{Arg, App};
use std::{fs::File, process};
use std::io::BufReader;
use std::io::prelude::*;
use serenity::client::validate_token;
extern crate dirs;

fn main() -> std::io::Result<()>{
    let matches = App::new("dcli")
        .version("0.0.1")
        .author("mgurga")
        .about("Recieve discord messages from the cli")
        .arg(Arg::new("verbose")
             .short('v')
             .long("verbose")
             .about("show verbose output")
             .takes_value(false))
        .arg(Arg::new("token")
             .short('t')
             .long("token")
             .value_name("TOKEN")
             .about("discord authontication token, if not defined tokenfile will be used")
             .takes_value(true)
             .required(false))
        .arg(Arg::new("tokenfile")
             .short('T')
             .long("tokenfile")
             .takes_value(true)
             .value_name("PATH")
             .default_value(&*format!("{}/dcli/token", dirs::config_dir().unwrap().display()))
             .about("token file location, token file should only contain the token, nothing else"))
        .arg(Arg::new("server")
             .short('s')
             .long("server")
             .value_name("NUMBER")
             .about("number from list of joined servers")
             .default_value("all")
             .takes_value(true)
             .required(false))
        .arg(Arg::new("channel")
             .short('c')
             .long("channel")
             .value_name("NUMBER")
             .about("number from list of server channels")
             .default_value("all")
             .takes_value(true)
             .required(false))
        .subcommand(App::new("listservers")
                    .about("lists all servers with numbers"))
        .subcommand(App::new("listchannels")
                    .about("lists all channels from server number")
                    .arg(Arg::new("server")
                         .about("server to list channels from")
                         .takes_value(false)
                         .required(true)))
        .get_matches();
    let mut token = String::new();

    if matches.is_present("listservers") {
        if matches.is_present("verbose") {
            println!("listing servers...")
        }
    }

    if matches.is_present("listchannels") {
        if let Some(serverid) = matches.subcommand_matches("listchannels") {
            if matches.is_present("verbose") {
                println!("listing channels from server number {}...", serverid.value_of("server").unwrap());
            }
        } else {
            println!("server number is not detected, the command should look like this:");
            println!("dcli -t <TOKEN> listchannels 1");
            process::exit(1);
        }
    }

    if matches.is_present("token") {
        if matches.is_present("verbose") {
            println!("using token from cli");
        }
        token = String::from(matches.value_of("token").unwrap());
    } else {
        if matches.is_present("verbose") {
            println!("getting token from tokenpath");
            println!("tokenpath is {}", matches.value_of("tokenfile").unwrap())
        }
        
        let tokenfile = File::open(matches.value_of("tokenfile").unwrap())?;
        let mut bufreader = BufReader::new(tokenfile);
        bufreader.read_to_string(&mut token)?;
    }
    
    if matches.is_present("verbose") {
        // println!("got token: {}", token);
    }

    if !validate_token(token).is_ok() {
        println!("token is not valid");
        process::exit(1);
    }

    Ok(())
}

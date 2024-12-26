//use std::process::Output;

use clap::{command, Arg, Command};
use sicxe_asm::assembler::{assemble, assemble_parallel};

fn main() {
    let matche_result = command!()
        .subcommand(
            Command::new("assemble")
                .about("Assemble the source code")
                .arg(Arg::new("source").help("source file").required(true))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("output filename"),
                ),
        )
        .subcommand(
            Command::new("optimize")
                .about("Optimize the object code")
                .arg(Arg::new("object").help("object file").required(true))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("output filename"),
                ),
        )
        .subcommand(
            Command::new("dir")
                .about("Optimize all object files in the directory")
                .arg(Arg::new("dir").short('d').long("dir").help("directory"))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("output filename"),
                ),
        )
        .subcommand(
            Command::new("parallel")
                .about("parallel assemble (experimental)")
                .arg(Arg::new("source").help("source file").required(true))
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("FILE")
                        .help("output filename"),
                ),
        )
        .get_matches();

    if let Some(matches) = matche_result.subcommand_matches("assemble") {
        assemble_command(matches);
    } else if let Some(matches) = matche_result.subcommand_matches("optimize") {
        optimize_command(matches);
    } else if let Some(matches) = matche_result.subcommand_matches("dir") {
        dir_command(matches);
    }else if let Some(matches) = matche_result.subcommand_matches("parallel") {
        parallel_command(matches);
    }
}

fn assemble_command(matches: &clap::ArgMatches) {
    let source = matches.get_one::<String>("source").unwrap();
    let source = std::fs::read_to_string(source).expect("Failed to read the file");
    let obj = assemble(&source);

    if let Err(e) = obj {
        println!("{}", e);
        return;
    }

    if let Some(output) = matches.get_one::<String>("output") {
        std::fs::write(output, obj.unwrap()).expect("Failed to write the file");
    } else {
        println!("{}", obj.unwrap());
    }
}
fn optimize_command(matches: &clap::ArgMatches) {
    let object = matches.get_one::<String>("object").unwrap();
    let object = std::fs::read_to_string(object).expect("Failed to read the file");
    let obj = assemble(&object);

    if let Err(e) = obj {
        println!("{}", e);
        return;
    }

    if let Some(output) = matches.get_one::<String>("output") {
        std::fs::write(output, obj.unwrap()).expect("Failed to write the file");
    } else {
        println!("{}", obj.unwrap());
    }
}
fn dir_command(matches: &clap::ArgMatches) {
    let dir = matches.get_one::<String>("dir").unwrap();
    let output = matches.get_one::<String>("output").unwrap();
    let files = std::fs::read_dir(dir).expect("Failed to read the directory");
    let mut objs = Vec::new();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        if path.is_file() {
            let obj = std::fs::read_to_string(path).expect("Failed to read the file");
            objs.push(obj);
        }
    }
    let objs = objs.join("\n");
    let obj = assemble(&objs);

    if let Err(e) = obj {
        println!("{}", e);
        return;
    }

    std::fs::write(output, obj.unwrap()).expect("Failed to write the file");
}
fn parallel_command(matches: &clap::ArgMatches) {
    let source = matches.get_one::<String>("source").unwrap();
    let source = std::fs::read_to_string(source).expect("Failed to read the file");
    let obj = assemble_parallel(&source);

    if let Err(e) = obj {
        println!("{}", e);
        return;
    }

    if let Some(output) = matches.get_one::<String>("output") {
        std::fs::write(output, obj.unwrap()).expect("Failed to write the file");
    } else {
        println!("{}", obj.unwrap());
    }
}
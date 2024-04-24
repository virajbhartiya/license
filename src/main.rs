use chrono::{Datelike, Utc};
use clap::{App, Arg, SubCommand};
use std::fs;
use std::process;

fn main() {
    let matches = App::new("license")
       .version("1.0")
       .about("Generate license files")
       .subcommand(
            SubCommand::with_name("install")
               .about("Install a license")
               .arg(
                    Arg::with_name("license")
                       .required(true)
                       .index(1)
                       .help("The license to install"),
                )
               .arg(
                    Arg::with_name("year")
                       .short('y')
                       .long("year")
                       .takes_value(true)
                       .help("The year to use. Example: 2024"),
                )
               .arg(
                    Arg::with_name("fullname")
                       .short('n')
                       .long("fullname")
                       .takes_value(true)
                       .help("Your fullname"),
                )
               .arg(
                    Arg::with_name("project")
                       .short('p')
                       .long("project")
                       .takes_value(true)
                       .help("Project name"),
                )
               .arg(
                    Arg::with_name("extension")
                       .short('e')
                       .long("extension")
                       .takes_value(true)
                       .help("The file extension for the license. Example: txt. Defaults to no extension"),
                ),
        )
       .subcommand(
            SubCommand::with_name("view")
               .about("View a license")
               .arg(
                    Arg::with_name("license")
                       .required(true)
                       .index(1)
                       .help("The license to view"),
                ),
        )
       .get_matches();

    match matches.subcommand() {
        Some(("install", install_matches)) => {
            let license: String = install_matches
                .value_of("license")
                .unwrap_or("")
                .to_lowercase()
                .to_string();
            let year: String = install_matches
                .value_of("year")
                .unwrap_or(&Utc::now().year().to_string())
                .to_string();
            let fullname: String = install_matches
                .value_of("fullname")
                .unwrap_or("")
                .to_string();
            let projectname: String = install_matches
                .value_of("project")
                .unwrap_or("")
                .to_string();
            let extension: String = install_matches
                .value_of("extension")
                .unwrap_or("")
                .to_string();

            let license_file = format!("licenses/{}.txt", license);
            let license_content = match fs::read_to_string(&license_file) {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Error reading license file {}: {}", &license_file, err);
                    process::exit(1);
                }
            };

            let result = license_content
                .replace("[year]", &year)
                .replace("[fullname]", &fullname)
                .replace("[project]", &projectname);

            let generated_license = format!(
                "LICENSE{}",
                if extension.is_empty() {
                    String::new()
                } else {
                    format!(".{}", extension)
                }
            );
            match fs::write(&generated_license, result) {
                Ok(_) => println!(
                    "Successfully added {} license to {} file.",
                    &license, &generated_license
                ),
                Err(err) => {
                    eprintln!("Error writing license file {}: {}", &generated_license, err);
                    process::exit(1);
                }
            }
        }
        Some(("view", view_matches)) => {
            let license: String = view_matches
                .value_of("license")
                .unwrap_or("")
                .to_lowercase();
            let license_file = format!("licenses/{}.txt", license);
            match fs::read_to_string(&license_file) {
                Ok(content) => println!("{}", content),
                Err(err) => {
                    eprintln!("Error reading license file {}: {}", &license_file, err);
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("No subcommand provided. Use 'install' or 'view'.");
            process::exit(1);
        }
    }
}

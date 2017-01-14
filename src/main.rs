extern crate clap;
extern crate env_logger;
extern crate guzuta;

fn main() {
    env_logger::init().unwrap();

    let app = clap::App::new("guzuta")
        .version("0.0.0")
        .about("Custom repository manager for ArchLinux pacman")
        .subcommand(clap::SubCommand::with_name("repo-add")
            .about("Add PACKAGE_PATH to DB_PATH")
            .arg(clap::Arg::with_name("repo-key")
                .long("repo-key")
                .help("GPG key to sign repository database"))
            .arg(clap::Arg::with_name("PACKAGE_PATH")
                .required(true)
                .help("Path to package to be added"))
            .arg(clap::Arg::with_name("DB_PATH")
                .required(true)
                .help("Path to repository database")))
        .subcommand(clap::SubCommand::with_name("repo-remove")
            .about("Remove PACKAGE_NAME to DB_PATH")
            .arg(clap::Arg::with_name("repo-key")
                .long("repo-key")
                .help("GPG key to sign repository database"))
            .arg(clap::Arg::with_name("PACKAGE_NAME")
                .required(true)
                .help("Path to package to be removed"))
            .arg(clap::Arg::with_name("DB_PATH")
                .required(true)
                .help("Path to repository database")))
        .subcommand(clap::SubCommand::with_name("files-add")
            .about("Add PACKAGE_PATH to FILES_PATH")
            .arg(clap::Arg::with_name("repo-key")
                .long("repo-key")
                .help("GPG key to sign repository database"))
            .arg(clap::Arg::with_name("PACKAGE_PATH")
                .required(true)
                .help("Path to package to be added"))
            .arg(clap::Arg::with_name("FILES_PATH")
                .required(true)
                .help("Path to repository database")));
    let matches = app.clone().get_matches();

    run_subcommand(matches.subcommand());
}

fn run_subcommand(subcommand: (&str, Option<&clap::ArgMatches>)) {
    match subcommand {
        ("repo-add", Some(repo_add_command)) => {
            repo_add(repo_add_command);
        }
        ("repo-remove", Some(repo_remove_command)) => {
            repo_remove(repo_remove_command);
        }
        ("files-add", Some(files_add_command)) => {
            files_add(files_add_command);
        }
        _ => {
            panic!("Unknown subcommand");
        }
    }
}

fn repo_add(args: &clap::ArgMatches) {
    let signer = args.value_of("repo-key").map(|key| guzuta::Signer::new(key.to_owned()));
    let package = guzuta::Package::load(&args.value_of("PACKAGE_PATH").unwrap());
    let mut repository = guzuta::Repository::new(args.value_of("DB_PATH").unwrap().to_owned(),
                                                 signer);

    repository.load();
    repository.add(&package);
    repository.save(false);
}

fn repo_remove(args: &clap::ArgMatches) {
    let signer = args.value_of("repo-key").map(|key| guzuta::Signer::new(key.to_owned()));
    let package_name = args.value_of("PACKAGE_NAME").unwrap();
    let mut repository = guzuta::Repository::new(args.value_of("DB_PATH").unwrap().to_owned(),
                                                 signer);

    repository.load();
    repository.remove(&package_name);
    repository.save(false);
}

fn files_add(args: &clap::ArgMatches) {
    let signer = args.value_of("repo-key").map(|key| guzuta::Signer::new(key.to_owned()));
    let package = guzuta::Package::load(&args.value_of("PACKAGE_PATH").unwrap());
    let mut repository = guzuta::Repository::new(args.value_of("FILES_PATH").unwrap().to_owned(),
                                                 signer);

    repository.load();
    repository.add(&package);
    repository.save(true);
}

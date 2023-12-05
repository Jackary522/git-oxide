use configparser::ini::Ini;
use std::fs;

fn default_ini() -> Ini {
    let mut config = Ini::new();
    config.set("core", "repositoryformatversion", Some("0".to_string()));
    config.set("core", "filemode", Some("true".to_string()));
    config.set("core", "bare", Some("false".to_string()));
    config.set("core", "ignorecase", Some("true".to_string()));
    config
}

pub fn initialize_git_dir() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();

    fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
    let config = default_ini();
    config.write(".git/config").unwrap();

    println!("Initialized git directory");
}

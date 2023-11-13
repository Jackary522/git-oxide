use clap::{Parser, Subcommand, Args};

mod rusty_git;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Kwargs {
    #[clap(subcommand)]
    command: CommandType,
}

#[derive(Debug, Args)]
pub struct UserInput {
    input: Vec<String>,
}

#[derive(Debug, Subcommand)]
enum CommandType {
    /// Add file contents to the index
    Add(UserInput),
    /// Provide content or type and size information for repository objects
    CatFile(UserInput),
    /// Will update HEAD to set the specified branch as the current branch
    Checkout(UserInput),
    /// Debug gitignore / exclude files
    CheckIgnore(UserInput),
    /// Record changes to the repository
    Commit(UserInput),
    /// Compute object ID and optionally create an object from a file
    HashObject(UserInput),
    /// Initialize a new, empty repository
    Init(UserInput),
    /// Show commit logs
    Log(UserInput),
    /// Show information about files in the index and the working tree
    LsFiles(UserInput),
    /// List the contents of a tree object
    LsTree(UserInput),
    /// Pick out and massage parameters
    RevParse(UserInput),
    /// Remove files from the working tree and from the index
    Rm(UserInput),
    /// List references in a local repository
    ShowRef(UserInput),
    /// Show the working tree status
    Status(UserInput),
    /// Create, list, delete or verify a tag object signed with GPG
    Tag(UserInput),
}
fn main() {
    let kwargs: Kwargs = Kwargs::parse();

    match kwargs.command {
        CommandType::Add(input) => rusty_git::cmd_add(input),
        CommandType::CatFile(_) => rusty_git::cmd_cat_file(),
        CommandType::Checkout(input) => rusty_git::cmd_checkout(input),
        CommandType::CheckIgnore(_) => rusty_git::cmd_check_ignore(),
        CommandType::Commit(_) => rusty_git::cmd_commit(),
        CommandType::HashObject(_) => rusty_git::cmd_hash_object(),
        CommandType::Init(input) => rusty_git::cmd_init(input),
        CommandType::Log(_) => rusty_git::cmd_log(),
        CommandType::LsFiles(_) => rusty_git::cmd_ls_files(),
        CommandType::LsTree(_) => rusty_git::cmd_ls_tree(),
        CommandType::RevParse(_) => rusty_git::cmd_rev_parse(),
        CommandType::Rm(_) => rusty_git::cmd_rm(),
        CommandType::ShowRef(_) => rusty_git::cmd_show_ref(),
        CommandType::Status(_) => rusty_git::cmd_status(),
        CommandType::Tag(_) => rusty_git::cmd_tag(),
    };
}

#![warn(clippy::pedantic)]

mod repo_files;
mod repo_init;
mod repo_obj;

use crate::repo_files::{cat_file, clone_repo, commit_tree, hash_object, ls_tree, write_tree};
use crate::repo_init::initialize_git_dir;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Commandline {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a Git directory
    // #[arg(short)]
    Init,
    /// Print the contents of a hashed file
    CatFile {
        /// The hash of the hashed file to be read
        #[arg(name = "hash")]
        hash: String,
    },
    /// Hash a Git object
    HashObject {
        /// The object to hash
        #[arg(name = "object")]
        object: String,
    },
    /// List a Git tree object
    LsTree {
        /// The hash of the Git tree object
        #[arg(name = "tree_hash")]
        tree_hash: String,
        /// Print only the tree leaf names
        #[arg(long, action)]
        name_only: bool,
    },
    /// Write a Git tree object
    WriteTree,
    /// Commit a Git tree object
    CommitTree {
        /// The Sha1 encoded string for a Git tree object
        #[arg(name = "tree_sha")]
        tree_sha: String,
        /// The Sha1 encoded string for a Git commit object
        #[arg(name = "commit_sha", short = 'p')]
        commit_sha: String,
        /// The message to accompany the commit
        #[arg(name = "message", short = 'm')]
        message: String,
    },
    /// Clone an existing repository
    Clone {
        /// The URL of the existing repository
        #[arg(name = "url")]
        url: String,
    },
}
fn main() {
    let args = Commandline::parse();
    match args.command {
        Some(command) => match command {
            Commands::Init => initialize_git_dir(),
            Commands::CatFile { hash } => cat_file(&hash),
            Commands::HashObject { object } => hash_object(&object, "blob"),
            Commands::LsTree {
                name_only,
                tree_hash,
            } => ls_tree(name_only, &tree_hash),
            Commands::WriteTree => write_tree(),
            Commands::CommitTree {
                tree_sha,
                commit_sha,
                message,
            } => commit_tree(&tree_sha, &commit_sha, &message),
            Commands::Clone { url } => clone_repo(&url),
        },
        None => println!("No command was given! Try again."),
    }
}

#![warn(clippy::pedantic)]

//! Main module for `rusty_git` CLI.
//!
//! This module serves as the entry point for the `rusty_git` command-line interface,
//! providing users with a range of commands to interact with Git-like repositories.
//! It leverages the `clap` crate for parsing command-line arguments and offers various
//! subcommands such as initializing a repository, hashing objects, printing contents of
//! a hashed file, listing tree objects, and cloning repositories.
//!
//! The module ties together functionalities from the `repo_commit`, `repo_files`, and
//! `repo_init` modules, allowing users to perform complex Git operations through simple
//! command-line commands. It's designed to be user-friendly and intuitive, making it
//! accessible for both beginners and experienced users who are familiar with Git.
//!
//! Commands:
//! - `Init`: Initialize a new Git directory.
//! - `CatFile`: Print the contents of a hashed file.
//! - `HashObject`: Hash a Git object and optionally write it to a file.
//! - `LsTree`: List the contents of a Git tree object, with an option to show only names.
//! - `WriteTree`: Write a Git tree object and print its hash.
//! - `CommitTree`: Commit a Git tree object with a message.
//! - `Clone`: Clone an existing Git repository.
//!
//! This module is crucial for the overall functionality of `rusty_git`, providing
//! an interface between the user and the Git-like system's backend operations.

mod lib {
    pub mod repo_commit;
    pub mod repo_files;
    pub mod repo_init;
    pub mod repo_objects;
}

use crate::lib::repo_commit::{clone_repo, commit_tree};
use crate::lib::repo_files::{cat_file, hash_object, ls_tree, print_tree};
use crate::lib::repo_init::initialize_git_dir;
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
        /// The hash to be read
        #[arg(name = "hash")]
        hash: String,
    },
    /// Hash a Git object either to console or file
    HashObject {
        /// The object to hash
        #[arg(name = "object")]
        object: String,
        /// Hash object to file
        #[arg(long, short, action)]
        write: bool,
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
            Commands::HashObject { object, write } => {
                hash_object(&object, lib::repo_objects::ObjectType::Blob, write);
            }
            Commands::LsTree {
                name_only,
                tree_hash,
            } => ls_tree(name_only, &tree_hash),
            Commands::WriteTree => print_tree(),
            Commands::CommitTree {
                // commit_sha,
                message,
                tree_sha,
            } => commit_tree(None, &message, &tree_sha),
            Commands::Clone { url } => clone_repo(&url),
        },
        None => println!("No command was given! Try again."),
    }
}

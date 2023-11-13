// use ordered_hash_map::OrderedHashMap;
// use chrono;
// use pwd_grp;
// use glob;
// use math::round::ceil;
// use regex::Regex;
#![allow(dead_code, unused_variables, unused_imports)]

use std::{io::{Write, Read}, path::PathBuf, fs};

use self::repo_init::{GitRepo, repo_file, repo_create};
use self::repo_obj::GitObject;
use super::UserInput;

pub mod repo_init;
pub mod repo_obj;


pub fn cmd_cat_file() {
    todo!();
}

pub fn cmd_check_ignore() {
    todo!();
}

pub fn cmd_checkout(args: UserInput) {
    todo!();
}

pub fn cmd_commit() {
    todo!()
}

pub fn cmd_hash_object() {
    todo!()
}

pub fn cmd_add(args: UserInput) {
    // TODO Make sure to clear this
    println!("{:?}", args);
    println!("Hello, World!");
    // repo_path(repo, path);
}

pub fn cmd_log() {
    todo!()
}

pub fn cmd_ls_files() {
    todo!()
}

pub fn cmd_ls_tree() {
    todo!()
}

pub fn cmd_rev_parse() {
    todo!()
}

pub fn cmd_rm() {
    todo!()
}

pub fn cmd_show_ref() {
    todo!()
}

pub fn cmd_status() {
    todo!()
}

pub fn cmd_tag() {
    todo!()
}

pub fn cmd_init(args: UserInput) {
    let path: String = if args.input.is_empty() {
        String::from(".")
    } else {
        args.input.join("/")
    };

    repo_create(path.as_str());
}
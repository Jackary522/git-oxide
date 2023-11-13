# Rust Git Client
## Overview
This Rust-based Git client offers a range of functionalities for interacting with Git repositories. It is designed to provide a comprehensive and efficient way to manage Git operations, taking advantage of Rust's performance and safety features.

### IMPORTANT NOTE: 
The sections of this project that currently are working is the git repository initialization and object instantiation. This alone will provide a scaffold that allows interoperability with git and allow git commands to be run against that repo. This is minimum functionality and more will be added soon.

## Features
__Repository Initialization:__ Initialize new Git repositories with necessary directory structure and default configuration.

__Git Objects Handling__: Read and write Git objects with support for compression and SHA1 hashing.

__File and Directory Management__: Functions to manage files and directories within a Git repository.

__User Commands__: A set of commands for various Git operations, currently placeholders for future implementation.

## Modules
`repo_init`: Handles initialization and configuration of Git repositories.

`repo_obj`: Defines the GitObject structure and methods for managing Git objects.

## Structs and Traits
`GitRepo`: Represents a Git repository with worktree, git directory, and configuration.

`GitObject`: Represents a Git object with associated data and format.

`GitObjMethods` (trait): Provides methods for initializing, serializing, and deserializing Git objects.

## Key Functions
`object_read`: Read a Git object from the repository.

`object_write`: Write a Git object to the repository, returning its SHA1 hash.

`repo_create`: Create a new repository at a specified path.

`repo_file`, `repo_path`, `repo_dir`: Utility functions for file and directory management within a Git repository.

## Dependencies
`flate2`: For compression and decompression of data.

`sha1`: For computing SHA1 hashes.

`configparser`: For handling configuration files.

`std::io`, `std::fs`, `std::path`: Standard library modules for file I/O and path management.

## Usage
The client is currently under development. The provided functions and structures form the core of the Git client, with user-facing commands to be implemented. Each function is marked with todo!() or a similar placeholder for future code.

## Disclaimer
This client is in the early stages of development and is not yet suitable for production use.
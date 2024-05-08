# git_oxide

## Introduction
Welcome to `git_oxide`, a Git-like version control system implemented in Rust. This project is designed to provide a simplified yet powerful interface for managing version control, leveraging the efficiency and safety of Rust. `git_oxide` is perfect for those who are familiar with Git and are looking for a system that incorporates Rust's modern features.

## Project Purpose and Goals
The main goal of `git_oxide` is to create a functional, lightweight version control system that mimics key functionalities of Git. Key features include:

- Initializing new repositories
- Creating and handling various Git objects like blobs, trees, and commits
- Compressing and decompressing Git objects
- Basic repository operations such as listing contents, creating tree objects, and cloning repositories

https://github.com/Jackary522/git-client-rust/assets/84044652/f2b77a1c-2f85-425b-a916-485380fbddbf

## Getting Started

### Prerequisites
- Rust and Cargo (latest stable version)
- Basic familiarity with Rust and command-line operations

### Building the Project
1. **Clone the Repository**: Start by cloning `git_oxide` to your local machine.
   ```
   git clone https://github.com/Jackary522/git-oxide.git
   cd git_oxide
   ```

2. **Build the Project**: Use Cargo to build the project.
   ```
   cargo build --release
   ```

3. **Run Tests**: (Optional) Run the tests to ensure everything is working as expected.
   ```
   cargo test
   ```

### Running the Project
After building, you can run `git_oxide` directly through Cargo:
```
cargo run -- [COMMAND]
```

Or as an executable:
```
./target/release/git_oxide [COMMAND]
```

Replace `[COMMAND]` with the desired operation (e.g., `init`, `cat-file`, `clone`). For a list of all commands and their descriptions, use:
```
cargo run -- --help
```

## License
This project is licensed under the [MIT License](LICENSE).

# OhtuVersionhallinta
This program is an exercise for the 2015 course Ohjelmistotuotanto (Software engineering) at the department of computer science in the University of Helsinki.

It is meant to be a proof-of-concept version control system, with extremely simplified operations. It is written in Rust, and acts also as a Rust exercise for me (it's my first Rust program).

# Dependencies
Building the project requires rustc and cargo. Stable rust 1.2 was used for development. You can get Rust from: https://www.rust-lang.org/

# Building
Enter the project directory and build the program using the command "cargo build --relese".

# Usage
ohtuv \<command>, where \<command> is one of:
* init : initializes the repository in the current directory. Creates a directory ".ohtuv" to hold the repository.
* find : finds the current repository path by walking up the directory tree.

/**
 * Most programs have dependencies on some libraries. If you have ever managed dependencies by hand, you know how much of a pain this can be. Luckily, the Rust ecosystem comes standard with cargo! cargo can manage dependencies for a project.
 */

// # A binary
// cargo new foo

// # OR A library
// cargo new --lib foo

/**
 * For the rest of this chapter, let's assume we are making a binary, rather than a library, but all of the concepts are the same.
 *
 * After the above commands, you should see a file hierarchy like this:
 */

// foo
// ├── Cargo.toml
// └── src
//     └── main.rs

/**
 * The main.rs is the root source file for your new project -- nothing new there. The Cargo.toml is the config file for cargo for this project (foo). If you look inside it, you should see something like this:
 */
// [package]
// name = "foo"
// version = "0.1.0"
// authors = ["mark"]

// [dependencies]



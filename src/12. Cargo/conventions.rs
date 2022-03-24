/**
 * Suppose that we wanted to have two binaries in the same project, though. What then?
 *
 * It turns out that cargo supports this. The default binary name is main, as we saw before, but you can add additional binaries by placing them in a bin/ directory:
 */

// foo
// ├── Cargo.toml
// └── src
//     ├── main.rs
//     └── bin
//         └── my_other_bin.rs

// cargo build --bin my_other_bin
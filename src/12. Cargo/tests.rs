// foo
// ├── Cargo.toml
// ├── src
// │   └── main.rs
// │   └── lib.rs
// └── tests
//     ├── my_test.rs
//     └── my_other_test.rs

// $ cargo test

// $ cargo test
//    Compiling blah v0.1.0 (file:///nobackup/blah)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.89 secs
//      Running target/debug/deps/blah-d3b32b97275ec472

// running 3 tests
// test test_bar ... ok
// test test_baz ... ok
// test test_foo_bar ... ok
// test test_foo ... ok

// test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out


#![allow(unused)]
fn main() {
    #[cfg(test)]
    mod tests {
        // Import the necessary modules
        use std::fs::OpenOptions;
        use std::io::Write;

        // This test writes to a file
        #[test]
        fn test_file() {
            // Opens the file ferris.txt or creates one if it doesn't exist.
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ferris.txt")
                .expect("Failed to open ferris.txt");

            // Print "Ferris" 5 times.
            for _ in 0..5 {
                file.write_all("Ferris\n".as_bytes())
                    .expect("Could not write to ferris.txt");
            }
        }

        // This test tries to write to the same file
        #[test]
        fn test_file_also() {
            // Opens the file ferris.txt or creates one if it doesn't exist.
            let mut file = OpenOptions::new()
                .append(true)
                .create(true)
                .open("ferris.txt")
                .expect("Failed to open ferris.txt");

            // Print "Corro" 5 times.
            for _ in 0..5 {
                file.write_all("Corro\n".as_bytes())
                    .expect("Could not write to ferris.txt");
            }
        }
    }
}

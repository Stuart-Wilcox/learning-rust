// [package]
// ...
// build = "build.rs"


/**
 * The build script is simply another Rust file that will be compiled and invoked prior to compiling anything else in the package. Hence it can be used to fulfill pre-requisites of your crate.
 * Cargo provides the script with inputs via environment variables specified here that can be used.
 * The script provides output via stdout. All lines printed are written to target/debug/build/<pkg>/output. Further, lines prefixed with cargo: will be interpreted by Cargo directly and hence can be used to define parameters for the package's compilation.
 * For further specification and examples have a read of the Cargo specification.
 */
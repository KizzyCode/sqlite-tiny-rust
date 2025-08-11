use cc::Build;

fn main() {
    // Rerun if one of the amalgamation files changed
    println!("cargo:rerun-if-changed=dist/sqlite3.c");
    println!("cargo:rerun-if-changed=dist/sqlite3.h");

    // Build SQLite
    Build::new()
        .extra_warnings(true)
        .warnings_into_errors(true)
        // SQLite has some function with unused parameters in some configurations
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("/wd4100")
        // Recommended flags; see https://www.sqlite.org/compile.html
        .flag("-DSQLITE_DQS=0")
        .flag("-DSQLITE_DEFAULT_MEMSTATUS=0")
        .flag("-DSQLITE_DEFAULT_WAL_SYNCHRONOUS=1")
        .flag("-DSQLITE_OMIT_DEPRECATED=1")
        .flag("-DSQLITE_OMIT_SHARED_CACHE=1")
        .flag("-DSQLITE_STRICT_SUBTYPE=1")
        // Build lib
        .include("dist/")
        .file("dist/sqlite3.c")
        .file("src/ffi/glue.c")
        .compile("libsqlite3.a");
}

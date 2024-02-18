use cc::Build;

fn main() {
    Build::new()
        .warnings_into_errors(true)
        // Recommended flags; see https://www.sqlite.org/compile.html
        .flag("-DSQLITE_DQS=0")
        .flag("-DSQLITE_DEFAULT_MEMSTATUS=0")
        .flag("-DSQLITE_DEFAULT_WAL_SYNCHRONOUS=1")
        .flag("-DSQLITE_OMIT_DEPRECATED")
        //.flag("-DSQLITE_OMIT_SHARED_CACHE") -- causes warnings if enabled
        .flag("-DSQLITE_STRICT_SUBTYPE=1")
        // Build lib
        .file("dist/sqlite3.c")
        .file("src/ffi/glue.c")
        .compile("libsqlite3.a");
}

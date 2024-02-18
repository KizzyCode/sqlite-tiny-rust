use cc::Build;

fn main() {
    Build::new()
        .extra_warnings(true)
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

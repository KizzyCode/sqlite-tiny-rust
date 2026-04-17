use cc::Build;

fn main() {
    // Rerun if one of the amalgamation files changed
    println!("cargo:rerun-if-changed=dist/sqlite3.c");
    println!("cargo:rerun-if-changed=dist/sqlite3.h");

    // Build SQLite
    let mut builder = Build::new();
    builder.extra_warnings(true);

    // SQLite causes some warnings in some configurations
    #[cfg(feature = "sqlite-warningsintoerrors")]
    builder.warnings_into_errors(true);

    // Recommended flags; see https://www.sqlite.org/compile.html
    builder.flag("-DSQLITE_DQS=0");
    builder.flag("-DSQLITE_DEFAULT_MEMSTATUS=0");
    builder.flag("-DSQLITE_DEFAULT_WAL_SYNCHRONOUS=1");
    builder.flag("-DSQLITE_OMIT_DEPRECATED=1");
    builder.flag("-DSQLITE_OMIT_SHARED_CACHE=1");
    builder.flag("-DSQLITE_STRICT_SUBTYPE=1");

    // Opinionated feature set
    builder.flag("-DSQLITE_ENABLE_API_ARMOR=1");
    builder.flag("-DSQLITE_ENABLE_FTS3=1");
    builder.flag("-DSQLITE_ENABLE_FTS3_PARENTHESIS=1");
    builder.flag("-DSQLITE_ENABLE_FTS4=1");
    builder.flag("-DSQLITE_ENABLE_FTS5=1");
    builder.flag("-DSQLITE_ENABLE_MATH_FUNCTIONS=1");
    builder.flag("-DSQLITE_ENABLE_ORDERED_SET_AGGREGATES=1");
    builder.flag("-DSQLITE_ENABLE_PERCENTILE=1");
    builder.flag("-DSQLITE_ENABLE_RTREE=1");
    builder.flag("-DSQLITE_ENABLE_UPDATE_DELETE_LIMIT=1");
    builder.flag("-DSQLITE_SOUNDEX=1");

    // Register source files
    builder.include("dist/");
    builder.file("dist/sqlite3.c");
    builder.file("src/ffi/glue.c");

    // Compile static library
    builder.compile("libsqlite3.a");
}

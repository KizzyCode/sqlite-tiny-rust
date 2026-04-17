# SQLite
This folder contains the unmodified [SQLite amalgamation 3.53.0 @2026-04-09](https://www.sqlite.org/download.html).

## Bindgen
To regenerate the bindings after an update, call bindgen from the project's root directory:
```sh
bindgen --no-layout-tests \
  --default-macro-constant-type=signed \
  --use-core \
  \
  --no-recursive-allowlist \
  --allowlist-type "sqlite.*" \
  --allowlist-var "sqlite.*" \
  --allowlist-var "SQLITE.*" \
  --allowlist-function "sqlite.*" \
  --blocklist-function "sqlite3_vmprintf" \
  --blocklist-function "sqlite3_vsnprintf" \
  --blocklist-function "sqlite3_str_vappendf" \
  \
  --output ./src/ffi/bindgen.rs ./dist/sqlite3.h
```

## Compile options
Currently, SQLite is compiled with the following recommended options:
- `-DSQLITE_DQS=0`
- `-DSQLITE_DEFAULT_MEMSTATUS=0`
- `-DSQLITE_DEFAULT_WAL_SYNCHRONOUS=1`
- `-DSQLITE_OMIT_DEPRECATED=1`
- `-DSQLITE_OMIT_SHARED_CACHE=1`
- `-DSQLITE_STRICT_SUBTYPE=1`

Furthermore, the following features are enabled on an opinionated base:
- `-DSQLITE_ENABLE_API_ARMOR=1`
- `-DSQLITE_ENABLE_FTS3=1`
- `-DSQLITE_ENABLE_FTS3_PARENTHESIS=1`
- `-DSQLITE_ENABLE_FTS4=1`
- `-DSQLITE_ENABLE_FTS5=1`
- `-DSQLITE_ENABLE_MATH_FUNCTIONS=1`
- `-DSQLITE_ENABLE_ORDERED_SET_AGGREGATES=1`
- `-DSQLITE_ENABLE_PERCENTILE=1`
- `-DSQLITE_ENABLE_RTREE=1`
- `-DSQLITE_ENABLE_UPDATE_DELETE_LIMIT=1`
- `-DSQLITE_SOUNDEX=1`

See <https://www.sqlite.org/compile.html> and [the `build.rs`](../build.rs) for further information.

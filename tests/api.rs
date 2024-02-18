#![cfg(feature = "api")]

use sqlite_tiny::Sqlite;

/// A schema for a basic test table
const SCHEMA: &str = "
    CREATE TABLE test (
        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        -- integers
        req_integer INTEGER NOT NULL DEFAULT 0,
        opt_integer INTEGER,
        -- reals
        req_real REAL NOT NULL DEFAULT 0.0,
        opt_real REAL,
        -- texts
        req_text TEXT NOT NULL DEFAULT '',
        opt_text TEXT,
        -- blobs
        req_blob BLOB NOT NULL DEFAULT X'',
        opt_blob BLOB
    );
";

#[test]
fn success() {
    // Create in-memory database
    let database = Sqlite::uri("file:test.db?mode=memory").expect("failed to open database");
    let mut statement = database.prepare(SCHEMA).expect("failed to prepare schema");

    // Initialize database with schema
    let result = statement.step().expect("failed to initialize test database");
    assert!(result.is_none());

    // Prepare row
    #[rustfmt::skip]
    let mut statement = database.prepare("
        INSERT INTO test (id, req_integer, opt_integer, req_real, opt_real, req_text, opt_text, req_blob, opt_blob)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    ").expect("failed to prepare write to test database");
    statement.bind(1, 1).expect("failed to bind value");
    statement.bind(2, 4).expect("failed to bind value");
    statement.bind(3, Some(7)).expect("failed to bind value");
    statement.bind(4, 4.4).expect("failed to bind value");
    statement.bind(5, Some(7.7)).expect("failed to bind value");
    statement.bind(6, "Testolope").expect("failed to bind value");
    statement.bind(7, Some("testolope")).expect("failed to bind value");
    statement.bind(8, b"TESTOLOPE".as_slice()).expect("failed to bind value");
    statement.bind(9, Some(b"tESTOLOPE".as_slice())).expect("failed to bind value");

    // Execute statement
    let result = statement.step().expect("failed to write to test database");
    assert!(result.is_none());

    // Read row back
    let mut statement =
        database.prepare("SELECT * FROM test WHERE id=1").expect("failed to prepare read from test database");
    let result = statement.step().expect("failed to read from test database").expect("missing expected result");

    // Validate values
    assert_eq!(result.read::<i32>(0).expect("failed to read ID"), 1);
    assert_eq!(result.read::<i32>(1).expect("failed to read required i32"), 4);
    assert_eq!(result.read::<Option<i32>>(2).expect("failed to read optional i32"), Some(7));
    assert_eq!(result.read::<f64>(3).expect("failed to read required f64"), 4.4);
    assert_eq!(result.read::<Option<f64>>(4).expect("failed to read optional f64"), Some(7.7));
    assert_eq!(result.read::<String>(5).expect("failed to read required string"), "Testolope");
    assert_eq!(
        result.read::<Option<String>>(6).expect("failed to read optional string"),
        Some("testolope".to_string())
    );
    assert_eq!(result.read::<Vec<u8>>(7).expect("failed to read required bytevec"), b"TESTOLOPE");
    assert_eq!(
        result.read::<Option<Vec<u8>>>(8).expect("failed to read optional bytevec"),
        Some(b"tESTOLOPE".to_vec())
    );
}

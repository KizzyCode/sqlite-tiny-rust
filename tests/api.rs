#![cfg(feature = "api")]

use sqlite_tiny::Sqlite;

/// A schema for a basic test table
const CREATE_TABLE: &str = "
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
    ) STRICT;
";

#[test]
fn success() {
    // Create in-memory database
    let database = Sqlite::uri("file:test.db?mode=memory").expect("failed to open database");
    let statement = database.query(CREATE_TABLE).expect("failed to prepare schema");

    // Initialize database with schema
    let result = statement.execute().expect("failed to initialize test database");
    assert!(result.is_none());

    // Insert row
    const INSERTY_QUERY: &str = r#"
        INSERT INTO test (id, req_integer, opt_integer, req_real, opt_real, req_text, opt_text, req_blob, opt_blob)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
    "#;
    let result = (database.query(INSERTY_QUERY))
        .and_then(|query| query.bind(1, 1))
        .and_then(|query| query.bind(2, 4))
        .and_then(|query| query.bind(3, Some(7)))
        .and_then(|query| query.bind(4, 4.4))
        .and_then(|query| query.bind(5, Some(7.7)))
        .and_then(|query| query.bind(6, "Testolope"))
        .and_then(|query| query.bind(7, Some("testolope")))
        .and_then(|query| query.bind(8, b"TESTOLOPE".as_slice()))
        .and_then(|query| query.bind(9, Some(b"tESTOLOPE".as_slice())))
        .and_then(|query| query.execute())
        .expect("failed to execute query");
    assert!(result.is_none());

    // Read row back
    const SELECT_QUERY: &str = "SELECT * FROM test WHERE id=1";
    let result = (database.query(SELECT_QUERY))
        .and_then(|query| query.execute())
        .expect("failed to read from test database")
        .expect("missing expected result");

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

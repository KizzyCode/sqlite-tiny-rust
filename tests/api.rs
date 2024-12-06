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
    let row = statement.execute().map(|result| result.row()).expect("failed to initialize test database");
    assert!(row.is_err());

    // Insert row
    const INSERT_QUERY: &str = r#"
        INSERT INTO test (req_integer, opt_integer, req_real, opt_real, req_text, opt_text, req_blob, opt_blob)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
    "#;
    let row = (database.query(INSERT_QUERY))
        .and_then(|query| query.bind(1, 4))
        .and_then(|query| query.bind(2, Some(7)))
        .and_then(|query| query.bind(3, 4.4))
        .and_then(|query| query.bind(4, Some(7.7)))
        .and_then(|query| query.bind(5, "Testolope"))
        .and_then(|query| query.bind(6, Some("testolope")))
        .and_then(|query| query.bind(7, b"TESTOLOPE".as_slice()))
        .and_then(|query| query.bind(8, Some(b"tESTOLOPE".as_slice())))
        .and_then(|query| query.execute())
        .map(|result| result.row())
        .expect("failed to execute query");
    assert!(row.is_err());

    // Read row back
    const SELECT_QUERY: &str = "SELECT * FROM test";
    let row = (database.query(SELECT_QUERY))
        .and_then(|query| query.execute())
        .map(|result| result.row())
        .expect("failed to read from test database")
        .expect("missing expected result");

    // Validate values
    assert_eq!(row.read::<i32>(0).expect("failed to read ID"), 1);
    assert_eq!(row.read::<i32>(1).expect("failed to read required i32"), 4);
    assert_eq!(row.read::<Option<i32>>(2).expect("failed to read optional i32"), Some(7));
    assert_eq!(row.read::<f64>(3).expect("failed to read required f64"), 4.4);
    assert_eq!(row.read::<Option<f64>>(4).expect("failed to read optional f64"), Some(7.7));
    assert_eq!(row.read::<String>(5).expect("failed to read required string"), "Testolope");
    assert_eq!(row.read::<Option<String>>(6).expect("failed to read optional string"), Some("testolope".to_string()));
    assert_eq!(row.read::<Vec<u8>>(7).expect("failed to read required bytevec"), b"TESTOLOPE");
    assert_eq!(row.read::<Option<Vec<u8>>>(8).expect("failed to read optional bytevec"), Some(b"tESTOLOPE".to_vec()));

    // Insert two other rows
    const INSERT_QUERY_2: &str = "INSERT INTO test (req_text) VALUES ('tESTOLOPE')";
    database.query(INSERT_QUERY_2).and_then(|query| query.execute()).expect("failed to execute query");
    database.query(INSERT_QUERY_2).and_then(|query| query.execute()).expect("failed to execute query");

    // Read rows back
    const SELECT_QUERY_2: &str = "SELECT * FROM test ORDER BY id ASC";
    let mut result =
        database.query(SELECT_QUERY_2).and_then(|query| query.execute()).expect("failed to read from test database");

    // Validate IDs
    let mut index = 1;
    while let Some(row) = result.next().expect("failed to read row from test database") {
        // Validate ID
        let id = row.read::<usize>(0).expect("failed to read ID");
        assert_eq!(id, index);
        index += 1;
    }

    // Batch execute statement
    const DROP_QUERY: &str = r#"
        BEGIN;
        DELETE FROM test WHERE req_integer = 4;
        DELETE FROM test WHERE req_text = 'tESTOLOPE';
        COMMIT;
    "#;
    database.exec(DROP_QUERY).expect("failed to drop tables");

    // Ensure database is empty
    const COUNT_QUERY: &str = "SELECT COUNT(id) FROM test";
    let row = (database.query(COUNT_QUERY))
        .and_then(|query| query.execute())
        .map(|result| result.row())
        .expect("failed to read from test database")
        .expect("missing expected result");
    assert_eq!(row.read::<i32>(0).expect("failed to read row count"), 0);
}

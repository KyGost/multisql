use crate::*;

test_case!(error, async move {
    run!("CREATE TABLE TableA (id INTEGER);");
    run!("INSERT INTO TableA (id) VALUES (1);");

    let test_cases = vec![
        (ExecuteError::QueryNotSupported.into(), "COMMIT;"),
        (
            ExecuteError::TableNotExists.into(),
            "INSERT INTO Nothing VALUES (1);",
        ),
        (
            ExecuteError::TableNotExists.into(),
            "UPDATE Nothing SET a = 1;",
        ),
        (
            FetchError::TableNotFound("Nothing".to_owned()).into(),
            "SELECT * FROM Nothing;",
        ),
        (
            TableError::TableFactorNotSupported.into(),
            "SELECT * FROM TableA JOIN (SELECT * FROM TableB) as TableC ON 1 = 1",
        ),
        /*(
            JoinError::UsingOnJoinNotSupported.into(),
            "SELECT * FROM TableA JOIN TableA USING (id);",
        ),*/
        (
            WIPError::TODO.into(), //EvaluateError::NestedSelectRowNotFound.into(),
            "SELECT * FROM TableA WHERE id = (SELECT id FROM TableA WHERE id = 2);",
        ),
        (
            WIPError::TODO.into(), //EvaluateError::ValueNotFound("noname".to_owned()).into(),
            "SELECT * FROM TableA WHERE noname = 1;",
        ),
        (
            InsertError::WrongNumberOfValues.into(),
            "INSERT INTO TableA (id2) VALUES (1);",
        ),
        (
            InsertError::WrongNumberOfValues.into(),
            "INSERT INTO TableA (id2, id) VALUES (100);",
        ),
        (
            InsertError::WrongNumberOfValues.into(),
            "INSERT INTO TableA VALUES (100), (100, 200);",
        ),
        (
            WIPError::TODO.into(), //LiteralError::UnsupportedLiteralType(r#"X'123'"#.to_owned()).into(),
            "SELECT * FROM TableA Where id = X'123';",
        ),
        #[cfg(feature = "alter-table")]
        (
            AlterError::UnsupportedAlterTableOperation(
                r#"ADD CONSTRAINT "hey" PRIMARY KEY (asdf)"#.to_owned(),
            )
            .into(),
            r#"ALTER TABLE Foo ADD CONSTRAINT "hey" PRIMARY KEY (asdf);"#,
        ),
    ];

    for (error, sql) in test_cases.into_iter() {
        test!(Err(error), sql);
    }
});

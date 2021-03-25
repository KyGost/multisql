#![cfg(feature = "auto-increment")]
use {
    super::{error::err_into, SledStorage},
    crate::{AutoIncrement, MutResult, Row, Value},
    async_trait::async_trait,
    fstrings::*,
    sqlparser::ast::ColumnDef,
};

macro_rules! try_into {
    ($self: expr, $expr: expr) => {
        match $expr {
            Err(e) => {
                return Err(($self, e));
            }
            Ok(v) => v,
        }
    };
}

#[async_trait(?Send)]
impl AutoIncrement for SledStorage {
    async fn generate_values(
        self,
        table_name: &str,
        columns: Vec<(usize, &ColumnDef)>,
        rows: Vec<Row>,
    ) -> MutResult<Self, Vec<Row>> {
        // FAIL: No-mut
        let mut storage = self;
        let mut rows = rows;
        for column in columns.iter() {
            let result = generate_column_values(storage, table_name, column, rows).await?;
            storage = result.0;
            rows = result.1;
        }
        Ok((storage, rows))
    }
}

async fn generate_column_values(
    storage: SledStorage,
    table_name: &str,
    column: &(usize, &ColumnDef),
    rows: Vec<Row>,
) -> MutResult<SledStorage, Vec<Row>> {
    const ONE: Value = Value::I64(1);
    let (column_index, column_name) = *column;
    let column_name = column_name.name.value.as_str();

    let first_value = try_into!(
        storage,
        storage
            .tree
            .get(f!("generator/{table_name}/{column_name}").as_bytes())
            .map_err(err_into)
    )
    .map(|value| bincode::deserialize(&value).ok())
    .flatten()
    .unwrap_or(ONE);

    // FAIL: No-mut
    let mut rows = rows;
    let mut value = first_value;
    for row_index in 0..rows.len() {
        rows[row_index].0[column_index] = value.clone();
        value = try_into!(storage, value.add(&ONE));
    }

    let last_value = try_into!(storage, bincode::serialize(&value).map_err(err_into));

    let key = f!("generator/{table_name}/{column_name}");
    let key = key.as_bytes();

    try_into!(
        storage,
        storage.tree.insert(key, last_value).map_err(err_into)
    );

    Ok((storage, rows))
}

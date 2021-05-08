use async_trait::async_trait;
use serde::Serialize;
use std::fmt::Debug;
use thiserror::Error;

use sqlparser::ast::ColumnDef;

use {super::StorageError, crate::Result};

#[derive(Error, Serialize, Debug, PartialEq)]
pub enum AlterTableError {
	#[error("Table not found: {0}")]
	TableNotFound(String),

	#[error("Renaming column not found")]
	RenamingColumnNotFound,

	#[error("Default value is required: {0}")]
	DefaultValueRequired(String),

	#[error("Adding column already exists: {0}")]
	AddingColumnAlreadyExists(String),

	#[error("Dropping column not found: {0}")]
	DroppingColumnNotFound(String),
}

#[async_trait(?Send)]
pub trait AlterTable {
	async fn rename_schema(&mut self, _table_name: &str, _new_table_name: &str) -> Result<()> {
		Err(StorageError::Unimplemented.into())
	}

	async fn rename_column(
		&mut self,
		_table_name: &str,
		_old_column_name: &str,
		_new_column_name: &str,
	) -> Result<()> {
		Err(StorageError::Unimplemented.into())
	}

	async fn add_column(&mut self, _table_name: &str, _column_def: &ColumnDef) -> Result<()> {
		Err(StorageError::Unimplemented.into())
	}

	async fn drop_column(
		&mut self,
		_table_name: &str,
		_column_name: &str,
		_if_exists: bool,
	) -> Result<()> {
		Err(StorageError::Unimplemented.into())
	}
}

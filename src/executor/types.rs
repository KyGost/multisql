use {crate::Value, serde::Serialize, std::fmt::Debug};

pub type Alias = Option<String>;
pub type Label = String;
pub type Row = Vec<Value>;
pub type LabelsAndRows = (Vec<Label>, Vec<Row>);
pub type ObjectName = Vec<String>;

#[derive(Debug, Clone)]
pub struct ColumnInfo {
	pub table: ComplexTableName,
	pub name: String,
	pub index: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ComplexTableName {
	pub database: String,
	pub alias: Alias,
	pub name: String,
}
impl ColumnInfo {
	pub fn of_name(name: String) -> Self {
		ColumnInfo {
			table: ComplexTableName {
				database: String::new(),
				name: String::new(),
				alias: None,
			},
			name,
			index: None,
		}
	}
}

impl PartialEq<ObjectName> for ColumnInfo {
	fn eq(&self, other: &ObjectName) -> bool {
		let mut other = other.clone();
		other.reverse();
		let names_eq = other
			.get(0)
			.map(|column| column == &self.name)
			.unwrap_or(false);
		let tables_eq = other
			.get(1)
			.map(|table| {
				table == &self.table.name
					|| self
						.table
						.alias
						.as_ref()
						.map(|alias| table == alias)
						.unwrap_or(false)
			})
			.unwrap_or(true);
		names_eq && tables_eq
	}
}

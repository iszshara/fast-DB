use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum DataType {
    Text(String),
    Integer(i32),
    UnsignedInteger(u64),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, Clone)]
enum ColumnType {
    Text,
    Integer,
    UnsignedInteger,
    Float,
    Bool,
}

enum DatabaseError {
    WrongDataType {
        column: String,
        expected: ColumnType,
        found: DataType,
    },
    ValueTooLarge,
    DuplicateEntry,
}

struct DbError {
    table: String,
    column: String,
    row: usize,
    value: DataType,
    kind: DatabaseError,
}

struct Table {
    pub name: String,
    pub fields: HashMap<String, ColumnType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
    pub row_count: usize,
}

impl Table {
    fn save_data(&mut self, data: HashMap<String, DataType>) -> Result<(), DbError> {
        let row = self.columns.values().next().map(|v| v.len()).unwrap_or(0);
        for (column_name, value) in &data {
            if let Some(expected) = self.fields.get(column_name) {
                match (expected, value) {
                    (ColumnType::Text, DataType::Text(_)) => {}
                    (ColumnType::UnsignedInteger, DataType::UnsignedInteger(_)) => {}
                    (ColumnType::Integer, DataType::Integer(_)) => {}
                    (ColumnType::Float, DataType::Float(_)) => {}
                    (ColumnType::Bool, DataType::Bool(_)) => {}
                    _ => {
                        return Err(DbError {
                            table: self.name.clone(),
                            column: column_name.clone(),
                            row: 0,
                            value: value.clone(),
                            kind: DatabaseError::WrongDataType {
                                column: column_name.clone(),
                                expected: expected.clone(),
                                found: value.clone(),
                            },
                        });
                    }
                }
                let duplicate = self.columns.values().all(|v| v.len() > row);
                if duplicate {
                    return Err(DbError {
                        table: self.name.clone(),
                        column: column_name.clone(),
                        row,
                        value: value.clone(),
                        kind: DatabaseError::DuplicateEntry,
                    });
                }
                let column = self.columns.entry(column_name.clone()).or_insert(vec![]);
                column.push(value.clone());
            } else {
                return Err(DbError {
                    table: self.name.clone(),
                    column: column_name.clone(),
                    row: 0,
                    value: value.clone(),
                    kind: DatabaseError::WrongDataType {
                        column: column_name.clone(),
                        expected: ColumnType::Text,
                        found: value.clone(),
                    },
                });
            }
        }
        Ok(())
    }
}

fn default_value_column_type(col_type: &ColumnType) -> DataType {
    match col_type {
        ColumnType::Bool => DataType::Bool(false),
        ColumnType::Float => DataType::Float(0.0),
        ColumnType::Integer => DataType::Integer(0),
        ColumnType::Text => DataType::Text("".to_string()),
        ColumnType::UnsignedInteger => DataType::UnsignedInteger(0),
    }
}
fn main() {}

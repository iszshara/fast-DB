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
}

impl Table {
    fn save_data(&mut self, data: HashMap<String, DataType>) -> Result<(), DbError> {
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

fn main() {}

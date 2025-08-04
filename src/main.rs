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
    WrongDataType,
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

fn main() {}

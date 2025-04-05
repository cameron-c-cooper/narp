use std::collections::HashMap;

mod ds;

#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    UInteger32(u32),
    Float32(f32),
}

// shouldn't be too many tables so just using String instead of str should be ok
pub struct Table {
    pub name: String,
    pub fields: HashMap<String, DataType>,
    pub columns: HashMap<String, Vec<DataType>>,
    pub select_columns: Vec<String>,
}

pub enum TableErrors {}

impl Table {
    pub fn save(&self, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors> {
        unimplemented!();
    }

    pub fn load(
        table_name: String,
        select_columns: Vec<String>,
        format: FileFormat,
    ) -> Result<Table, TableErrors> {
        unimplemented!()
    }
}

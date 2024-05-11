use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::{DataField, DataFieldDef, DataFieldError};

/// Holds a list of the fields found in a row.
#[derive(Debug)]
pub struct DataRow {
    fields: Vec<DataField>
}

/// Errors that DataRows may encounter.
pub enum DataRowError {
    /// A field-specific error (contains details).
    FieldError(DataFieldError),
    /// A row length is out of bounds.
    BadRowLength(usize),
    /// A field name was specified but not found.
    FieldNameNotFound(String)
}

/// Convenient Result shorthand for DataRowError results.
pub type Result<T> = std::result::Result<T, DataRowError>;

impl Display for DataRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataRowError::FieldError(fe) => fe.to_string(),
            DataRowError::BadRowLength(l) => format!("Bad Row Length ({})", l),
            DataRowError::FieldNameNotFound(n) => format!("Field Name Not Found ({})", n)
        };
        write!(f, "{}", s)
    }
}

impl Debug for DataRowError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Error for DataRowError {}

impl From<DataFieldError> for DataRowError {
    fn from(value: DataFieldError) -> Self {
        DataRowError::FieldError(value)
    }
}

impl DataRow {
    const MINIMUM_LENGTH: usize = 183; // todo: make this configurable

    /// Try to create a DataRow structure using the definitions provided.
    pub fn try_create(row: &str, row_defs: &Vec<DataFieldDef>) -> Result<DataRow> {
        if row.len() < Self::MINIMUM_LENGTH {
            return Err(DataRowError::BadRowLength(row.len()))
        }

        let tfs = DataField::try_from_row;
        let mut fields = Vec::new();

        for row_def in row_defs {
            fields.push(tfs(row, row_def)?);
        }

        Ok(DataRow {
            fields
        })
    }

    /// Get a copy of the row with specified fields in order. This is useful for constructing
    /// certain output formats, e.g., CSV.
    ///
    /// ```
    /// use ffreader::{DataRow, DataFieldDef, DataFieldResult};
    /// fn post_function(value: String) -> DataFieldResult<String> {
    ///     Ok(value) // does nothing; demo purposes only
    /// }
    /// let field_defs = vec![
    ///     DataFieldDef::new("field1", 0, 2, &post_function),
    ///     DataFieldDef::new("field2", 2, 4, &post_function),
    ///     DataFieldDef::new("field3", 4, 6, &post_function),
    ///     DataFieldDef::new("field4", 6, 8, &post_function)
    /// ];
    /// // todo: fix the padded length once MINIMUM_LENGTH is made configurable
    /// let raw_row = "abcdefgh...............................................................................................................................................................................";
    /// let row = DataRow::try_create(raw_row, &field_defs).unwrap();
    /// let fields = vec!["field3", "field1", "field2"];
    /// let ordered_row = row.get_ordered_fields(&fields).unwrap()
    ///                         .iter().map(|f| f.data())
    ///                         .collect::<Vec<String>>().join(" ");
    /// assert_eq!(ordered_row, "ef ab cd");
    /// ```
    pub fn get_ordered_fields(&self, field_list: &Vec<&str>) -> Result<Vec<DataField>>{
        let mut list = vec![];

        for f in field_list {
            if let Some(c) = self.fields.iter().find(|n| n.name() == f) {
                list.push((*c).clone());
            }
            else {
                return Err(DataRowError::FieldNameNotFound(f.to_string()));
            }
        }

        Ok(list)
    }

    /// Get a reference to the DataFields contained in the struct.
    pub fn fields(&self) -> &Vec<DataField> {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use crate::DataFieldResult;
    use super::*;

    fn test_row() -> String {
        String::from("5412345678  54     1   123 TEST PERSN            5412345678                           5412345678001  54    4    1  Z              5412345678           TEST AVE ABC              000075                      0-0001-111.000            0        R")
    }

    fn echo_ok(s: String) -> DataFieldResult<String> { Ok(s) }

    fn test_field_defs() -> Vec<DataFieldDef<'static>> {
        vec![
            DataFieldDef::new("AccountNo1", 0, 11, &echo_ok),
            DataFieldDef::new("CyclNo1", 11, 16, &echo_ok),
            DataFieldDef::new("Status", 16, 23, &echo_ok),
            DataFieldDef::new("Demo_Name", 23, 49, &echo_ok),
            DataFieldDef::new("DemoTestKey", 49, 60, &echo_ok),
            DataFieldDef::new("DemoLine2", 60, 86, &echo_ok),
            DataFieldDef::new("ThingID", 86, 100, &echo_ok),
            DataFieldDef::new("Cycle12", 100, 105, &echo_ok),
            DataFieldDef::new("DemoLength", 105, 110, &echo_ok),
            DataFieldDef::new("No", 110, 114, &echo_ok),
            DataFieldDef::new("Type", 114, 119, &echo_ok),
            DataFieldDef::new("TST", 119, 130, &echo_ok),
            DataFieldDef::new("TestKey", 130, 141, &echo_ok),
            DataFieldDef::new("StreetDirection", 141, 151, &echo_ok),
            DataFieldDef::new("StreetName", 151, 177, &echo_ok),
            DataFieldDef::new("StreetNumber", 177, 184, &echo_ok),
            DataFieldDef::new("StreetUnit", 184, 191, &echo_ok),
            DataFieldDef::new("ThingSerial", 191, 205, &echo_ok),
            DataFieldDef::new("Test_Key", 205, 231, &echo_ok),
            DataFieldDef::new("ThingSize", 231, 237, &echo_ok),
            DataFieldDef::new("Special", 237, 242, &echo_ok)
        ]
    }

    #[test]
    fn creation_extraction_works() {
        let row = test_row();
        let defs = test_field_defs();

        let datarow = DataRow::try_create(&row, &defs).unwrap();

        let fields = datarow.fields();

        assert_eq!(fields.iter().find(|s| s.name() == "ThingID").unwrap().data(), "5412345678001");
        assert_eq!(fields.iter().find(|s| s.name() == "AccountNo1").unwrap().data(), "5412345678");
        assert_eq!(fields.iter().find(|s| s.name() == "Special").unwrap().data(), "R");
        assert_eq!(fields.iter().find(|s| s.name() == "ThingSize").unwrap().data(), "0");
        assert_eq!(fields.iter().find(|s| s.name() == "Demo_Name").unwrap().data(), "123 TEST PERSN");
    }
}
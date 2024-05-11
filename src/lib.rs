#![warn(missing_docs)]
//! # ffreader
//! `ffreader` is a library built to facilitate processing of certain fixed-width
//! flat text files.
//!
//! It includes the following features:
//! - Field definitions based on column offset.
//! - Custom post-processing callbacks for each field.
//! - CSV and JSON output.
//!
//! Currently only ASCII text is supported.
//!
//! Usage is intended to be simple:
//! 1. Create any post-processing (validation and/or alteration) functions needed
//! 2. Create a `Vec` of `DataFieldDef` objects describing the fields and assigning post-processing functions
//! 3. Use `DataFile::try_load()` to open and process your file.
//! 4. Do what you need to with the data obtained. For example, you could obtain a subset of fields and
//! turn them into a CSV-formatted file using `DataFile::get_ordered_fields()`, use the rows directly
//! with `DataFile::rows()`, or check for problems with `DataFile::warnings()`.

mod datafield;
mod datarow;
mod datafile;
mod loadwarning;

pub use datafield::DataField;
pub use datafield::DataFieldDef;
pub use datafield::DataFieldError;
pub use datafield::Result as DataFieldResult;

pub use datarow::DataRow;
pub use datarow::DataRowError;
pub use datarow::Result as DataRowResult;

pub use datafile::DataFile;
pub use datafile::DataFileError;
pub use datafile::Result as DataFileResult;

pub use loadwarning::LoadWarning;

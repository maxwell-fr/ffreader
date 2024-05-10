# ffreader
A flat file reader utility library.

### Purpose
This library was built to parse flat text files produced by a certain older application. 

It is shared in the hopes that it might be useful elsewhere.

## Basics
An array of DataFieldDef objects is passed to DataFile::try_load() along with a path to a flat file to be processed.

DataFieldDef objects include where the field is as well as a processing function for validation or other utilty.

The resulting DataFile contains the data as well as a Vec of warnings encountered, if any.

See documentation for more details and examples.
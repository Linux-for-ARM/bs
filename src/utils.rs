//! A number of additional functions to simplify development

use crate::error::Error;
use std::fs;
use std::path::Path;

/// Reads the specified file into a `String`. Wrapper over [`std::fs::read_to_string()`].
pub fn read_to_string<P: AsRef<Path>>(pth: P) -> Result<String, Error> {
    fs::read_to_string(&pth)
        .map_err(|err| Error::ReadingError(pth.as_ref().display().to_string(), err.to_string()))
}

/// Writing the specified data to a file. Wrapper over [`std::fs::write()`].
pub fn write<P, C>(pth: P, cont: C) -> Result<(), Error>
where
    P: AsRef<Path>,
    C: AsRef<[u8]>,
{
    fs::write(&pth, &cont)
        .map_err(|err| Error::WritingError(pth.as_ref().display().to_string(), err.to_string()))
}

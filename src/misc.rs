use core::fmt;
use std::io;

pub fn write_comma_io<T>(mut file: T, comma: &mut bool) -> io::Result<()>
where T: io::Write {
    if *comma {
        file.write_all(b",")?;
    }
    *comma = true;
    Ok(())
}

pub fn write_comma_fmt<T>(mut file: T, comma: &mut bool) -> fmt::Result
where T: fmt::Write {
    if *comma {
        file.write_str(",")?;
    }
    *comma = true;
    Ok(())
}

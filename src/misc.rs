use core::fmt;
use std::io;

use arcstr::ArcStr;

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

pub type SmolStr = ArcStr;

pub mod base64 {
    use base64::Engine;
    use serde::{
        Deserialize,
        Deserializer,
        Serialize,
        Serializer,
    };

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = base64::engine::general_purpose::STANDARD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        base64::engine::general_purpose::STANDARD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

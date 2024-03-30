#[macro_export]
macro_rules! write_option {
    ($f:expr, $k:expr, $v:expr) => {{
        if let Some(v) = $v {
            write!($f, "{}: {}\n", $k, v)?;
        }
    }};
}

#[macro_export]
macro_rules! match_dict {
    ($value:expr, $expected:expr) => {{
        match $value {
            serde_bencode::value::Value::Dict(v) => Ok(v),
            serde_bencode::value::Value::List(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Seq,
                &$expected,
            )),
            serde_bencode::value::Value::Int(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Signed(v),
                &$expected,
            )),
            serde_bencode::value::Value::Bytes(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bytes(&v),
                &$expected,
            )),
        }
    }};
}

#[macro_export]
macro_rules! match_list {
    ($value:expr, $expected:expr) => {{
        match $value {
            serde_bencode::value::Value::List(v) => Ok(v),
            serde_bencode::value::Value::Dict(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &$expected,
            )),
            serde_bencode::value::Value::Int(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Signed(*v),
                &$expected,
            )),
            serde_bencode::value::Value::Bytes(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bytes(&v),
                &$expected,
            )),
        }
    }};
}

#[macro_export]
macro_rules! match_bytes {
    ($value:expr, $expected:expr) => {{
        match $value {
            serde_bencode::value::Value::Bytes(v) => Ok(v),
            serde_bencode::value::Value::Dict(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &$expected,
            )),
            serde_bencode::value::Value::Int(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Signed(*v),
                &$expected,
            )),
            serde_bencode::value::Value::List(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Seq,
                &$expected,
            )),
        }
    }};
}

#[macro_export]
macro_rules! match_int {
    ($value:expr, $expected:expr) => {{
        match $value {
            serde_bencode::value::Value::Int(v) => Ok(v),
            serde_bencode::value::Value::Dict(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Map,
                &$expected,
            )),
            serde_bencode::value::Value::Bytes(v) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Bytes(&v),
                &$expected,
            )),
            serde_bencode::value::Value::List(_) => Err(serde::de::Error::invalid_type(
                serde::de::Unexpected::Seq,
                &$expected,
            )),
        }
    }};
}

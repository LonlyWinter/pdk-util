//! PDK Macro

use std::{array::from_fn, fmt::Display, str::FromStr};

use crate::{PDKError, PDKResult};

pub fn lef_parse_base(line: &str) -> (&str, usize) {
    let line_now = line.trim_start();
    line_now
        .split_once(' ')
        .map(|(left, right)| (left.trim(), line.len() - right.len()))
        .unwrap_or_else(|| (line.trim(), line.len()))
}

pub fn lef_trim_value_end(value: &str) -> &str {
    let value = if let Some(idx) = value.rfind('#') {
        value[..idx].trim_end()
    } else {
        value
    };
    value.trim_end_matches(';').trim().trim_matches('"')
}

pub fn lef_parse_string(value: &str) -> PDKResult<String> {
    let value = lef_trim_value_end(value);
    Ok(value.to_string())
}

pub fn lef_parse_bool(value: &str) -> PDKResult<bool> {
    let value = lef_trim_value_end(value);
    let value = matches!(
        value,
        "ON" | "On" | "on" | "1" | "true" | "True" | "TRUE" | "Yes" | "Y"
    );
    Ok(value)
}

pub fn lef_parse_char(value: &str) -> PDKResult<char> {
    let value = lef_trim_value_end(value);
    value.chars().next().ok_or(PDKError::Empty("lef line"))
}

pub fn lef_parse_f32(value: &str) -> PDKResult<f32> {
    let value = lef_trim_value_end(value);
    let value = value.parse::<f32>()?;
    Ok(value)
}

pub fn lef_parse_u32(value: &str) -> PDKResult<u32> {
    let value = lef_trim_value_end(value);
    let value = value.parse::<u32>()?;
    Ok(value)
}

pub fn lef_parse_arr<const N: usize, E: Display, T: FromStr<Err = E> + Default + Copy>(
    value: &str,
) -> PDKResult<[T; N]> {
    let value = lef_trim_value_end(value);
    let mut res = [T::default(); N];
    for (idx, val) in value
        .split_whitespace()
        .filter(|&v| v.chars().next().map(|v| v.is_numeric()) == Some(true))
        .enumerate()
        .take(N)
    {
        // eprintln!("Parse arr: {:?}", val);
        res[idx] = val
            .parse::<T>()
            .map_err(|e| PDKError::Parse(e.to_string()))?;
    }
    Ok(res)
}

pub fn lef_parse_string_arr<const N: usize>(value: &str) -> PDKResult<[String; N]> {
    let value = lef_trim_value_end(value);
    let mut res = from_fn(|_| String::with_capacity(0));
    for (idx, val) in value.split_whitespace().enumerate().take(N) {
        res[idx] = val.to_owned();
    }
    Ok(res)
}

#[macro_export]
macro_rules! lef_macro {
    ("value", $pub:vis $name:ident, $($derive:meta,)*;
        // enum key
        $($key:ident = $pat:pat)*
    ) => {
        #[derive(Default, PartialEq, $($derive,)*)]
        $pub enum $name {
            #[default]
            Unknown,
            $(
                $key,
            )*
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::PDKError;
            fn try_from(value: &str) -> $crate::PDKResult<Self> {
                let value = $crate::lef::macros::lef_trim_value_end(value);
                match value {
                    $(
                        $pat => Ok(Self::$key),
                    )*
                    _ => Err($crate::PDKError::UnImplemented(format!("Unknown {}: {:?}", stringify!($name), value))),
                }
            }
        }

    };

    ("value_with", $pub:vis $name:ident, $($derive:meta,)*;
        // enum key with value
        $($key0:ident = $ty0:ty = $func:path = $pat0:pat)*,
        // enum key
        $($key:ident = $pat:pat)*
    ) => {
        #[derive(Default, PartialEq, $($derive,)*)]
        $pub enum $name {
            #[default]
            Unknown,
            $(
                $key,
            )*
            $(
                $key0($ty0),
            )*
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::PDKError;
            fn try_from(value: &str) -> $crate::PDKResult<Self> {
                let value = $crate::lef::macros::lef_trim_value_end(value);
                let (a, b) = value
                    .split_once(' ')
                    .map(| (a, b) | (a.trim(), b.trim()))
                    .unwrap_or_else(|| (value[value.len()..].trim(), value));
                match a {
                    $(
                        $pat => Ok(Self::$key),
                    )*
                    $(
                        $pat0 => {
                            let r = $func(b)?;
                            Ok(Self::$key0(r))
                        }
                    )*
                    _ => Err($crate::PDKError::UnImplemented(format!("Unknown {}: {:?}", stringify!($name), value))),
                }
            }
        }
    };

    (
        $pub:vis $name:ident, $($derive:meta,)*;
        // groups: key -> value_from_reader
        $($group:ident = $gvalue:ident = $gpat:pat)*,
        // arr: [value_from_reader]
        $($arr1:ident = $avalue1:ident = $apat1:pat)*,
        // arr: [value_from_str]
        $($arr2:ident = $avalue2:ty = $afunc2:path = $apat2:pat)*,
        // attr: key => value_from_func
        $($key:ident = $value:ty = $func:path = $pat:pat)*,
        // attr: key => value_from_reader
        $($key2:ident = $value2:ident = $pat2:pat)*
    ) => {
        #[derive(Default, PartialEq, $($derive,)*)]
        $pub struct $name {
            $(
                $pub $key: $value,
            )*
            $(
                $pub $key2: $value2,
            )*
            $(
                $pub $arr1: Vec<$avalue1>,
            )*
            $(
                $pub $arr2: Vec<$avalue2>,
            )*
            $(
                $pub $group: std::collections::HashMap<String, $gvalue>,
            )*
        }

        impl $name {
            $pub fn parse<F: std::io::BufRead + std::io::Seek>(reader: &mut F) -> $crate::PDKResult<Self> {
                let mut line = String::with_capacity(256);
                let mut res = Self::default();
                while let Ok(n) = reader.read_line(&mut line)
                    && n > 0
                {
                    if line.trim_start().starts_with("#") {
                        line.clear();
                        continue;
                    }
                    let (key, value_start) = $crate::lef::macros::lef_parse_base(&line);
                    let value = line[value_start..].trim();
                    // set key = value
                    // eprintln!("Key: {:?}, Value: {:?}", key, value);
                    match key {
                        $(
                            $pat => {
                                res.$key = $func(value)?;
                            },
                        )*
                        $(
                            $pat2 => {
                                res.$key2 = $value2::parse(reader)?;
                            },
                        )*
                        $(
                            $apat2 => {
                                let v = $afunc2(value)?;
                                res.$arr2.push(v);
                            },
                        )*
                        $(
                            $apat1 => {
                                let v = $avalue1::parse(reader)?;
                                res.$arr1.push(v);
                            },
                        )*
                        $(
                            $gpat => {
                                let v = $gvalue::parse(reader)?;
                                res.$group.insert(
                                    value.to_string(),
                                    v
                                );
                            },
                        )*
                        "" => {
                            // eprintln!("Continue: {}", stringify!($name));
                        },
                        "END" => {
                            // eprintln!("End: {}", stringify!($name));
                            break;
                        }
                        _ => {
                            // no match, seek old position
                            // eprintln!("No match: {:?}, {:?}", key, line);
                            reader.seek_relative(-(n as i64))?;
                            break;
                        }
                    }
                    line.clear();
                }
                $(
                    res.$arr1.shrink_to_fit();
                )*
                $(
                    res.$arr2.shrink_to_fit();
                )*
                $(
                    res.$group.shrink_to_fit();
                )*
                Ok(res)
            }
        }
    }
}

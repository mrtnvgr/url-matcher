use lazy_static::lazy_static;
use regex::Regex;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub trait FromPattern<T>
where
    T: DeserializeOwned,
{
    fn from_pattern(pattern: &str, url: &str) -> Result<T, Error> {
        let re = Regex::new(&make_expression(pattern))?;
        Ok(recap::from_captures::<T>(&re, url)?)
    }
}

pub fn matcher<'a>(pattern: &'a str, url: &'a str) -> Result<HashMap<String, &'a str>, Error> {
    let mut map = HashMap::new();

    let expr = make_expression(pattern);
    let re = Regex::new(&expr)?;

    let caps = re.captures(url).ok_or(Error::NoCaptures)?;

    for (index, key) in re.capture_names().enumerate() {
        if let (Some(k), Some(c)) = (key, caps.get(index)) {
            map.insert(k.to_owned(), c.as_str());
        }
    }

    Ok(map)
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to find any captures")]
    NoCaptures,
    #[error(transparent)]
    RegexError(#[from] regex::Error),
    #[error(transparent)]
    RecapError(#[from] recap::Error),
}

pub fn make_expression(pattern: &str) -> String {
    lazy_static! {
        static ref CHARS: Regex = Regex::new(r":(?P<key>[a-zA-Z0-9_.+-]+)").unwrap();
        static ref KEY: Regex = Regex::new(r"(?P<key>[?&.])").unwrap();
    }

    let escaped = KEY.replace_all(pattern, r"\$key");

    let mut exp = String::from(r"^");

    exp.push_str(&CHARS.replace_all(&escaped, r"(?P<$key>[a-zA-Z0-9_.+-]+)"));
    exp.push('$');

    exp
}

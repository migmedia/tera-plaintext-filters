//! Filters for the [Tera](https://github.com/Keats/tera) engine, useful for ascii-text file generation.
//!
//! To generate a Markdown-Table like this:
//! ```markdown
//!| No |      *Name*        |  Score |
//!|----|--------------------|--------|
//!| 1. |      Charly        |   3000 |
//!| 2. |     Alexander      |    800 |
//!| 3. |     Josephine      |    760 |
//!```
//!
//! A Tera-template could look like:
//! ```markdown
//! | No |       *Name*       |  Score |
//! |----|--------------------|--------|
//! {% for member in team | slice(end=10) %}
//! | {{ loop.index ~ '.' | left_align(length=4) }} | {{
//!     member.name | center(length=20) }} | {{
//!     member.score | right_align(length=10) }} |
//! {% endfor %}
//! ```
//!
use std::{collections::HashMap, hash::BuildHasher};
use tera::{to_value, try_get_value, Error, Value};

/// Right-aligns the token to a given length.   
///
/// # Usage in Tera-Templates
/// `{{ name | right_align(length=20) }}`
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_plaintext_filters::right_align;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("right_align", right_align);
///
/// let i = "{{ i | right_align(length=20) }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "           some text");
/// ```
pub fn right_align<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
) -> tera::Result<Value> {
    let (text, len) = eval_value(value, args, "right_align")?;
    Ok(to_value(format!("{text:>len$}")).unwrap())
}

/// Left-aligns the token to a given length.
///
/// # Usage in Tera-Templates
/// `{{ name | left_align(length=20) }}`
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_plaintext_filters::left_align;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("left_align", left_align);
///
/// let i = "{{ i | left_align(length=20) }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "some text           ");
/// ```
pub fn left_align<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
) -> tera::Result<Value> {
    let (text, len) = eval_value(value, args, "left_align")?;
    Ok(to_value(format!("{text:len$}")).unwrap())
}

/// Centers the token to a given length.   
///
/// # Usage in Tera-Templates
/// `{{ name | center(length=20) }}`
///
/// # Example
///
/// ```
/// use tera::{Context, Tera};
/// use tera_plaintext_filters::center;
///
/// let mut ctx = Context::new();
/// ctx.insert("i", "some text");
///
/// let mut tera = Tera::default();
/// tera.register_filter("center", center);
///
/// let i = "{{ i | center(length=20) }}";
/// let rendered = tera.render_str(i, &ctx).unwrap();
/// assert_eq!(rendered, "     some text      ");
/// ```
pub fn center<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
) -> tera::Result<Value> {
    let (text, len) = eval_value(value, args, "center")?;
    Ok(to_value(format!("{text:^len$}")).unwrap())
}

fn eval_value<S: BuildHasher>(
    value: &Value,
    args: &HashMap<String, Value, S>,
    filter_name: &'static str,
) -> tera::Result<(String, usize)> {
    if value.is_object() || value.is_array() {
        return Err(Error::msg(format!(
            "Filter `{filter_name}` was called on an incorrect value: got `{value}` \
                        but expected a text or number",
        )));
    }
    let len = match args.get("length") {
        Some(length) => {
            try_get_value!(filter_name, "length", usize, length)
        }
        None => {
            return Err(Error::msg(format!(
                "Filter `{filter_name}` expected an arg called `length`",
            )))
        }
    };
    Ok(match value.as_str() {
        Some(str) => (str.to_string(), len),
        // null => ""
        None if value.is_null() => (String::new(), len),
        None => (value.to_string(), len),
    })
}

#[cfg(test)]
mod should {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn check_alignment() {
        let v = json!("Shorttext");
        let mut hm = HashMap::new();
        let r = center(&v, &hm);
        assert!(r.is_err());

        hm.insert("length".to_string(), json!(20));
        let r = center(&v, &hm).unwrap();
        assert_eq!("     Shorttext      ", r.as_str().unwrap());
        let r = left_align(&v, &hm).unwrap();
        assert_eq!("Shorttext           ", r.as_str().unwrap());
        let r = right_align(&v, &hm).unwrap();
        assert_eq!("           Shorttext", r.as_str().unwrap());
    }

    #[test]
    fn check_input() {
        let v = json!("12.23");
        let mut hm = HashMap::new();
        hm.insert("length".to_string(), json!(20));
        let r = center(&v, &hm).unwrap();
        assert_eq!("       12.23        ", r.as_str().unwrap());
        let v = json!(12.23);
        let r = center(&v, &hm).unwrap();
        assert_eq!("       12.23        ", r.as_str().unwrap());
        assert_eq!("                    ", center(&json!(null), &hm).unwrap());

        assert!(center(&json!({ "a": "notice", "b": 124.0 }), &hm).is_err());
        assert!(center(&json!(["notice", "the", "trailing", "comma -->",]), &hm).is_err());
    }
}

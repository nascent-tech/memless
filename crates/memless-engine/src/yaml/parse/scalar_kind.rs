use memless_domain::document::RawStyle;
use serde_saphyr::granit_parser::{ScalarStyle, Tag};

pub(super) fn is_null(value: &str, style: ScalarStyle, tag: Option<&Tag>) -> bool {
    style == ScalarStyle::Plain && tag.is_none() && matches!(value, "~" | "null" | "Null" | "NULL")
}

pub(super) fn raw_style(style: ScalarStyle, tag: Option<&Tag>) -> RawStyle {
    if style == ScalarStyle::Plain && tag.is_none() {
        RawStyle::Plain
    } else {
        RawStyle::ExplicitText
    }
}

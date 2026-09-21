use super::ParseFailure;
use memless_domain::refusal::TextPosition;
use serde_saphyr::granit_parser::ScanError;

pub(super) fn scan_failure(error: ScanError) -> ParseFailure {
    let marker = error.marker();
    let at = TextPosition { line: marker.line(), column: marker.col() + 1 };
    ParseFailure { at: Some(at) }
}

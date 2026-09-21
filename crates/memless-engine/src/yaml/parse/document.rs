use super::builder::Builder;
use super::ParseFailure;

pub(super) fn on_document_start(builder: &mut Builder) -> Result<(), ParseFailure> {
    builder.documents += 1;
    if builder.documents > 1 {
        return Err(ParseFailure { at: None });
    }
    Ok(())
}

use super::attach::attach;
use super::builder::Builder;
use super::ParseFailure;

pub(super) fn on_alias(builder: &mut Builder, id: usize) -> Result<(), ParseFailure> {
    let Some(node) = builder.anchors.get(&id).cloned() else {
        return Err(ParseFailure { at: None });
    };
    attach(builder, node);
    Ok(())
}

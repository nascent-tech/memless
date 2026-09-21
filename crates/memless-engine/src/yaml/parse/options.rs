use serde_saphyr::granit_parser::Options;

pub(super) fn saturate_options() -> Options {
    let mut options = Options::default();
    options.emit_comments = false;
    options.flow_nesting_limit = usize::MAX;
    options.block_nesting_limit = usize::MAX;
    options.simple_key_max_lookahead = usize::MAX;
    options.max_buffered_comment_events = usize::MAX;
    options.max_directive_bytes = usize::MAX;
    options.max_reserved_directive_params = usize::MAX;
    options
}

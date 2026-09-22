use super::escape::quote_lexeme;
use super::key_guess::guessed_non_text;
use super::quote::needs_syntax_quote;
use memless_domain::document::RawKey;

pub(super) fn render_key(key: &RawKey) -> String {
    let text = match key {
        RawKey::Text(text) => text,
        RawKey::NonText { rendered } => return quote_lexeme(rendered),
    };
    if needs_syntax_quote(text) || guessed_non_text(text) {
        return quote_lexeme(text);
    }
    text.clone()
}

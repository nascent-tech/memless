pub(crate) fn guess_decimal(lexeme: &str) -> Option<f64> {
    let unsigned = lexeme.strip_prefix(['+', '-']).unwrap_or(lexeme);
    let (integer, fraction) = unsigned.split_once('.')?;
    let well_formed = !integer.is_empty()
        && !fraction.is_empty()
        && integer.bytes().all(|byte| byte.is_ascii_digit())
        && fraction.bytes().all(|byte| byte.is_ascii_digit());
    well_formed.then(|| lexeme.parse::<f64>().ok().filter(|value| value.is_finite())).flatten()
}

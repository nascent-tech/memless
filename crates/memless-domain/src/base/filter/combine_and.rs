pub(crate) fn combine_and(results: &mut Vec<bool>) {
    let right = results.pop().unwrap_or(false);
    let left = results.pop().unwrap_or(false);
    results.push(left && right);
}

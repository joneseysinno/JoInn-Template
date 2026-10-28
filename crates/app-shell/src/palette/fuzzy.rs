/// Subsequence match: "pext" matches "Part extrude".
pub(super) fn fuzzy(q: &str, s: &str) -> bool {
    let mut it = s.chars();
    q.chars().filter(|c| !c.is_whitespace()).all(|c| it.any(|x| x == c))
}

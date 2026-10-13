//! Small text helpers for the list filters.

/// `haystack.to_lowercase().contains(needle_lower)` without allocating the lowercased haystack.
///
/// The filters run over every row on every frame (17 568 melee weapons, ~70 000 armors); cloning each name
/// and lowercasing it was about 60 % of the frame cost while a search box had text in it. `needle_lower`
/// must already be lowercase. An ASCII needle is matched byte-wise (a multi-byte UTF-8 character never
/// contains an ASCII byte, so this cannot match inside one); any other needle takes the allocating path.
pub fn contains_ci(haystack: &str, needle_lower: &str) -> bool {
    if needle_lower.is_empty() {
        return true;
    }
    if needle_lower.is_ascii() {
        let n = needle_lower.as_bytes();
        let h = haystack.as_bytes();
        if n.len() > h.len() {
            return false;
        }
        return h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n));
    }
    haystack.to_lowercase().contains(needle_lower)
}

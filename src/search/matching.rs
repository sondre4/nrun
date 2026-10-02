use nucleo::{Matcher, Utf32Str, pattern::Pattern};

pub fn search_query_empty(search_query: &str) -> bool {
    return search_query.trim().is_empty();
}

pub fn match_find_score(pattern: &Pattern, utf32_buffer: &mut Vec<char>, matcher: &mut Matcher, text: &str) -> f64 {
    let utf32_text = Utf32Str::new(text, utf32_buffer);
    let found_score = pattern.score(utf32_text, matcher);

    if found_score.is_none() {
        return 0.0f64;
    }

    let score = found_score.unwrap();
    let score_f64 = f64::from(score);
    return score_f64;
}

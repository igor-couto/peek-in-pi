pub fn naive_brute_force_search(pattern: &[u8], text: &[u8]) -> Result<u32, String> {
    'search_inside_text: for index in 0..text.len() {
        if text[index] == pattern[0] {
            for i in 0..pattern.len() {
                if pattern[i] != text[index + i] {
                    continue 'search_inside_text;
                }
            }
            return Ok(index as u32);
        }
    }
    Err(String::from("Not found."))
}

pub fn search(number_to_search: &String, pi: &String) -> Result<u32, String> {
    let number_to_search_bytes = number_to_search.as_bytes();
    let pi_bytes = pi.as_bytes();

    for pi_index in 0..pi_bytes.len() {
        println!("pi_index: {}", pi_index);
        if pi_bytes[pi_index] == number_to_search_bytes[0] {
            for i in 1..number_to_search_bytes.len() {
                if number_to_search_bytes[i] != pi_bytes[pi_index + i] {
                    break;
                }
                return Ok(pi_index as u32);
            }
        }
    }

    Err(String::from("Not found."))
}

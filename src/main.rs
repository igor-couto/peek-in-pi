mod input;
mod pi;
mod search;

fn main() {
    let number_to_search = input::get_player_input();

    let file_content = pi::open_one_million_file().expect("Fail to read the pi file");

    let number_to_search_bytes = number_to_search.as_bytes();
    let pi_bytes = file_content.as_bytes();

    let result = search::naive_brute_force_search(&number_to_search_bytes, &pi_bytes);

    println!("Result: {}", result.unwrap() + 1);
}

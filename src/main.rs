mod input;
mod pi;
mod search;

fn main() {
    let number_to_search = input::get_player_input();

    let file_content = pi::open_one_million_file().expect("Fail to read the pi file");

    let result = search::search(&number_to_search, &file_content);

    println!("Result: {}", result.unwrap() + 1);
}

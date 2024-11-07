pub fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
    .expect("Error to read input");
    return input.trim().to_string();
}

pub fn get_price() -> f32 {
    let input = get_input().replace(",",".");
    return input.parse::<f32>().unwrap();
}
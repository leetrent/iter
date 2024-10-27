fn find_color_or(elements: &[String], search: &str, fallback: &str) -> String {
    elements
    .iter()
    .find(|el| el.contains(search))
    .map_or(String::from(fallback), |el| el.to_string()
    )
}

fn main() {
    let colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];
    println!("Original: {:#?}", colors);

    let found_color = find_color_or(&colors, "re", "Orang");
    println!("found_color: {:#?}", found_color);

    let not_found_color = find_color_or(&colors, "abc", "Orange");
    println!("not_found_color: {:#?}", not_found_color);
}


fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
    .iter()
    .map(
        |el| el.chars().map(|c| c.to_string()).collect()
    )
    .collect()
}

fn main() {
    let colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];
    println!("Original: {:#?}", colors);

    let exploded = explode(&colors);
    println!("Exploded: {:#?}", exploded);
}


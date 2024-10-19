fn shorten_strings(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn main() {
    let mut colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    // BEFORE
    println!("{:#?}", colors);
    shorten_strings(&mut colors);

    // AFTER
    println!("{:#?}", colors)
}

fn shorten_strings_using_vec(elements: &mut Vec<String>) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}

fn shorten_strings_using_slice(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| el.truncate(1));
}
fn main() {
    use_vec();
    use_slice();
}

fn use_vec() {
    let mut colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    // BEFORE
    println!("{:#?}", colors);
    shorten_strings_using_vec(&mut colors);

    // AFTER
    println!("{:#?}", colors)
}

fn use_slice() {
    let mut colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    // BEFORE
    println!("{:#?}", colors);
    shorten_strings_using_slice(&mut colors[1..3]);

    // AFTER
    println!("{:#?}", colors)
}

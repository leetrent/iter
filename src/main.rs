fn print_elements(elements: &Vec<String>) {
    elements.iter()
    .map(|el| format!("{} {}", el, el)) // iterator adapter (does not call "next")
    .for_each(|el| println!("{}", el)); // iterator consumer (calls "next")
}


fn main() {
    let colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    print_elements(&colors);
}

fn print_elements(elements: &Vec<String>) {
    elements.iter().for_each(|el| println!("{}", el));
}


fn main() {
    let colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    print_elements(&colors);
}

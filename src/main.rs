// fn to_uppercase(elements: &[String]) -> Vec<String> {
//     elements
//     .iter()
//     .map(|el| el.to_uppercase())
//     .collect()
// }

fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements
    .iter()
    .map(|el| el.to_uppercase())
    .collect::<Vec<String>>()
}

fn main() {
    let colors = vec! [
        String::from("red"),
        String::from("green"),
        String::from("blue")
    ];

    println!("{:#?}", colors);
    let uppercased = to_uppercase(&colors);
    println!("{:#?}", uppercased)
}


fn main() {
    let mut sentence = String::from("My Hello, world!");
    for _ in 0..100 {
        sentence.push_str("hgshgvgsvgsvgsvgvs");
        println!("{:p}", sentence.as_ptr());
    }
}
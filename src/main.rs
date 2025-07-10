use rand::seq::SliceRandom;

fn get_random_sentence() -> &'static str {
    let sentences = [
        "this is a random 1",
        "this is a random 2",
        "this is a random 3",
        "this is a random 4",
        "this is a random 5",
    ];

    let mut rng = rand::thread_rng();
    sentences.choose(&mut rng).unwrap()
}

fn main() {
    let res = get_random_sentence();

    println!("{}", res)
}

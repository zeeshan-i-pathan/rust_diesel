use rust_diesel::{repository::PostRepository, *};

fn main() {
    let connection = establish_connection();
    let results = PostRepository::new(connection).find_all().unwrap();

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}

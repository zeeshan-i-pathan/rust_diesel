use std::io::{stdin, Read};

use rust_diesel::{establish_connection, models::NewPost, repository::PostRepository};

fn main() {
    let connection = establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end(); // Remove the trailing newline

    println!("\nOk! Let's write {title} (Press {EOF} when finished)\n",);
    stdin().read_to_string(&mut body).unwrap();

    let post = PostRepository::new(connection)
        .create(NewPost {
            title: title,
            body: &body,
        })
        .unwrap();
    println!("\nSaved draft {title} with id {}", post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";

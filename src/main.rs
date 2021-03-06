use std::{env, process};

#[tokio::main]
async fn main() {

let args: Vec<String> = env::args().collect();
let name: &str;
if args.len() >1 {
    name = &args[1];
} else {
    eprintln!("Please enter a name. e.g cargo run James");
    process::exit(1);
}

let result = cli::fetch_data(name);

let obj_json = cli::turn_into_json(&result.await.expect("could not load"));

let predictions = obj_json.unwrap();
cli::show_predictions(predictions);
}

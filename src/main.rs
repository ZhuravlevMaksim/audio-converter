use std::env;
use youtube_url_unwrap::Extractor;
use std::process::exit;
use std::fs::File;
use std::io::{Write, BufWriter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let uid = if let Some(uid) = args.get(1) {
        println!("Working on: {:?}", uid);
        uid
    } else {
        println!("No uid arg, exit.");
        exit(0)
    };

    let stream = Extractor::new().get_opus_stream(uid).await?;

    println!("{:#?}", stream);

    let client = reqwest::Client::new();
    let res = client.get(stream.url.unwrap())
        .send().await?
        .bytes().await?;

    {
        let file = File::create("./".to_owned() + &stream.title + ".opus")
            .expect("Unable to create file");

        BufWriter::new(file).write_all(&res)
            .expect("Unable to write data");
    }

    Ok(())
}

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let stream = youtube_url_unwrap::platform::get_opus_stream(args.get(1).unwrap());

    println!("{:#?}", stream.await?);

    Ok(())

}

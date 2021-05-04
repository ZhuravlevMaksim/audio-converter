use std::env;
use youtube_url_unwrap::Extractor;
use std::process::{exit, Command};
use std::fs::{File, rename};
use tempfile::tempdir;
use std::io::{Write, BufWriter};
use std::ffi::OsStr;

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
    println!("downloading...");

    let client = reqwest::Client::new();
    let res = client.get(stream.url.unwrap())
        .send().await?
        .bytes().await?;

    println!("prepare file");

    let dir = tempdir()?;
    let temp_opus = dir.path().join("temp.opus");
    let file = File::create(&temp_opus)?;
    BufWriter::new(&file).write_all(&res)
        .expect("Unable to write data");

    println!("{:?}", OsStr::new(&temp_opus));

    let tmp_name = "tmp.mp3"; // workaround for cmd /c when need quotes
    let command = format!("ffmpeg -i {} -vn -ar 44100 -ac 2 -b:a 192k {}", OsStr::new(&temp_opus).to_str().unwrap(), tmp_name);

    println!("{:?}", &command);

    Command::new("cmd")
        .args(&["/C", &command])
        .output()
        .expect("failed to execute process");

    rename(tmp_name,  format!("{}.mp3", stream.title));

    drop(file);
    dir.close()?;

    println!("done");

    Ok(())
}

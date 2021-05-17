use std::{env, io};
use youtube_url_unwrap::Extractor;
use std::process::{exit, Command};
use std::fs::{File, rename};
use tempfile::tempdir;
use std::io::{Write, BufWriter, Read};
use std::ffi::OsStr;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if let Some(uid) = args.get(1) {
        if uid.contains("y_uid.json") {
            let input = read_input()?;
            let uid = input.get("uid").unwrap().as_str().unwrap();
            extract(uid).await?;
            write_web_nm(&serde_json::to_string(&args).unwrap())?
        } else {
            println!("Working on: {:?}", uid);
            extract(&uid).await?
        };
    }

    Ok(())
}

// todo: тут по всей видимости нужно передать размер в начале, но все это магия какая-то, надо почитать
pub fn write_web_nm(msg: &str) -> io::Result<()> {
    let mut outstream = io::stdout();

    let message = serde_json::to_string(msg)?;

    let len = message.len();
    if len > 1024 * 1024 {
        panic!("Message was too large, length: {}", len)
    }

    // тут особенно
    let mut size_vector = vec![];
    size_vector.write_u32::<LittleEndian>(len as u32).unwrap();

    outstream.write(&size_vector)?;
    outstream.write_all(message.as_bytes())?;
    outstream.flush()?;
    Ok(())
}

pub fn read_input() -> io::Result<serde_json::Value> {
    let mut input = io::stdin();
    let length = input.read_u32::<LittleEndian>().unwrap();
    let mut buffer = vec![0; length as usize];
    input.read_exact(&mut buffer)?;
    let json_val: serde_json::Value = serde_json::from_slice(&buffer).unwrap();
    Ok(json_val)
}

async fn extract(uid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let stream = Extractor::new().get_opus_stream(uid).await?;

    // println!("{:#?}", stream);
    // println!("downloading...");

    let client = reqwest::Client::new();
    let res = client.get(stream.url.unwrap())
        .send().await?
        .bytes().await?;

    // println!("prepare file");

    let dir = tempdir()?;
    let temp_opus = dir.path().join("temp.opus");
    let file = File::create(&temp_opus)?;
    BufWriter::new(&file).write_all(&res)
        .expect("Unable to write data");

    // println!("{:?}", OsStr::new(&temp_opus));

    let tmp_name = "tmp.mp3"; // workaround for cmd /c when need quotes
    let command = format!("ffmpeg -i {} -vn -ar 44100 -ac 2 -b:a 192k {}", OsStr::new(&temp_opus).to_str().unwrap(), tmp_name);

    // println!("{:?}", &command);

    Command::new("cmd")
        .args(&["/C", &command])
        .output()
        .expect("failed to execute process");

    rename(tmp_name, format!("{}.mp3", stream.title));

    drop(file);
    dir.close()?;

    // println!("done");

    Ok(())
}

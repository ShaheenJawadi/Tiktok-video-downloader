use std::fs::File;
use std::io::Write;
use reqwest::{Client, header};
use tokio;
use serde_json::Value;
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::Input;
use std::path::Path;
#[tokio::main]

async fn main() {

    let url: String = Input::new()
        .with_prompt("🔗 Enter the TikTok video URL")
        .interact_text()
        .unwrap();

    match download_tiktok_video(&url).await {
        Ok(output_file) => println!("✅ Download complete: {}", output_file),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
}

async fn download_tiktok_video(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    println!("🔍 Fetching video metadata...");

    let api_url = format!("https://www.tikwm.com/api/?url={}", url);
    let response = client.get(&api_url)
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .await?;

    let json: Value = response.json().await?;
    let video_url = json["data"]["play"]
        .as_str()
        .ok_or("❌ Video URL not found in API response")?;

    let video_title = json["data"]["title"]
        .as_str()
        .unwrap_or("tiktok_video");

    let mut output_file = format!("{}.mp4", video_title);

    if Path::new(&output_file).exists() {
        output_file = format!("{}-{}.mp4", video_title, chrono::Utc::now().timestamp());
    }

    println!("⬇️ Downloading video from: {}", video_url);
    let video_response = client.get(video_url).send().await?;
    let total_size = video_response.content_length().unwrap_or(0);
    let bytes = video_response.bytes().await?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-"));

    let mut file = File::create(&output_file)?;
    file.write_all(&bytes)?;
    pb.finish_with_message("✅ Download complete!");

    println!("📁 Video saved as: {}", output_file);
    Ok(output_file)
}

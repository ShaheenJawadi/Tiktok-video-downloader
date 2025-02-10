# TikTok Video Downloader

A fast and efficient command-line tool written in Rust that downloads TikTok videos while preserving their original quality. The tool features a progress bar, automatic file naming, and handles duplicate filenames.

## Features

- 🚀 Fast downloads using asynchronous operations
- 📊 Real-time progress bar with ETA
- 🔄 Automatic handling of duplicate filenames
- 💫 Simple interactive command-line interface
- 🎥 Preserves original video quality

## Installation

### Prerequisites

- Rust 1.83 or higher
- Cargo 

### Building from Source

1. Clone the repository:
```bash
git clone https://github.com/ShaheenJawadi/tiktok-video-downloader.git
cd tiktok-video-downloader
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/tiktok-video-downloader`

### Docker Installation

1. Build the Docker image:
```bash
docker build -t tiktok-downloader .
```

2. Run the container:
```bash
docker run -it tiktok-downloader
```

## Usage

1. Run the program:
```bash
cargo run --release
```

2. When prompted, paste the TikTok video URL:
```
🔗 Enter the TikTok video URL: https://www.tiktok.com/@user/video/1234567890
```

3. The program will:
    - Fetch the video metadata
    - Download the video
    - Show a progress bar
    - Save the video with the original title
 

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Disclaimer

This tool is for educational purposes only. Please respect TikTok's terms of service and content creators' rights when using this tool.

## Acknowledgments

- Thanks to the Rust community for the excellent crates used in this project
- Built using the public TikWM API
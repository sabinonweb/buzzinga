
# Reddit Video Scraper & Compilation Bot

A Rust-based bot that collects videos and images from Reddit, processes them, and creates compilation videos automatically for upload to YouTube.  

This project automates content creation by combining videos, memes, and text-to-speech into engaging compilation videos.  

---

## Features

- **Video Compilation**  
  Collect videos from multiple subreddits (gaming, absurd content, memes, etc.) and stitch them into a single compilation.

- **Thumbnail Generation**  
  Automatically generate thumbnails for your videos.

- **Meme & Image Processing**  
  - Compile meme images.  
  - Extract text from images using OCR.  
  - Convert extracted text to speech for narration.

- **YouTube Upload**  
  Automatically upload compiled videos to YouTube (up to 5 per day due to API limits).  

---

## Incentive

This bot is designed for:  

- Automating repetitive content creation tasks.  
- Producing daily uploads without manual effort.  
- Experimenting with Rust for multimedia processing and Reddit API integration.  

---

## Tech Stack

- **Rust** – Main programming language.  
- **Roux** – Rust wrapper for Reddit API.  
- **FFmpeg** – Video processing (stitching, audio, conversion).  
- **Tesseract OCR** – Extract text from images.  
- **gTTS / Text-to-Speech** – Convert extracted text to audio.  
- **YouTube API** – Upload videos programmatically.  

Optional GUI for testing/monitoring can be built using **Qt for Rust (Rust Qt binding)**.

---

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/reddit-video-scraper.git
cd reddit-video-scraper

# Ensure Rust is installed (with cargo)
rustc --version
cargo --version

# Install FFmpeg and Tesseract OCR on your system
# Example for macOS with Homebrew:
brew install ffmpeg tesseract

# Example for Ubuntu/Debian:
sudo apt update
sudo apt install ffmpeg tesseract-ocr

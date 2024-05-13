use lazy_static::lazy_static;
use regex_lite::Regex;
use serde::{Deserialize, Serialize};
use std::{mem, path::PathBuf};

extern "C" {
    fn callback(info_ptr: *mut u8, info_len: u32);
}

lazy_static! {
    static ref YOUTUBE_MEDIA_IDENT: Regex = Regex::new(r"\s\[([a-zA-Z0-9_-]+)\]$").unwrap();
}

#[no_mangle]
pub extern "C" fn process_media_info_json(info_ptr: *mut u8, info_len: u32) {
    let info_len = info_len as usize;
    let buffer = unsafe { Vec::from_raw_parts(info_ptr, info_len, info_len) };
    match serde_json::from_slice::<MediaInfo>(&buffer) {
        Ok(mut info) => {
            let title = info.title.clone();
            let caps = YOUTUBE_MEDIA_IDENT.captures(&title);
            if let Some(caps) = caps {
                let vid = &caps[1];
                if info.title != vid {
                    info.title = info.title.replace(&format!(" [{}]", vid), "");
                    info.categories.push("YouTube".to_owned());
                    info.cover_url = Some(format!("https://i3.ytimg.com/vi/{vid}/mqdefault.jpg"));
                    if let Ok(mut bytes) = serde_json::to_string(&info).map(|r| r.into_bytes()) {
                        let ptr = bytes.as_mut_ptr();
                        let len = bytes.len();
                        mem::forget(bytes);
                        unsafe { callback(ptr, len as u32) };
                    }
                }
            }
        }
        Err(err) => {
            println!("Parse media info error: {}", err);
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaInfo {
    pub id: i64,
    pub path: PathBuf,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: u32,
    pub library: String,
    pub cover_path: Option<PathBuf>,
    pub cover_url: Option<String>,
    pub sample_rate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub audio_bitrate: Option<u32>,
    pub overall_bitrate: Option<u32>,
    pub channels: Option<u8>,
    pub duration_seconds: u32,
    pub categories: Vec<String>,
    pub file_name: String,
    pub file_type: String,
}

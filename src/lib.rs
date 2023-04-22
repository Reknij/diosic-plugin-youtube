use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{ffi::{CStr, CString}, mem};

#[no_mangle]
pub extern "C" fn process_media_info_json(info_ptr: *const u8, file_path: *const u8) -> *const u8 {
    let str = unsafe { CStr::from_ptr(info_ptr as _).to_str().unwrap() };
    let info = serde_json::from_str::<MediaInfo>(str);
    match info {
        Ok(mut info) => {
            let re = Regex::new(r"\s\[(\w+)\]$").unwrap();
            let title = info.title.clone();
            let caps = re.captures(&title);
            match caps {
                Some(caps) => {
                    let vid = &caps[1];
                    if info.title != vid {
                        info.title = info.title.replace(&format!(" [{}]", vid), "");
                        info.categories.push("YouTube".to_owned());
                        info.cover = Some(format!("http://img.youtube.com/vi/{}/hqdefault.jpg", vid));

                        let r = serde_json::to_string(&info);
                        if let Ok(r) = r {
                            let c_string = CString::new(r);
                            match c_string {
                                Ok(cs) => {
                                    cs.as_ptr() as _
                                },
                                Err(_) => info_ptr,
                            }
                        } else {
                            info_ptr
                        }
                    }
                    else {
                        info_ptr
                    }
                }
                None => info_ptr,
            }
        }
        Err(err) => {
            println!("err: {}", err);
            info_ptr
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MediaInfo {
    pub id: String,
    pub path: String,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub genre: String,
    pub year: String,
    pub library: String,
    pub cover: Option<String>,
    pub categories: Vec<String>,
    pub simple_rate: Option<u32>,
    pub bit_depth: Option<u8>,
    pub audio_bitrate: Option<u32>,
    pub overall_bitrate: Option<u32>,
    pub channels: Option<u8>,
    pub duration_milliseconds: u128,
}

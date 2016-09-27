extern crate hyper;
extern crate libc;
extern crate rustc_serialize;

use std::{thread, slice, str};
use std::io::Read;
use std::sync::mpsc;
use std::ffi::{CStr, CString};
use std::collections::HashMap;
use libc::{c_char, size_t};
use hyper::{Client, Url};
use rustc_serialize::json;

#[no_mangle]
pub extern fn get_htmls_from(urls:*const *const c_char, lenght: size_t) -> *mut c_char {

    let urls = unsafe {
        slice::from_raw_parts(urls, lenght as usize)
    };
    let urls: Vec<String> = urls.iter()
        .map(|&p| unsafe { CStr::from_ptr(p) })         // make iter of &CStr
        .map(|cs| cs.to_bytes())                        // make iter of &[u8]
        .map(|bs| str::from_utf8(bs).unwrap_or(""))     // make iter of &str
        .map(|s| s.to_string())                         // make iter of String
        .collect();

    let (tx, rx) = mpsc::channel();
    for url in urls.iter() {
        let url = url.clone();
        let tx = tx.clone();
        thread::spawn(move || {
            let client = Client::new();
            let mut html = String::new();
            match Url::parse(&url) {
                Result::Err(_) => {},
                Result::Ok(hyper_url) => {
                    match client.get(hyper_url).send() {
                        Result::Err(_) => {},
                        Result::Ok(mut res) => {
                            match res.read_to_string(&mut html) {
                                _ => {}
                            }
                        }
                    }
                }
            };
            tx.send((url, html)).unwrap();
        });
    }

    let mut url2bodies  = HashMap::new();
    for _ in urls.iter() {
        match rx.recv() {
            Result::Ok((url, html)) => { url2bodies.insert(url, html); },
            Result::Err(_) => {}
        }
    }

    let result = json::encode(&url2bodies).unwrap();
    CString::new(result).unwrap().into_raw()
}

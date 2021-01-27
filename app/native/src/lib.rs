use neon::prelude::*;
use neon::register_module;
mod scrapper;

extern crate hyper;


use std::io::Read;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;


use hyper::{ client::{Client, ResponseFuture}, header, http::{Request, Response, StatusCode} };
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    scrapper::check_mod();
    return Ok(cx.string(scrapper::test()));
}

register_module!(mut cx, {
    cx.export_function("hello", hello)
});

fn main() {
    //Regex for youtube URLs.
    let url_regex = Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*").unwrap();
    let mut vid = "https://www.youtube.com/watch?v=uhlfcFzsLGU&ab_channel=Nix77";
    let is_adaptive = false;
    if url_regex.is_match(vid) 
    {
        let vid_split = url_regex.captures(vid).unwrap();
        vid = vid_split.get(1).unwrap().as_str();
    }
    let url = format!("https://youtube.com/get_video_info?video_id={}", vid);
    download(&url);
}

fn get_vid(url: &str) {
    //let mut response =
}

async fn download(url: &str)
{
    let mut response = request(url).await;
    let mut resp_str = String::new();
    response.read_to_string(&mut resp_str);
    let info = scrapper::VideoInfo::parse(&resp_str).unwrap();

    //println!("Vid info: {}", info);

    let streams = info.streams;

    for (i, stream) in streams.iter().enumerate() 
    {
        println!("{}- {} {}", i, stream.quality, stream.stream_type);
    }

    
    //println!("Choose quality (0): ");

    println!("Please wait...");

    if let Some(ref stream) = streams.get(480) 
    {
        // get response from selected quality
        println!("Downloading {}", url);
        let response = send_request(&stream.url);
        println!("Download is starting...");

        let filename = match stream.extension() {
            Some(ext) => format!("{}.{}", info.title, ext),
            None => info.title,
        };

        // write file to disk
        write_file(response, &filename);
    } 
    else 
    {
        println!("Invalid stream index");
    }
    
}

// fn downloadj(url: &str, adaptive: bool) {
//     println!("Fetching video info from {}", url);
//     let mut response = send_request(url);
//     let mut response_str = String::new();
//     response.read_to_string(&mut response_str).unwrap();
//     println!("Response {}", response_str);
//     let info = scrapper::VideoInfo::parse(&response_str).unwrap();
//     //debug!("Video info {:#?}", info);

//     let streams = if adaptive {
//         info.adaptive_streams
//     } else {
//         info.streams
//     };

//     for (i, stream) in streams.iter().enumerate() {
//         println!("{}- {} {}",
//                  i,
//                  stream.quality,
//                  stream.stream_type);
//     }

//     println!("Choose quality (0): ");
//     let input = read_line().trim().parse().unwrap_or(0);

//     println!("Please wait...");

//     if let Some(ref stream) = streams.get(input) {
//         // get response from selected quality
//         println!("Downloading {}", url);
//         let response = send_request(&stream.url);
//         println!("Download is starting...");

//         // get file size from Content-Length header
//         let file_size = get_file_size(&response);

//         let filename = match stream.extension() {
//             Some(ext) => format!("{}.{}", info.title, ext),
//             None => info.title,
//         };

//         // write file to disk
//         write_file(response, &filename, file_size);
//     } else {
//         println!("Invalid stream index");
//     }
// }

async fn send_request (url: &str) -> ResponseFuture {
    return Client::new().get(url.parse().unwrap());
}

// get file size from Content-Length header
fn get_file_size(response: &Response) -> u64 {
    let mut file_size = 0;
    match response.headers.get::<ContentLength>(){
        Some(length) => file_size = length.0,
        None => println!("Content-Length header missing"),
    };
    return file_size;
}

fn write_file(mut response: Response, title: &str) {
    // initialize progressbar


    // Download and write to file
    let mut buf = [0; 128 * 1024];
    let mut file = File::create(title).unwrap();
    loop {
        match response.read(&mut buf) {
            Ok(len) => {
                file.write_all(&buf[..len]).unwrap();
                
                if len == 0 {
                    break;
                }
                len
            }
            Err(why) => panic!("{}", why),
        };
    }
}




// fn send_requestj(url: &str) -> Response {
//     let ssl = NativeTlsClient::new().unwrap();
//     let connector = HttpsConnector::new(ssl);
//     let client = Client::with_connector(connector);
//     return client.get(url).send().unwrap_or_else(|e| {
//         error!("Network request failed: {}", e);
//         process::exit(1);
//     });
// }

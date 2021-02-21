use neon::prelude::*;
use neon::register_module;
pub mod youtube;

extern crate hyper;
use hyper_tls::HttpsConnector;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;


use hyper::{ client::{Client, ResponseFuture }, header, http::{Request, Response, StatusCode} };
//use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


// fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
//     //youtube::check_mod();
//     return Ok(cx.string(format_url("https://www.youtube.com/watch?v=uhlfcFzsLGU&ab_channel=Nix77")));
// }

// register_module!(mut cx, {
//     cx.export_function("hello", hello)
// });

pub async fn send_request (url: &str) -> Result<(), Box<dyn Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let response = client.get(url.parse().unwrap()).await?;
    return Ok(());
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_request() {
//         println!("Begin request.");
//         let vid = "https://www.youtube.com/watch?v=uhlfcFzsLGU&ab_channel=Nix77";
//         let url = format_url(vid);
//         println!("Url: {}", url);
         
//         let mut response = Client::new().get(url.parse().unwrap()).await;

//         let r = response.unwrap();
//         println!("Url: {}", url);
//         r.status();

//         //let mut rt = tokio::runtime::Runtime::new().unwrap();
//         // let response = rt.block_on(async {
//         //     let mut client = Client::new();
//         //     let mut response = client.get(url.parse().unwrap()).await;
//         //     println!("Response: {}", response.unwrap().status());
//         //     //println!("Headers: {:#?}\n", response.unwrap().headers());
//         //     //println!("Body: {:#?}\n", response.unwrap().body());
//         //     // while let Some(next) = response.body() {
//         //     //     let chunk = next?;
//         //     //     io::stdout().write_all(&chunk).await?;
//         //     // }
//         //     println!("End request.");
            
//         // });
    
//     }
// }


// async fn get_vid(url: &str) {
//     let mut response = send_request(url).await;
//     let mut resp_str = String::new();
// }

// async fn download(url: &str)
// {
//     let mut response = send_request(url).await;
//     let mut resp_str = String::new();
//     response.to_string(&mut resp_str);
//     let info = scrapper::VideoInfo::parse(&resp_str).unwrap();

//     //println!("Vid info: {}", info);

//     let streams = info.streams;

//     for (i, stream) in streams.iter().enumerate() 
//     {
//         println!("{}- {} {}", i, stream.quality, stream.stream_type);
//     }

    
//     //println!("Choose quality (0): ");

//     println!("Please wait...");

//     if let Some(ref stream) = streams.get(480) 
//     {
//         // get response from selected quality
//         println!("Downloading {}", url);
//         let response = send_request(&stream.url);
//         println!("Download is starting...");

//         let filename = match stream.extension() {
//             Some(ext) => format!("{}.{}", info.title, ext),
//             None => info.title,
//         };

//         // write file to disk
//         write_file(response, &filename);
//     } 
//     else 
//     {
//         println!("Invalid stream index");
//     }
    
// }

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

// async fn send_request (url: &str) -> Result<()> {
//     let mut response_future = Client::new().get(url.parse().unwrap()).await {
//         Ok(v) => v,
//         Err(e) => {
//             hyper::Response::builder()
//                 .status(Status::BAD_REQUEST)
//                 .body(format!("Unable to parse JSON: {}", e).into())
//                 .unwrap()
//         }
//     };
// }

// // get file size from Content-Length header
// fn get_file_size(response: &Response) -> u64 {
//     let mut file_size = 0;
//     match response.headers.get::<ContentLength>(){
//         Some(length) => file_size = length.0,
//         None => println!("Content-Length header missing"),
//     };
//     return file_size;
// }

// fn write_file(mut response: Response, title: &str) {
//     // initialize progressbar


//     // Download and write to file
//     let mut buf = [0; 128 * 1024];
//     let mut file = File::create(title).unwrap();
//     loop {
//         match response.read(&mut buf) {
//             Ok(len) => {
//                 file.write_all(&buf[..len]).unwrap();
                
//                 if len == 0 {
//                     break;
//                 }
//                 len
//             }
//             Err(why) => panic!("{}", why),
//         };
//     }
// }




// fn send_requestj(url: &str) -> Response {
//     let ssl = NativeTlsClient::new().unwrap();
//     let connector = HttpsConnector::new(ssl);
//     let client = Client::with_connector(connector);
//     return client.get(url).send().unwrap_or_else(|e| {
//         error!("Network request failed: {}", e);
//         process::exit(1);
//     });
// }

// pub async fn send_request (url: &str) -> Result<(), Box<dyn Error>> {
//     let https = HttpsConnector::new();
//     let client = Client::builder().build::<_, hyper::Body>(https);
//     let response = client.get(url.parse().unwrap()).await?;
//     let headers = response.headers();
//     println!("Response: {}", response.status());
//     println!("Headers: {:?}\n", response.headers());
//     println!("Body: {:?}\n", response.body());


//     // while let Some(next) = response.body() {
//     //     let chunk = next?;
//     //     io::stdout().write_all(&chunk).await?;
//     // }

//     //let mut resp_str = String::new();
//     //let info = youtube::VideoInfo::parse(headers).unwrap();
//     //println!("Vid title: {}", info.title);

// //     let streams = info.streams;

// //     for (i, stream) in streams.iter().enumerate() 
// //     {
// //         println!("{}- {} {}", i, stream.quality, stream.stream_type);
// //     }


//     println!("End request.");
//     return Ok(());
// }

// async fn fetch_url(url: hyper::Uri) -> Result<(), Box<dyn Error>> {
//     let client = Client::new();

//     let mut res = client.get(url).await?;

//     println!("Response: {}", res.status());
//     println!("Headers: {:#?}\n", res.headers());

//     // Stream the body, writing each chunk to stdout as we get it
//     // (instead of buffering and printing at the end).
//     while let Some(next) = res.data().await {
//         let chunk = next?;
//         io::stdout().write_all(&chunk).await?;
//     }

//     println!("\n\nDone!");

//     Ok(())
// }
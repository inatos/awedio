
mod scrapper;

#[tokio::main]
async fn main() {
    let vid = "https://www.youtube.com/watch?v=uhlfcFzsLGU&ab_channel=Nix77";
    
    let url = scrapper::format_url(vid);

    let response = scrapper::send_request(&url).await.unwrap();
    //println!("Status: {}", response.status());
    //download(&url);
}
use mangadex_rust::client::MDex;

#[tokio::main]
async fn main() {
    let mdex = MDex::new();
    println!("Fetching manga!");
    let m = mdex.manga("a96676e5-8ae2-425e-b549-7f15dd34a6d8".to_string()).await;
    println!("{:?}", m);
    println!("Finished fetching Komi-san!");

    let mangas = mdex.mangas().await;
    println!("{:?}", mangas);
}

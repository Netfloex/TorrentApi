use torrent_search_client::TorrentClient;

#[tokio::main]

async fn main() {
    let client = TorrentClient::new();
    let torrents = client.search("Ubuntu Server").await;

    for torrent in torrents.iter() {
        println!("Torrent: {}", torrent.name);
        println!("Size: {}", torrent.size);
    }
}

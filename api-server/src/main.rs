use torrent_search_client::TorrentClient;

#[tokio::main]
async fn main() {
    let client = TorrentClient::new();
    client.search("Ubuntu Server").await;
}

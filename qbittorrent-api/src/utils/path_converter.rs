const DOWNLOADS_LOCAL: &str = "/downloads";
const DOWNLOADS_REMOTE: &str = "/downloads";

pub fn remote_to_local(remote: &str) -> String {
    remote.replace(DOWNLOADS_REMOTE, DOWNLOADS_LOCAL)
}

// pub fn local_to_remote(local: &str) -> String {
//     local.replace(DOWNLOADS_LOCAL, DOWNLOADS_REMOTE)
// }

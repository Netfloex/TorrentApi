use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "graphql", derive(async_graphql::Enum))]

pub enum TorrentState {
    /// Some error occurred, applies to paused torrents
    Error,
    /// Torrent data files is missing
    MissingFiles,
    /// Torrent is being seeded and data is being transferred
    Uploading,
    /// Torrent is paused and has finished downloading
    PausedUP,
    /// Queuing is enabled and torrent is queued for upload
    QueuedUP,
    /// Torrent is being seeded, but no connection were made
    StalledUP,
    /// Torrent has finished downloading and is being checked
    CheckingUP,
    /// Torrent is forced to uploading and ignore queue limit
    ForcedUP,
    /// Torrent is allocating disk space for download
    Allocating,
    /// Torrent is being downloaded and data is being transferred
    Downloading,
    /// Torrent has just started downloading and is fetching metadata
    MetaDL,
    /// Torrent is paused and has NOT finished downloading
    PausedDL,
    /// Queuing is enabled and torrent is queued for download
    QueuedDL,
    /// Torrent is being downloaded, but no connection were made
    StalledDL,
    /// Same as checkingUP, but torrent has NOT finished downloading
    CheckingDL,
    /// Torrent is forced to downloading to ignore queue limit
    ForcedDL,
    /// Checking resume data on qBt startup
    CheckingResumeData,
    /// Torrent is moving to another location
    Moving,
    /// Unknown status
    Unknown,
}

impl TorrentState {
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            TorrentState::Allocating
                | TorrentState::Downloading
                | TorrentState::MetaDL
                | TorrentState::CheckingDL
                | TorrentState::ForcedDL
                | TorrentState::CheckingResumeData
                | TorrentState::Unknown
        )
    }
}

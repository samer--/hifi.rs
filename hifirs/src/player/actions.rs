use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    Play,
    Pause,
    PlayPause,
    Next,
    Previous,
    Stop,
    Quit,
    SkipTo { num: u32 },
    JumpForward,
    JumpBackward,
    VolumeUp,
    VolumeDown,
    SetVolume { volume: f64 },
    PlayAlbum { album_id: String },
    PlayTrack { track_id: i32 },
    PlayUri { uri: String },
    PlayPlaylist { playlist_id: i64 },
    Search { query: String },
    FetchArtistAlbums { artist_id: i32 },
    FetchPlaylistTracks { playlist_id: i64 },
    FetchUserPlaylists,
}

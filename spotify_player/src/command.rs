use serde::Deserialize;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
/// Application's command
pub enum Command {
    NextTrack,
    PreviousTrack,
    ResumePause,
    Repeat,
    Shuffle,

    Quit,
    OpenCommandHelp,
    ClosePopup,

    SelectNext,
    SelectPrevious,
    ChoseSelected,

    SwitchPlaylist,
    SwitchTheme,
    SwitchDevice,

    SearchContextTracks,

    PlaySelectedTrackAlbum,

    SortByTrack,
    SortByArtists,
    SortByAlbum,
    SortByDuration,
    SortByAddedDate,
    ReverseOrder,
}
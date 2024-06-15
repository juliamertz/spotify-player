use crate::{config, key};

pub type UIStateGuard<'a> = parking_lot::MutexGuard<'a, UIState>;

mod page;
mod popup;

use super::*;

#[cfg(feature = "fzf")]
use fuzzy_matcher::skim::SkimMatcherV2;

pub use page::*;
pub use popup::*;

#[derive(Default, Debug)]
pub struct ImageRenderInfo {
    pub url: String,
    pub render_area: tui::layout::Rect,
    /// indicates if the image is rendered
    pub rendered: bool,
}

/// Application's UI state
#[derive(Debug)]
pub struct UIState {
    pub is_running: bool,
    pub theme: config::Theme,
    pub input_key_sequence: key::KeySequence,

    pub history: Vec<PageState>,
    pub popup: Option<PopupState>,

    /// The rectangle representing the playback progress bar,
    /// which is mainly used to handle mouse click events (for seeking command)
    pub playback_progress_bar_rect: tui::layout::Rect,

    #[cfg(feature = "image")]
    pub last_cover_image_render_info: ImageRenderInfo,
}

impl UIState {
    pub fn current_page(&self) -> &PageState {
        self.history.last().expect("non-empty history")
    }

    pub fn current_page_mut(&mut self) -> &mut PageState {
        self.history.last_mut().expect("non-empty history")
    }

    pub fn new_search_popup(&mut self) {
        self.current_page_mut().select(0);
        self.popup = Some(PopupState::Search {
            query: "".to_owned(),
        });
    }

    pub fn new_page(&mut self, page: PageState) {
        self.history.push(page);
        self.popup = None;
    }

    pub fn new_radio_page(&mut self, uri: &str) {
        self.new_page(PageState::Context {
            id: None,
            context_page_type: ContextPageType::Browsing(super::ContextId::Tracks(TracksId::new(
                format!("radio:{uri}"),
                "Recommendations",
            ))),
            state: None,
        });
    }

    /// Return whether there exists a focused popup.
    ///
    /// Currently, only search popup is not focused when it's opened.
    pub fn has_focused_popup(&self) -> bool {
        match self.popup.as_ref() {
            None => false,
            Some(popup) => !matches!(popup, PopupState::Search { .. }),
        }
    }

    /// Get a list of items possibly filtered by a search query if exists a search popup
    pub fn search_filtered_items<'a, T: std::fmt::Display>(&self, items: &'a [T]) -> Vec<&'a T> {
        match self.popup {
            Some(PopupState::Search { ref query }) => {
                let query = query.to_lowercase();
                #[cfg(feature = "fzf")]
                let matcher = SkimMatcherV2::default();

                items
                    .iter()
                    .filter(|t| {
                        if query.is_empty() {
                            true
                        } else {
                            let t = t.to_string().to_lowercase();
                            #[cfg(feature = "fzf")]
                            let m = matcher.fuzzy(&t, &query, false).is_some();
                            #[cfg(not(feature = "fzf"))]
                            let m = query.split(' ').any(|q| !q.is_empty() && t.contains(q));
                            m
                        }
                    })
                    .collect::<Vec<_>>()
            }
            _ => items.iter().collect::<Vec<_>>(),
        }
    }
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            is_running: true,
            theme: Default::default(),
            input_key_sequence: key::KeySequence { keys: vec![] },

            history: vec![PageState::Library {
                state: LibraryPageUIState::new(),
            }],
            popup: None,

            playback_progress_bar_rect: Default::default(),

            #[cfg(feature = "image")]
            last_cover_image_render_info: Default::default(),
        }
    }
}

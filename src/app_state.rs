use crate::content::Site;
use crate::observability::Observability;
use crate::page_cache::{self, PageCache};

#[derive(Clone)]
pub struct AppState {
    pub site: Site,
    pub observability: Observability,
    pub pages: PageCache,
}

impl AppState {
    pub fn new(site: Site, observability: Observability) -> Result<Self, String> {
        let pages = page_cache::preload_pages(&site)?;
        Ok(Self {
            site,
            observability,
            pages,
        })
    }
}

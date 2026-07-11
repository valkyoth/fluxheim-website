use std::collections::HashSet;

use crate::content::Site;
use crate::observability::Observability;
use crate::page_cache::{self, PageCache};

#[derive(Clone)]
pub struct AppState {
    pub site: Site,
    pub observability: Observability,
    pub pages: PageCache,
    pub download_artifacts: HashSet<String>,
}

impl AppState {
    pub fn new(site: Site, observability: Observability) -> Result<Self, String> {
        let pages = page_cache::preload_pages(&site)?;
        let download_artifacts = page_cache::download_artifacts(&pages);
        if download_artifacts.is_empty() {
            return Err("rendered site contains no download artifacts".to_owned());
        }
        Ok(Self {
            site,
            observability,
            pages,
            download_artifacts,
        })
    }
}

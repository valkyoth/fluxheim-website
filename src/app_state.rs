use crate::content::Site;
use crate::observability::Observability;

#[derive(Clone)]
pub struct AppState {
    pub site: Site,
    pub observability: Observability,
}

impl AppState {
    pub fn new(site: Site, observability: Observability) -> Self {
        Self {
            site,
            observability,
        }
    }
}

use serde::Deserialize;

const RELEASES_TOML: &str = include_str!("../content/releases.toml");

#[derive(Debug, Clone, Deserialize)]
pub struct ReleaseCatalog {
    pub latest: String,
    pub release: Vec<Release>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub title: String,
    pub summary: String,
}

impl ReleaseCatalog {
    pub fn load() -> Result<Self, String> {
        let catalog: Self = toml::from_str(RELEASES_TOML).map_err(|error| error.to_string())?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn latest_release(&self) -> Option<&Release> {
        self.release
            .iter()
            .find(|release| release.version == self.latest)
    }

    fn validate(&self) -> Result<(), String> {
        if self.release.is_empty() {
            return Err("at least one release is required".to_owned());
        }

        if self.latest_release().is_none() {
            return Err("latest release is missing from release list".to_owned());
        }

        let mut previous = None;
        for release in &self.release {
            release.validate()?;
            let current = version_tuple(&release.version)?;
            if let Some(previous) = previous
                && current >= previous
            {
                return Err("releases must be newest-first without duplicates".to_owned());
            }
            previous = Some(current);
        }

        Ok(())
    }
}

impl Release {
    fn validate(&self) -> Result<(), String> {
        version_tuple(&self.version)?;

        if !is_iso_date(&self.date) {
            return Err(format!("release {} has invalid date", self.version));
        }

        if self.title.trim().is_empty() || self.summary.trim().is_empty() {
            return Err(format!("release {} has empty text", self.version));
        }

        Ok(())
    }
}

fn version_tuple(version: &str) -> Result<(u32, u32, u32), String> {
    let parts: Vec<_> = version.split('.').collect();
    if parts.len() != 3 {
        return Err(format!("invalid version {version}"));
    }

    let major = parts[0]
        .parse::<u32>()
        .map_err(|_| format!("invalid version {version}"))?;
    let minor = parts[1]
        .parse::<u32>()
        .map_err(|_| format!("invalid version {version}"))?;
    let patch = parts[2]
        .parse::<u32>()
        .map_err(|_| format!("invalid version {version}"))?;

    Ok((major, minor, patch))
}

fn is_iso_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| matches!(index, 4 | 7) || byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::ReleaseCatalog;
    use crate::content::Site;
    use std::path::Path;

    #[test]
    fn release_catalog_matches_site_version() {
        let site = Site::load().expect("site loads");
        let catalog = ReleaseCatalog::load().expect("release catalog loads");

        assert_eq!(catalog.latest, site.config.fluxheim_version);
        assert_eq!(catalog.latest_release().unwrap().version, catalog.latest);
    }

    #[test]
    fn release_catalog_has_backfilled_release_notes() {
        let catalog = ReleaseCatalog::load().expect("release catalog loads");

        for release in &catalog.release {
            let path = format!("docs/releases/RELEASE_NOTES_{}.md", release.version);
            assert!(Path::new(&path).is_file(), "{path} is missing");
        }
    }

    #[test]
    fn release_catalog_versions_are_rendered_on_public_pages() {
        let catalog = ReleaseCatalog::load().expect("release catalog loads");
        let download = std::fs::read_to_string("download.html").expect("download html");
        let changelog = std::fs::read_to_string("changelog.html").expect("changelog html");

        for release in &catalog.release {
            let version = format!("v{}", release.version);
            assert!(download.contains(&version), "download missing {version}");
            assert!(changelog.contains(&version), "changelog missing {version}");
        }
    }
}

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use axum::body::Bytes;

use crate::content::{Locale, Site};
use crate::legacy::{legacy_html_paths, render, slug_for_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CachedPage {
    pub body: Bytes,
    pub slug: String,
}

pub type PageCache = HashMap<String, CachedPage>;

pub fn preload_pages(site: &Site) -> Result<PageCache, String> {
    let locales = site.locales().collect::<Vec<_>>();
    let paths = legacy_html_paths();
    let available = std::thread::available_parallelism()
        .map(usize::from)
        .unwrap_or(1);
    let workers = available.min(4).min(locales.len()).max(1);
    let chunk_size = locales.len().div_ceil(workers);

    let pages = std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for chunk in locales.chunks(chunk_size) {
            handles.push(scope.spawn(|| preload_locale_pages(site, chunk, &paths)));
        }

        let mut pages = HashMap::new();
        for handle in handles {
            let rendered = handle
                .join()
                .map_err(|_| "page preload worker panicked".to_owned())??;
            pages.extend(rendered);
        }
        Ok(pages)
    });
    crate::i18n_keys::clear_replacer_cache();
    pages
}

pub fn cached_page<'a>(
    site: &Site,
    pages: &'a PageCache,
    request_path: &str,
) -> Option<&'a CachedPage> {
    pages.get(&page_cache_key(site, request_path))
}

pub fn download_artifacts(pages: &PageCache) -> HashSet<String> {
    const PREFIX: &str = r#"href="/out/download/"#;
    pages
        .values()
        .filter_map(|page| std::str::from_utf8(&page.body).ok())
        .flat_map(|html| html.split(PREFIX).skip(1))
        .filter_map(|value| value.split_once("?locale=").map(|(artifact, _)| artifact))
        .filter(|artifact| {
            !artifact.is_empty()
                && artifact.len() <= 160
                && artifact
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || b"._-".contains(&byte))
        })
        .map(str::to_owned)
        .collect()
}

fn preload_locale_pages(
    site: &Site,
    locales: &[&Locale],
    paths: &[PathBuf],
) -> Result<PageCache, String> {
    let mut pages = HashMap::new();
    for locale in locales {
        for path in paths {
            for slug in aliases_for_path(path)? {
                let request_path = site.path_for(locale, &slug);
                let page = render(site, &request_path)
                    .ok_or_else(|| format!("cannot render page {request_path}"))?;
                pages.insert(
                    page_cache_key(site, &request_path),
                    CachedPage {
                        body: Bytes::from(page.html),
                        slug: page.slug,
                    },
                );
            }
        }
        crate::i18n_keys::clear_replacer_cache();
    }
    Ok(pages)
}

fn aliases_for_path(path: &Path) -> Result<Vec<String>, String> {
    let slug = slug_for_path(path)
        .ok_or_else(|| format!("cannot derive page slug for {}", path.display()))?;
    if slug.is_empty() {
        return Ok(vec![slug, "index".to_owned(), "index.html".to_owned()]);
    }

    let mut aliases = vec![slug.clone(), format!("{slug}.html")];
    if path.file_name().and_then(|name| name.to_str()) == Some("index.html") {
        let directory = path
            .parent()
            .and_then(Path::to_str)
            .unwrap_or_default()
            .trim_matches('/');
        if !directory.is_empty() {
            aliases.insert(0, directory.to_owned());
        }
    }
    Ok(aliases)
}

fn page_cache_key(site: &Site, request_path: &str) -> String {
    let clean = request_path.trim_matches('/');
    let (locale, slug) = site.split_path(clean);
    format!("{}\0{}", locale.locale_id, slug.trim_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::{cached_page, download_artifacts, preload_pages};
    use crate::content::Site;

    #[test]
    fn preloaded_pages_support_clean_and_legacy_aliases() {
        let site = Site::load().expect("site loads");
        let pages = preload_pages(&site).expect("pages preload");
        let clean = cached_page(&site, &pages, "/de/docs").expect("clean docs page");
        let legacy = cached_page(&site, &pages, "/de/docs/index.html").expect("legacy docs page");
        assert!(String::from_utf8_lossy(&clean.body).contains("fh-language-switcher"));
        assert!(String::from_utf8_lossy(&clean.body).contains(r#"href="/de/docs""#));
        assert!(String::from_utf8_lossy(&legacy.body).contains(r#"href="/de/docs/index.html""#));
        assert_eq!(crate::i18n_keys::replacer_cache_len(), 0);
        let artifacts = download_artifacts(&pages);
        assert!(artifacts.contains("fluxheim-1.7.7-full-x86_64-linux.tar.gz"));
        assert!(artifacts.contains("fluxheim-1.3.2-full-x86_64-linux.tar.gz"));
    }
}

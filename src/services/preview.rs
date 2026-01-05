use scraper::{Html, Selector};
use url::Url;

use crate::error::Result;
use crate::models::BookmarkPreview;

pub struct PreviewService;

impl PreviewService {
    pub async fn fetch_preview(url: &str) -> Result<BookmarkPreview> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        let response = client
            .get(url)
            .header("User-Agent", "Mozilla/5.0 (compatible; XyncBot/1.0)")
            .send()
            .await
            .ok();

        let Some(response) = response else {
            return Ok(BookmarkPreview {
                title: None,
                description: None,
                image: None,
                favicon: None,
            });
        };

        let html = response.text().await.unwrap_or_default();
        let document = Html::parse_document(&html);

        let title = Self::extract_title(&document);
        let description = Self::extract_description(&document);
        let image = Self::extract_image(&document, url);
        let favicon = Self::extract_favicon(url);

        Ok(BookmarkPreview {
            title,
            description,
            image,
            favicon,
        })
    }

    fn extract_title(document: &Html) -> Option<String> {
        let og_title = Selector::parse("meta[property='og:title']").ok()?;
        if let Some(elem) = document.select(&og_title).next() {
            if let Some(content) = elem.value().attr("content") {
                return Some(content.to_string());
            }
        }

        let title = Selector::parse("title").ok()?;
        document
            .select(&title)
            .next()
            .map(|e| e.text().collect::<String>())
    }

    fn extract_description(document: &Html) -> Option<String> {
        let og_desc = Selector::parse("meta[property='og:description']").ok()?;
        if let Some(elem) = document.select(&og_desc).next() {
            if let Some(content) = elem.value().attr("content") {
                return Some(content.to_string());
            }
        }

        let meta_desc = Selector::parse("meta[name='description']").ok()?;
        document
            .select(&meta_desc)
            .next()
            .and_then(|e| e.value().attr("content"))
            .map(|s| s.to_string())
    }

    fn extract_image(document: &Html, base_url: &str) -> Option<String> {
        let og_image = Selector::parse("meta[property='og:image']").ok()?;
        if let Some(elem) = document.select(&og_image).next() {
            if let Some(content) = elem.value().attr("content") {
                return Self::resolve_url(base_url, content);
            }
        }
        None
    }

    fn extract_favicon(base_url: &str) -> Option<String> {
        let parsed = Url::parse(base_url).ok()?;
        Some(format!(
            "{}://{}/favicon.ico",
            parsed.scheme(),
            parsed.host_str()?
        ))
    }

    fn resolve_url(base: &str, path: &str) -> Option<String> {
        if path.starts_with("http://") || path.starts_with("https://") {
            return Some(path.to_string());
        }

        let base_url = Url::parse(base).ok()?;
        base_url.join(path).ok().map(|u| u.to_string())
    }
}

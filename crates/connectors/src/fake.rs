use core_domain::{ProductError, ProductErrorCode, SourceId};

use crate::url::{UrlImportPreview, UrlImportRequest};
use crate::web::{PublicWebSearch, WebSearchResult};

pub struct FakePublicWebSearch;

impl PublicWebSearch for FakePublicWebSearch {
    fn search(&self, query: &str, limit: usize) -> Result<Vec<WebSearchResult>, ProductError> {
        Ok((0..limit)
            .map(|i| WebSearchResult {
                title: format!("Resultado {i} para {query}"),
                url: format!("https://example.com/{i}"),
                snippet: format!("Snippet simulado {query}"),
            })
            .collect())
    }
}

pub struct FakeUrlImportGateway;

impl FakeUrlImportGateway {
    pub fn preview(&self, request: UrlImportRequest) -> Result<UrlImportPreview, ProductError> {
        let url = request.url.trim();
        if url.contains("127.0.0.1") || url.contains("localhost") {
            return Err(ProductError::new(
                ProductErrorCode::SsrBlocked,
                "URL de loopback bloqueada por política",
            ));
        }
        if !url.starts_with("https://") {
            return Err(ProductError::new(
                ProductErrorCode::ValidationFailed,
                "Solo URLs HTTPS públicas en el stub",
            ));
        }
        Ok(UrlImportPreview {
            source_id: SourceId(format!("src-{}", url.len())),
            title: format!("Vista previa: {url}"),
            byte_length_estimate: 1024,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_loopback_urls() {
        let gw = FakeUrlImportGateway;
        let err = gw
            .preview(UrlImportRequest {
                url: "http://127.0.0.1/secret".into(),
            })
            .unwrap_err();
        assert_eq!(err.code, ProductErrorCode::SsrBlocked);
    }

    #[test]
    fn fake_search_respects_limit() {
        let search = FakePublicWebSearch;
        let results = search.search("clima", 2).unwrap();
        assert_eq!(results.len(), 2);
    }
}

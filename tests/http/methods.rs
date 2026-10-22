use super::locales::{request, request_with_body};
use http::{Method, StatusCode, header};

#[tokio::test]
async fn pages_and_artifacts_only_accept_get_and_head() {
    for path in ["/", "/ja/changelog", "/docs/source/systemd.md", "/missing"] {
        for method in [
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::TRACE,
            Method::CONNECT,
        ] {
            let (status, headers, _) = request_with_body(method.clone(), path, "").await;
            assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED, "{method} {path}");
            assert_eq!(headers[header::ALLOW], "GET,HEAD", "{method} {path}");
        }
        let (get_status, get_headers, _) = request(path).await;
        let (head_status, head_headers, body) = request_with_body(Method::HEAD, path, "").await;
        assert_eq!(head_status, get_status, "{path}");
        assert_eq!(
            head_headers[header::CONTENT_TYPE],
            get_headers[header::CONTENT_TYPE]
        );
        assert!(body.is_empty(), "HEAD {path} must not return a body");
    }
}

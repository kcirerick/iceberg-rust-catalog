use futures::{future, future::BoxFuture, Stream, stream, future::FutureExt, stream::TryStreamExt};
use hyper::{Request, Response, StatusCode, Body, HeaderMap};
use hyper::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use log::warn;
#[allow(unused_imports)]
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::future::Future;
use std::marker::PhantomData;
use std::task::{Context, Poll};
use swagger::{ApiError, BodyExt, Has, RequestParser, XSpanIdString};
pub use swagger::auth::Authorization;
use swagger::auth::Scopes;
use url::form_urlencoded;

#[allow(unused_imports)]
use crate::models;
use crate::header;

pub use crate::context;

type ServiceFuture = BoxFuture<'static, Result<Response<Body>, crate::ServiceError>>;

use crate::{Api,
     CommitTransactionResponse,
     CreateNamespaceResponse,
     CreateTableResponse,
     CreateViewResponse,
     DropNamespaceResponse,
     DropTableResponse,
     DropViewResponse,
     ListNamespacesResponse,
     ListTablesResponse,
     ListViewsResponse,
     LoadNamespaceMetadataResponse,
     LoadTableResponse,
     LoadViewResponse,
     NamespaceExistsResponse,
     RegisterTableResponse,
     RenameTableResponse,
     RenameViewResponse,
     ReplaceViewResponse,
     ReportMetricsResponse,
     TableExistsResponse,
     UpdatePropertiesResponse,
     UpdateTableResponse,
     ViewExistsResponse,
     GetConfigResponse,
     GetTokenResponse
};

mod paths {
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref GLOBAL_REGEX_SET: regex::RegexSet = regex::RegexSet::new(vec![
            r"^/v1/config$",
            r"^/v1/oauth/tokens$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/properties$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/register$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables/(?P<table>[^/?#]*)$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables/(?P<table>[^/?#]*)/metrics$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/views$",
            r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/views/(?P<view>[^/?#]*)$",
            r"^/v1/(?P<prefix>[^/?#]*)/tables/rename$",
            r"^/v1/(?P<prefix>[^/?#]*)/transactions/commit$",
            r"^/v1/(?P<prefix>[^/?#]*)/views/rename$"
        ])
        .expect("Unable to create global regex set");
    }
    pub(crate) static ID_V1_CONFIG: usize = 0;
    pub(crate) static ID_V1_OAUTH_TOKENS: usize = 1;
    pub(crate) static ID_V1_PREFIX_NAMESPACES: usize = 2;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE: usize = 3;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES: usize = 4;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/properties$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER: usize = 5;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/register$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES: usize = 6;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_TABLES");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE: usize = 7;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables/(?P<table>[^/?#]*)$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS: usize = 8;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/tables/(?P<table>[^/?#]*)/metrics$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS: usize = 9;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/views$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS");
    }
    pub(crate) static ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW: usize = 10;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/namespaces/(?P<namespace>[^/?#]*)/views/(?P<view>[^/?#]*)$")
                .expect("Unable to create regex for V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW");
    }
    pub(crate) static ID_V1_PREFIX_TABLES_RENAME: usize = 11;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_TABLES_RENAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/tables/rename$")
                .expect("Unable to create regex for V1_PREFIX_TABLES_RENAME");
    }
    pub(crate) static ID_V1_PREFIX_TRANSACTIONS_COMMIT: usize = 12;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_TRANSACTIONS_COMMIT: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/transactions/commit$")
                .expect("Unable to create regex for V1_PREFIX_TRANSACTIONS_COMMIT");
    }
    pub(crate) static ID_V1_PREFIX_VIEWS_RENAME: usize = 13;
    lazy_static! {
        pub static ref REGEX_V1_PREFIX_VIEWS_RENAME: regex::Regex =
            #[allow(clippy::invalid_regex)]
            regex::Regex::new(r"^/v1/(?P<prefix>[^/?#]*)/views/rename$")
                .expect("Unable to create regex for V1_PREFIX_VIEWS_RENAME");
    }
}

pub struct MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        MakeService {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C, Target> hyper::service::Service<Target> for MakeService<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Service<T, C>;
    type Error = crate::ServiceError;
    type Future = future::Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, target: Target) -> Self::Future {
        futures::future::ok(Service::new(
            self.api_impl.clone(),
        ))
    }
}

fn method_not_allowed() -> Result<Response<Body>, crate::ServiceError> {
    Ok(
        Response::builder().status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .expect("Unable to create Method Not Allowed response")
    )
}

pub struct Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    api_impl: T,
    marker: PhantomData<C>,
}

impl<T, C> Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    pub fn new(api_impl: T) -> Self {
        Service {
            api_impl,
            marker: PhantomData
        }
    }
}

impl<T, C> Clone for Service<T, C> where
    T: Api<C> + Clone + Send + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    fn clone(&self) -> Self {
        Service {
            api_impl: self.api_impl.clone(),
            marker: self.marker,
        }
    }
}

impl<T, C> hyper::service::Service<(Request<Body>, C)> for Service<T, C> where
    T: Api<C> + Clone + Send + Sync + 'static,
    C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
{
    type Response = Response<Body>;
    type Error = crate::ServiceError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.api_impl.poll_ready(cx)
    }

    fn call(&mut self, req: (Request<Body>, C)) -> Self::Future { async fn run<T, C>(mut api_impl: T, req: (Request<Body>, C)) -> Result<Response<Body>, crate::ServiceError> where
        T: Api<C> + Clone + Send + 'static,
        C: Has<XSpanIdString> + Has<Option<Authorization>> + Send + Sync + 'static
    {
        let (request, context) = req;
        let (parts, body) = request.into_parts();
        let (method, uri, headers) = (parts.method, parts.uri, parts.headers);
        let path = paths::GLOBAL_REGEX_SET.matches(uri.path());

        match method {

            // CommitTransaction - POST /v1/{prefix}/transactions/commit
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_TRANSACTIONS_COMMIT) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_TRANSACTIONS_COMMIT
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_TRANSACTIONS_COMMIT in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_TRANSACTIONS_COMMIT.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_commit_transaction_request: Option<models::CommitTransactionRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_commit_transaction_request) => param_commit_transaction_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter CommitTransactionRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter CommitTransactionRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_commit_transaction_request = match param_commit_transaction_request {
                                    Some(param_commit_transaction_request) => param_commit_transaction_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter CommitTransactionRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter CommitTransactionRequest")),
                                };

                                let result = api_impl.commit_transaction(
                                            param_prefix,
                                            param_commit_transaction_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CommitTransactionResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                CommitTransactionResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::AnUnknownServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_AN_UNKNOWN_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_A_GATEWAY_OR_PROXY_RECEIVED_AN_INVALID_RESPONSE_FROM_THE_UPSTREAM_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(504).expect("Unable to turn 504 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CommitTransactionResponse::AServer_2
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for COMMIT_TRANSACTION_A_SERVER_2"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CommitTransactionRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CommitTransactionRequest")),
                        }
            },

            // CreateNamespace - POST /v1/{prefix}/namespaces
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_create_namespace_request: Option<models::CreateNamespaceRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_create_namespace_request) => param_create_namespace_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_namespace(
                                            param_prefix,
                                            param_create_namespace_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateNamespaceResponse::RepresentsASuccessfulCallToCreateANamespace
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_REPRESENTS_A_SUCCESSFUL_CALL_TO_CREATE_A_NAMESPACE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::NotAcceptable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(406).expect("Unable to turn 406 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_NOT_ACCEPTABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateNamespaceResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_NAMESPACE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CreateNamespaceRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CreateNamespaceRequest")),
                        }
            },

            // CreateTable - POST /v1/{prefix}/namespaces/{namespace}/tables
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Header parameters
                let param_x_iceberg_access_delegation = headers.get(HeaderName::from_static("x-iceberg-access-delegation"));

                let param_x_iceberg_access_delegation = match param_x_iceberg_access_delegation {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Iceberg-Access-Delegation - {}", err)))
                                        .expect("Unable to create Bad Request response for invalid header X-Iceberg-Access-Delegation"));

                        },
                    },
                    None => {
                        None
                    }
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_create_table_request: Option<models::CreateTableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_create_table_request) => param_create_table_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_table(
                                            param_prefix,
                                            param_namespace,
                                            param_x_iceberg_access_delegation,
                                            param_create_table_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateTableResponse::TableMetadataResultAfterCreatingATable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_TABLE_METADATA_RESULT_AFTER_CREATING_A_TABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CreateTableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CreateTableRequest")),
                        }
            },

            // CreateView - POST /v1/{prefix}/namespaces/{namespace}/views
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_create_view_request: Option<models::CreateViewRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_create_view_request) => param_create_view_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.create_view(
                                            param_prefix,
                                            param_namespace,
                                            param_create_view_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                CreateViewResponse::ViewMetadataResultWhenLoadingAView
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_VIEW_METADATA_RESULT_WHEN_LOADING_A_VIEW"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                CreateViewResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for CREATE_VIEW_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CreateViewRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CreateViewRequest")),
                        }
            },

            // DropNamespace - DELETE /v1/{prefix}/namespaces/{namespace}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.drop_namespace(
                                            param_prefix,
                                            param_namespace,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DropNamespaceResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                DropNamespaceResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropNamespaceResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_NAMESPACE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DropTable - DELETE /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_table = match percent_encoding::percent_decode(path_params["table"].as_bytes()).decode_utf8() {
                    Ok(param_table) => match param_table.parse::<String>() {
                        Ok(param_table) => param_table,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter table: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["table"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_purge_requested = query_params.iter().filter(|e| e.0 == "purgeRequested").map(|e| e.1.clone())
                    .next();
                let param_purge_requested = match param_purge_requested {
                    Some(param_purge_requested) => {
                        let param_purge_requested =
                            <bool as std::str::FromStr>::from_str
                                (&param_purge_requested);
                        match param_purge_requested {
                            Ok(param_purge_requested) => Some(param_purge_requested),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter purgeRequested - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter purgeRequested")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.drop_table(
                                            param_prefix,
                                            param_namespace,
                                            param_table,
                                            param_purge_requested,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DropTableResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                DropTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // DropView - DELETE /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_view = match percent_encoding::percent_decode(path_params["view"].as_bytes()).decode_utf8() {
                    Ok(param_view) => match param_view.parse::<String>() {
                        Ok(param_view) => param_view,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter view: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["view"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.drop_view(
                                            param_prefix,
                                            param_namespace,
                                            param_view,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                DropViewResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                DropViewResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                DropViewResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for DROP_VIEW_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ListNamespaces - GET /v1/{prefix}/namespaces
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page_token = query_params.iter().filter(|e| e.0 == "pageToken").map(|e| e.1.clone())
                    .next();
                let param_page_token = match param_page_token {
                    Some(param_page_token) => {
                        let param_page_token =
                            <String as std::str::FromStr>::from_str
                                (&param_page_token);
                        match param_page_token {
                            Ok(param_page_token) => Some(param_page_token),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageToken - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageToken")),
                        }
                    },
                    None => None,
                };
                let param_page_size = query_params.iter().filter(|e| e.0 == "pageSize").map(|e| e.1.clone())
                    .next();
                let param_page_size = match param_page_size {
                    Some(param_page_size) => {
                        let param_page_size =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page_size);
                        match param_page_size {
                            Ok(param_page_size) => Some(param_page_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageSize - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageSize")),
                        }
                    },
                    None => None,
                };
                let param_parent = query_params.iter().filter(|e| e.0 == "parent").map(|e| e.1.clone())
                    .next();
                let param_parent = match param_parent {
                    Some(param_parent) => {
                        let param_parent =
                            <String as std::str::FromStr>::from_str
                                (&param_parent);
                        match param_parent {
                            Ok(param_parent) => Some(param_parent),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter parent - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter parent")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.list_namespaces(
                                            param_prefix,
                                            param_page_token,
                                            param_page_size,
                                            param_parent,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ListNamespacesResponse::AListOfNamespaces
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_A_LIST_OF_NAMESPACES"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListNamespacesResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_NAMESPACES_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ListTables - GET /v1/{prefix}/namespaces/{namespace}/tables
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page_token = query_params.iter().filter(|e| e.0 == "pageToken").map(|e| e.1.clone())
                    .next();
                let param_page_token = match param_page_token {
                    Some(param_page_token) => {
                        let param_page_token =
                            <String as std::str::FromStr>::from_str
                                (&param_page_token);
                        match param_page_token {
                            Ok(param_page_token) => Some(param_page_token),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageToken - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageToken")),
                        }
                    },
                    None => None,
                };
                let param_page_size = query_params.iter().filter(|e| e.0 == "pageSize").map(|e| e.1.clone())
                    .next();
                let param_page_size = match param_page_size {
                    Some(param_page_size) => {
                        let param_page_size =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page_size);
                        match param_page_size {
                            Ok(param_page_size) => Some(param_page_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageSize - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageSize")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.list_tables(
                                            param_prefix,
                                            param_namespace,
                                            param_page_token,
                                            param_page_size,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ListTablesResponse::AListOfTableIdentifiers
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_A_LIST_OF_TABLE_IDENTIFIERS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListTablesResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_TABLES_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // ListViews - GET /v1/{prefix}/namespaces/{namespace}/views
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_page_token = query_params.iter().filter(|e| e.0 == "pageToken").map(|e| e.1.clone())
                    .next();
                let param_page_token = match param_page_token {
                    Some(param_page_token) => {
                        let param_page_token =
                            <String as std::str::FromStr>::from_str
                                (&param_page_token);
                        match param_page_token {
                            Ok(param_page_token) => Some(param_page_token),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageToken - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageToken")),
                        }
                    },
                    None => None,
                };
                let param_page_size = query_params.iter().filter(|e| e.0 == "pageSize").map(|e| e.1.clone())
                    .next();
                let param_page_size = match param_page_size {
                    Some(param_page_size) => {
                        let param_page_size =
                            <i32 as std::str::FromStr>::from_str
                                (&param_page_size);
                        match param_page_size {
                            Ok(param_page_size) => Some(param_page_size),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter pageSize - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter pageSize")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.list_views(
                                            param_prefix,
                                            param_namespace,
                                            param_page_token,
                                            param_page_size,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ListViewsResponse::AListOfTableIdentifiers
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_A_LIST_OF_TABLE_IDENTIFIERS"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ListViewsResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LIST_VIEWS_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // LoadNamespaceMetadata - GET /v1/{prefix}/namespaces/{namespace}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.load_namespace_metadata(
                                            param_prefix,
                                            param_namespace,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoadNamespaceMetadataResponse::ReturnsANamespace
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_RETURNS_A_NAMESPACE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadNamespaceMetadataResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_NAMESPACE_METADATA_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // LoadTable - GET /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_table = match percent_encoding::percent_decode(path_params["table"].as_bytes()).decode_utf8() {
                    Ok(param_table) => match param_table.parse::<String>() {
                        Ok(param_table) => param_table,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter table: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["table"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Header parameters
                let param_x_iceberg_access_delegation = headers.get(HeaderName::from_static("x-iceberg-access-delegation"));

                let param_x_iceberg_access_delegation = match param_x_iceberg_access_delegation {
                    Some(v) => match header::IntoHeaderValue::<String>::try_from((*v).clone()) {
                        Ok(result) =>
                            Some(result.0),
                        Err(err) => {
                            return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Invalid header X-Iceberg-Access-Delegation - {}", err)))
                                        .expect("Unable to create Bad Request response for invalid header X-Iceberg-Access-Delegation"));

                        },
                    },
                    None => {
                        None
                    }
                };

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_snapshots = query_params.iter().filter(|e| e.0 == "snapshots").map(|e| e.1.clone())
                    .next();
                let param_snapshots = match param_snapshots {
                    Some(param_snapshots) => {
                        let param_snapshots =
                            <String as std::str::FromStr>::from_str
                                (&param_snapshots);
                        match param_snapshots {
                            Ok(param_snapshots) => Some(param_snapshots),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter snapshots - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter snapshots")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.load_table(
                                            param_prefix,
                                            param_namespace,
                                            param_table,
                                            param_x_iceberg_access_delegation,
                                            param_snapshots,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoadTableResponse::TableMetadataResultWhenLoadingATable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_TABLE_METADATA_RESULT_WHEN_LOADING_A_TABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // LoadView - GET /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_view = match percent_encoding::percent_decode(path_params["view"].as_bytes()).decode_utf8() {
                    Ok(param_view) => match param_view.parse::<String>() {
                        Ok(param_view) => param_view,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter view: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["view"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.load_view(
                                            param_prefix,
                                            param_namespace,
                                            param_view,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                LoadViewResponse::ViewMetadataResultWhenLoadingAView
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_VIEW_METADATA_RESULT_WHEN_LOADING_A_VIEW"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                LoadViewResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for LOAD_VIEW_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // NamespaceExists - HEAD /v1/{prefix}/namespaces/{namespace}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.namespace_exists(
                                            param_prefix,
                                            param_namespace,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                NamespaceExistsResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                NamespaceExistsResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                NamespaceExistsResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for NAMESPACE_EXISTS_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // RegisterTable - POST /v1/{prefix}/namespaces/{namespace}/register
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_register_table_request: Option<models::RegisterTableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_register_table_request) => param_register_table_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.register_table(
                                            param_prefix,
                                            param_namespace,
                                            param_register_table_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RegisterTableResponse::TableMetadataResultWhenLoadingATable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_TABLE_METADATA_RESULT_WHEN_LOADING_A_TABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RegisterTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REGISTER_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RegisterTableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RegisterTableRequest")),
                        }
            },

            // RenameTable - POST /v1/{prefix}/tables/rename
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_TABLES_RENAME) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_TABLES_RENAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_TABLES_RENAME in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_TABLES_RENAME.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_rename_table_request: Option<models::RenameTableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_rename_table_request) => param_rename_table_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter RenameTableRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter RenameTableRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_rename_table_request = match param_rename_table_request {
                                    Some(param_rename_table_request) => param_rename_table_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter RenameTableRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter RenameTableRequest")),
                                };

                                let result = api_impl.rename_table(
                                            param_prefix,
                                            param_rename_table_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RenameTableResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                RenameTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::NotAcceptable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(406).expect("Unable to turn 406 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_NOT_ACCEPTABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RenameTableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RenameTableRequest")),
                        }
            },

            // RenameView - POST /v1/{prefix}/views/rename
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_VIEWS_RENAME) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_VIEWS_RENAME
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_VIEWS_RENAME in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_VIEWS_RENAME.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_rename_table_request: Option<models::RenameTableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_rename_table_request) => param_rename_table_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter RenameTableRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter RenameTableRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_rename_table_request = match param_rename_table_request {
                                    Some(param_rename_table_request) => param_rename_table_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter RenameTableRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter RenameTableRequest")),
                                };

                                let result = api_impl.rename_view(
                                            param_prefix,
                                            param_rename_table_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                RenameViewResponse::OK
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                },
                                                RenameViewResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::NotAcceptable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(406).expect("Unable to turn 406 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_NOT_ACCEPTABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                RenameViewResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for RENAME_VIEW_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter RenameTableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter RenameTableRequest")),
                        }
            },

            // ReplaceView - POST /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_view = match percent_encoding::percent_decode(path_params["view"].as_bytes()).decode_utf8() {
                    Ok(param_view) => match param_view.parse::<String>() {
                        Ok(param_view) => param_view,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter view: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["view"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_commit_view_request: Option<models::CommitViewRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_commit_view_request) => param_commit_view_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.replace_view(
                                            param_prefix,
                                            param_namespace,
                                            param_view,
                                            param_commit_view_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ReplaceViewResponse::ViewMetadataResultWhenLoadingAView
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_VIEW_METADATA_RESULT_WHEN_LOADING_A_VIEW"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::AnUnknownServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_AN_UNKNOWN_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_A_GATEWAY_OR_PROXY_RECEIVED_AN_INVALID_RESPONSE_FROM_THE_UPSTREAM_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(504).expect("Unable to turn 504 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReplaceViewResponse::AServer_2
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPLACE_VIEW_A_SERVER_2"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CommitViewRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CommitViewRequest")),
                        }
            },

            // ReportMetrics - POST /v1/{prefix}/namespaces/{namespace}/tables/{table}/metrics
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_table = match percent_encoding::percent_decode(path_params["table"].as_bytes()).decode_utf8() {
                    Ok(param_table) => match param_table.parse::<String>() {
                        Ok(param_table) => param_table,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter table: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["table"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_report_metrics_request: Option<models::ReportMetricsRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_report_metrics_request) => param_report_metrics_request,
                                        Err(e) => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from(format!("Couldn't parse body parameter ReportMetricsRequest - doesn't match schema: {}", e)))
                                                        .expect("Unable to create Bad Request response for invalid body parameter ReportMetricsRequest due to schema")),
                                    }
                                } else {
                                    None
                                };
                                let param_report_metrics_request = match param_report_metrics_request {
                                    Some(param_report_metrics_request) => param_report_metrics_request,
                                    None => return Ok(Response::builder()
                                                        .status(StatusCode::BAD_REQUEST)
                                                        .body(Body::from("Missing required body parameter ReportMetricsRequest"))
                                                        .expect("Unable to create Bad Request response for missing body parameter ReportMetricsRequest")),
                                };

                                let result = api_impl.report_metrics(
                                            param_prefix,
                                            param_namespace,
                                            param_table,
                                            param_report_metrics_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ReportMetricsResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                ReportMetricsResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ReportMetricsResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for REPORT_METRICS_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter ReportMetricsRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter ReportMetricsRequest")),
                        }
            },

            // TableExists - HEAD /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_table = match percent_encoding::percent_decode(path_params["table"].as_bytes()).decode_utf8() {
                    Ok(param_table) => match param_table.parse::<String>() {
                        Ok(param_table) => param_table,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter table: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["table"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.table_exists(
                                            param_prefix,
                                            param_namespace,
                                            param_table,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                TableExistsResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                TableExistsResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                TableExistsResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for TABLE_EXISTS_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // UpdateProperties - POST /v1/{prefix}/namespaces/{namespace}/properties
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_update_namespace_properties_request: Option<models::UpdateNamespacePropertiesRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_update_namespace_properties_request) => param_update_namespace_properties_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_properties(
                                            param_prefix,
                                            param_namespace,
                                            param_update_namespace_properties_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdatePropertiesResponse::JSONDataResponseForASynchronousUpdatePropertiesRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_JSON_DATA_RESPONSE_FOR_A_SYNCHRONOUS_UPDATE_PROPERTIES_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::NotAcceptable
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(406).expect("Unable to turn 406 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_NOT_ACCEPTABLE"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::UnprocessableEntity
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(422).expect("Unable to turn 422 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_UNPROCESSABLE_ENTITY"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdatePropertiesResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_PROPERTIES_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter UpdateNamespacePropertiesRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter UpdateNamespacePropertiesRequest")),
                        }
            },

            // UpdateTable - POST /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_table = match percent_encoding::percent_decode(path_params["table"].as_bytes()).decode_utf8() {
                    Ok(param_table) => match param_table.parse::<String>() {
                        Ok(param_table) => param_table,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter table: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["table"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                // Body parameters (note that non-required body parameters will ignore garbage
                // values, rather than causing a 400 response). Produce warning header and logs for
                // any unused fields.
                let result = body.into_raw().await;
                match result {
                            Ok(body) => {
                                let mut unused_elements = Vec::new();
                                let param_commit_table_request: Option<models::CommitTableRequest> = if !body.is_empty() {
                                    let deserializer = &mut serde_json::Deserializer::from_slice(&body);
                                    match serde_ignored::deserialize(deserializer, |path| {
                                            warn!("Ignoring unknown field in body: {}", path);
                                            unused_elements.push(path.to_string());
                                    }) {
                                        Ok(param_commit_table_request) => param_commit_table_request,
                                        Err(_) => None,
                                    }
                                } else {
                                    None
                                };

                                let result = api_impl.update_table(
                                            param_prefix,
                                            param_namespace,
                                            param_table,
                                            param_commit_table_request,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        if !unused_elements.is_empty() {
                                            response.headers_mut().insert(
                                                HeaderName::from_static("warning"),
                                                HeaderValue::from_str(format!("Ignoring unknown fields in body: {:?}", unused_elements).as_str())
                                                    .expect("Unable to create Warning header value"));
                                        }

                                        match result {
                                            Ok(rsp) => match rsp {
                                                UpdateTableResponse::ResponseUsedWhenATableIsSuccessfullyUpdated
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_RESPONSE_USED_WHEN_A_TABLE_IS_SUCCESSFULLY_UPDATED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::NotFound
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_NOT_FOUND"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::Conflict
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(409).expect("Unable to turn 409 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_CONFLICT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::AnUnknownServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(500).expect("Unable to turn 500 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_AN_UNKNOWN_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(502).expect("Unable to turn 502 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_A_GATEWAY_OR_PROXY_RECEIVED_AN_INVALID_RESPONSE_FROM_THE_UPSTREAM_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(504).expect("Unable to turn 504 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                UpdateTableResponse::AServer_2
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for UPDATE_TABLE_A_SERVER_2"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
                            },
                            Err(e) => Ok(Response::builder()
                                                .status(StatusCode::BAD_REQUEST)
                                                .body(Body::from(format!("Couldn't read body parameter CommitTableRequest: {}", e)))
                                                .expect("Unable to create Bad Request response due to unable to read body parameter CommitTableRequest")),
                        }
            },

            // ViewExists - HEAD /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Path parameters
                let path: &str = uri.path();
                let path_params =
                    paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW
                    .captures(path)
                    .unwrap_or_else(||
                        panic!("Path {} matched RE V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW in set but failed match against \"{}\"", path, paths::REGEX_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW.as_str())
                    );

                let param_prefix = match percent_encoding::percent_decode(path_params["prefix"].as_bytes()).decode_utf8() {
                    Ok(param_prefix) => match param_prefix.parse::<String>() {
                        Ok(param_prefix) => param_prefix,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter prefix: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["prefix"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_namespace = match percent_encoding::percent_decode(path_params["namespace"].as_bytes()).decode_utf8() {
                    Ok(param_namespace) => match param_namespace.parse::<String>() {
                        Ok(param_namespace) => param_namespace,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter namespace: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["namespace"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                let param_view = match percent_encoding::percent_decode(path_params["view"].as_bytes()).decode_utf8() {
                    Ok(param_view) => match param_view.parse::<String>() {
                        Ok(param_view) => param_view,
                        Err(e) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't parse path parameter view: {}", e)))
                                        .expect("Unable to create Bad Request response for invalid path parameter")),
                    },
                    Err(_) => return Ok(Response::builder()
                                        .status(StatusCode::BAD_REQUEST)
                                        .body(Body::from(format!("Couldn't percent-decode path parameter as UTF-8: {}", &path_params["view"])))
                                        .expect("Unable to create Bad Request response for invalid percent decode"))
                };

                                let result = api_impl.view_exists(
                                            param_prefix,
                                            param_namespace,
                                            param_view,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                ViewExistsResponse::Success
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(204).expect("Unable to turn 204 into a StatusCode");
                                                },
                                                ViewExistsResponse::BadRequest
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                },
                                                ViewExistsResponse::Unauthorized
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                },
                                                ViewExistsResponse::NotFound
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(404).expect("Unable to turn 404 into a StatusCode");
                                                },
                                                ViewExistsResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for VIEW_EXISTS_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ViewExistsResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for VIEW_EXISTS_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                ViewExistsResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for VIEW_EXISTS_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetConfig - GET /v1/config
            hyper::Method::GET if path.matched(paths::ID_V1_CONFIG) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                // Query parameters (note that non-required or collection query parameters will ignore garbage values, rather than causing a 400 response)
                let query_params = form_urlencoded::parse(uri.query().unwrap_or_default().as_bytes()).collect::<Vec<_>>();
                let param_warehouse = query_params.iter().filter(|e| e.0 == "warehouse").map(|e| e.1.clone())
                    .next();
                let param_warehouse = match param_warehouse {
                    Some(param_warehouse) => {
                        let param_warehouse =
                            <String as std::str::FromStr>::from_str
                                (&param_warehouse);
                        match param_warehouse {
                            Ok(param_warehouse) => Some(param_warehouse),
                            Err(e) => return Ok(Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body(Body::from(format!("Couldn't parse query parameter warehouse - doesn't match schema: {}", e)))
                                .expect("Unable to create Bad Request response for invalid query parameter warehouse")),
                        }
                    },
                    None => None,
                };

                                let result = api_impl.get_config(
                                            param_warehouse,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetConfigResponse::ServerSpecifiedConfigurationValues
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_SERVER_SPECIFIED_CONFIGURATION_VALUES"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::IndicatesABadRequestError
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_INDICATES_A_BAD_REQUEST_ERROR"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::Unauthorized
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_UNAUTHORIZED"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::Forbidden
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(403).expect("Unable to turn 403 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_FORBIDDEN"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::CredentialsHaveTimedOut
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(419).expect("Unable to turn 419 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_CREDENTIALS_HAVE_TIMED_OUT"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::TheServiceIsNotReadyToHandleTheRequest
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(503).expect("Unable to turn 503 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_THE_SERVICE_IS_NOT_READY_TO_HANDLE_THE_REQUEST"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetConfigResponse::AServer
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_CONFIG_A_SERVER"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            // GetToken - POST /v1/oauth/tokens
            hyper::Method::POST if path.matched(paths::ID_V1_OAUTH_TOKENS) => {
                {
                    let authorization = match *(&context as &dyn Has<Option<Authorization>>).get() {
                        Some(ref authorization) => authorization,
                        None => return Ok(Response::builder()
                                                .status(StatusCode::FORBIDDEN)
                                                .body(Body::from("Unauthenticated"))
                                                .expect("Unable to create Authentication Forbidden response")),
                    };

                    // Authorization
                    if let Scopes::Some(ref scopes) = authorization.scopes {
                        let required_scopes: std::collections::BTreeSet<String> = vec![
                            "catalog".to_string(), // Allows interacting with the Config and Catalog APIs
                        ].into_iter().collect();

                        if !required_scopes.is_subset(scopes) {
                            let missing_scopes = required_scopes.difference(scopes);
                            return Ok(Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(missing_scopes.fold(
                                    "Insufficient authorization, missing scopes".to_string(),
                                    |s, scope| format!("{} {}", s, scope))
                                ))
                                .expect("Unable to create Authentication Insufficient response")
                            );
                        }
                    }
                }

                                // Form parameters
                                let param_grant_type = Some("grant_type_example".to_string());
                                let param_scope = Some("scope_example".to_string());
                                let param_client_id = Some("client_id_example".to_string());
                                let param_client_secret = Some("client_secret_example".to_string());
                                let param_requested_token_type = None;
                                let param_subject_token = Some("subject_token_example".to_string());
                                let param_subject_token_type = None;
                                let param_actor_token = Some("actor_token_example".to_string());
                                let param_actor_token_type = None;

                                let result = api_impl.get_token(
                                            param_grant_type,
                                            param_scope,
                                            param_client_id,
                                            param_client_secret,
                                            param_requested_token_type,
                                            param_subject_token,
                                            param_subject_token_type,
                                            param_actor_token,
                                            param_actor_token_type,
                                        &context
                                    ).await;
                                let mut response = Response::new(Body::empty());
                                response.headers_mut().insert(
                                            HeaderName::from_static("x-span-id"),
                                            HeaderValue::from_str((&context as &dyn Has<XSpanIdString>).get().0.clone().as_str())
                                                .expect("Unable to create X-Span-ID header value"));

                                        match result {
                                            Ok(rsp) => match rsp {
                                                GetTokenResponse::OAuth
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(200).expect("Unable to turn 200 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_TOKEN_O_AUTH"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetTokenResponse::OAuth_2
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(400).expect("Unable to turn 400 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_TOKEN_O_AUTH_2"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetTokenResponse::OAuth_3
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(401).expect("Unable to turn 401 into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_TOKEN_O_AUTH_3"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                                GetTokenResponse::OAuth_4
                                                    (body)
                                                => {
                                                    *response.status_mut() = StatusCode::from_u16(5XX).expect("Unable to turn 5XX into a StatusCode");
                                                    response.headers_mut().insert(
                                                        CONTENT_TYPE,
                                                        HeaderValue::from_str("application/json")
                                                            .expect("Unable to create Content-Type header for GET_TOKEN_O_AUTH_4"));
                                                    let body_content = serde_json::to_string(&body).expect("impossible to fail to serialize");
                                                    *response.body_mut() = Body::from(body_content);
                                                },
                                            },
                                            Err(_) => {
                                                // Application code returned an error. This should not happen, as the implementation should
                                                // return a valid response.
                                                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                                                *response.body_mut() = Body::from("An internal error occurred");
                                            },
                                        }

                                        Ok(response)
            },

            _ if path.matched(paths::ID_V1_CONFIG) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_OAUTH_TOKENS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_TABLES_RENAME) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_TRANSACTIONS_COMMIT) => method_not_allowed(),
            _ if path.matched(paths::ID_V1_PREFIX_VIEWS_RENAME) => method_not_allowed(),
            _ => Ok(Response::builder().status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Unable to create Not Found response"))
        }
    } Box::pin(run(self.api_impl.clone(), req)) }
}

/// Request parser for `Api`.
pub struct ApiRequestParser;
impl<T> RequestParser<T> for ApiRequestParser {
    fn parse_operation_id(request: &Request<T>) -> Option<&'static str> {
        let path = paths::GLOBAL_REGEX_SET.matches(request.uri().path());
        match *request.method() {
            // CommitTransaction - POST /v1/{prefix}/transactions/commit
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_TRANSACTIONS_COMMIT) => Some("CommitTransaction"),
            // CreateNamespace - POST /v1/{prefix}/namespaces
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES) => Some("CreateNamespace"),
            // CreateTable - POST /v1/{prefix}/namespaces/{namespace}/tables
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES) => Some("CreateTable"),
            // CreateView - POST /v1/{prefix}/namespaces/{namespace}/views
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS) => Some("CreateView"),
            // DropNamespace - DELETE /v1/{prefix}/namespaces/{namespace}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => Some("DropNamespace"),
            // DropTable - DELETE /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => Some("DropTable"),
            // DropView - DELETE /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::DELETE if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => Some("DropView"),
            // ListNamespaces - GET /v1/{prefix}/namespaces
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES) => Some("ListNamespaces"),
            // ListTables - GET /v1/{prefix}/namespaces/{namespace}/tables
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES) => Some("ListTables"),
            // ListViews - GET /v1/{prefix}/namespaces/{namespace}/views
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS) => Some("ListViews"),
            // LoadNamespaceMetadata - GET /v1/{prefix}/namespaces/{namespace}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => Some("LoadNamespaceMetadata"),
            // LoadTable - GET /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => Some("LoadTable"),
            // LoadView - GET /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::GET if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => Some("LoadView"),
            // NamespaceExists - HEAD /v1/{prefix}/namespaces/{namespace}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE) => Some("NamespaceExists"),
            // RegisterTable - POST /v1/{prefix}/namespaces/{namespace}/register
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_REGISTER) => Some("RegisterTable"),
            // RenameTable - POST /v1/{prefix}/tables/rename
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_TABLES_RENAME) => Some("RenameTable"),
            // RenameView - POST /v1/{prefix}/views/rename
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_VIEWS_RENAME) => Some("RenameView"),
            // ReplaceView - POST /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => Some("ReplaceView"),
            // ReportMetrics - POST /v1/{prefix}/namespaces/{namespace}/tables/{table}/metrics
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE_METRICS) => Some("ReportMetrics"),
            // TableExists - HEAD /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => Some("TableExists"),
            // UpdateProperties - POST /v1/{prefix}/namespaces/{namespace}/properties
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_PROPERTIES) => Some("UpdateProperties"),
            // UpdateTable - POST /v1/{prefix}/namespaces/{namespace}/tables/{table}
            hyper::Method::POST if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_TABLES_TABLE) => Some("UpdateTable"),
            // ViewExists - HEAD /v1/{prefix}/namespaces/{namespace}/views/{view}
            hyper::Method::HEAD if path.matched(paths::ID_V1_PREFIX_NAMESPACES_NAMESPACE_VIEWS_VIEW) => Some("ViewExists"),
            // GetConfig - GET /v1/config
            hyper::Method::GET if path.matched(paths::ID_V1_CONFIG) => Some("GetConfig"),
            // GetToken - POST /v1/oauth/tokens
            hyper::Method::POST if path.matched(paths::ID_V1_OAUTH_TOKENS) => Some("GetToken"),
            _ => None,
        }
    }
}

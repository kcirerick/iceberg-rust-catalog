//! Main library entry point for openapi_client implementation.

#![allow(unused_imports)]

use async_trait::async_trait;
use futures::{future, Stream, StreamExt, TryFutureExt, TryStreamExt};
use hyper::server::conn::Http;
use hyper::service::Service;
use log::info;
use std::future::Future;
use std::marker::PhantomData;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use swagger::{Has, XSpanIdString};
use swagger::auth::MakeAllowAllAuthenticator;
use swagger::EmptyContext;
use tokio::net::TcpListener;

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
use openssl::ssl::{Ssl, SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

use openapi_client::models;

/// Builds an SSL implementation for Simple HTTPS from some hard-coded file names
pub async fn create(addr: &str, https: bool) {
    let addr = addr.parse().expect("Failed to parse bind address");

    let server = Server::new();

    let service = MakeService::new(server);

    let service = MakeAllowAllAuthenticator::new(service, "cosmo");

    #[allow(unused_mut)]
    let mut service =
        openapi_client::server::context::MakeAddContext::<_, EmptyContext>::new(
            service
        );

    if https {
        #[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
        {
            unimplemented!("SSL is not implemented for the examples on MacOS, Windows or iOS");
        }

        #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "ios")))]
        {
            let mut ssl = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).expect("Failed to create SSL Acceptor");

            // Server authentication
            ssl.set_private_key_file("examples/server-key.pem", SslFiletype::PEM).expect("Failed to set private key");
            ssl.set_certificate_chain_file("examples/server-chain.pem").expect("Failed to set certificate chain");
            ssl.check_private_key().expect("Failed to check private key");

            let tls_acceptor = ssl.build();
            let tcp_listener = TcpListener::bind(&addr).await.unwrap();

            loop {
                if let Ok((tcp, _)) = tcp_listener.accept().await {
                    let ssl = Ssl::new(tls_acceptor.context()).unwrap();
                    let addr = tcp.peer_addr().expect("Unable to get remote address");
                    let service = service.call(addr);

                    tokio::spawn(async move {
                        let tls = tokio_openssl::SslStream::new(ssl, tcp).map_err(|_| ())?;
                        let service = service.await.map_err(|_| ())?;

                        Http::new()
                            .serve_connection(tls, service)
                            .await
                            .map_err(|_| ())
                    });
                }
            }
        }
    } else {
        // Using HTTP
        hyper::server::Server::bind(&addr).serve(service).await.unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct Server<C> {
    marker: PhantomData<C>,
}

impl<C> Server<C> {
    pub fn new() -> Self {
        Server{marker: PhantomData}
    }
}


use openapi_client::{
    Api,
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
    GetTokenResponse,
};
use openapi_client::server::MakeService;
use std::error::Error;
use swagger::ApiError;

#[async_trait]
impl<C> Api<C> for Server<C> where C: Has<XSpanIdString> + Send + Sync
{
    /// Commit updates to multiple tables in an atomic operation
    async fn commit_transaction(
        &self,
        prefix: String,
        commit_transaction_request: models::CommitTransactionRequest,
        context: &C) -> Result<CommitTransactionResponse, ApiError>
    {
        info!("commit_transaction(\"{}\", {:?}) - X-Span-ID: {:?}", prefix, commit_transaction_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create a namespace
    async fn create_namespace(
        &self,
        prefix: String,
        create_namespace_request: Option<models::CreateNamespaceRequest>,
        context: &C) -> Result<CreateNamespaceResponse, ApiError>
    {
        info!("create_namespace(\"{}\", {:?}) - X-Span-ID: {:?}", prefix, create_namespace_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create a table in the given namespace
    async fn create_table(
        &self,
        prefix: String,
        namespace: String,
        x_iceberg_access_delegation: Option<String>,
        create_table_request: Option<models::CreateTableRequest>,
        context: &C) -> Result<CreateTableResponse, ApiError>
    {
        info!("create_table(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", prefix, namespace, x_iceberg_access_delegation, create_table_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Create a view in the given namespace
    async fn create_view(
        &self,
        prefix: String,
        namespace: String,
        create_view_request: Option<models::CreateViewRequest>,
        context: &C) -> Result<CreateViewResponse, ApiError>
    {
        info!("create_view(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, create_view_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Drop a namespace from the catalog. Namespace must be empty.
    async fn drop_namespace(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<DropNamespaceResponse, ApiError>
    {
        info!("drop_namespace(\"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Drop a table from the catalog
    async fn drop_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        purge_requested: Option<bool>,
        context: &C) -> Result<DropTableResponse, ApiError>
    {
        info!("drop_table(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, table, purge_requested, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Drop a view from the catalog
    async fn drop_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<DropViewResponse, ApiError>
    {
        info!("drop_view(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, view, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List namespaces, optionally providing a parent namespace to list underneath
    async fn list_namespaces(
        &self,
        prefix: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        parent: Option<String>,
        context: &C) -> Result<ListNamespacesResponse, ApiError>
    {
        info!("list_namespaces(\"{}\", {:?}, {:?}, {:?}) - X-Span-ID: {:?}", prefix, page_token, page_size, parent, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all table identifiers underneath a given namespace
    async fn list_tables(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        context: &C) -> Result<ListTablesResponse, ApiError>
    {
        info!("list_tables(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", prefix, namespace, page_token, page_size, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all view identifiers underneath a given namespace
    async fn list_views(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        context: &C) -> Result<ListViewsResponse, ApiError>
    {
        info!("list_views(\"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", prefix, namespace, page_token, page_size, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Load the metadata properties for a namespace
    async fn load_namespace_metadata(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<LoadNamespaceMetadataResponse, ApiError>
    {
        info!("load_namespace_metadata(\"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Load a table from the catalog
    async fn load_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        x_iceberg_access_delegation: Option<String>,
        snapshots: Option<String>,
        context: &C) -> Result<LoadTableResponse, ApiError>
    {
        info!("load_table(\"{}\", \"{}\", \"{}\", {:?}, {:?}) - X-Span-ID: {:?}", prefix, namespace, table, x_iceberg_access_delegation, snapshots, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Load a view from the catalog
    async fn load_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<LoadViewResponse, ApiError>
    {
        info!("load_view(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, view, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Check if a namespace exists
    async fn namespace_exists(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<NamespaceExistsResponse, ApiError>
    {
        info!("namespace_exists(\"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Register a table in the given namespace using given metadata file location
    async fn register_table(
        &self,
        prefix: String,
        namespace: String,
        register_table_request: Option<models::RegisterTableRequest>,
        context: &C) -> Result<RegisterTableResponse, ApiError>
    {
        info!("register_table(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, register_table_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Rename a table from its current name to a new name
    async fn rename_table(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        context: &C) -> Result<RenameTableResponse, ApiError>
    {
        info!("rename_table(\"{}\", {:?}) - X-Span-ID: {:?}", prefix, rename_table_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Rename a view from its current name to a new name
    async fn rename_view(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        context: &C) -> Result<RenameViewResponse, ApiError>
    {
        info!("rename_view(\"{}\", {:?}) - X-Span-ID: {:?}", prefix, rename_table_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Replace a view
    async fn replace_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        commit_view_request: Option<models::CommitViewRequest>,
        context: &C) -> Result<ReplaceViewResponse, ApiError>
    {
        info!("replace_view(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, view, commit_view_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Send a metrics report to this endpoint to be processed by the backend
    async fn report_metrics(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        report_metrics_request: models::ReportMetricsRequest,
        context: &C) -> Result<ReportMetricsResponse, ApiError>
    {
        info!("report_metrics(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, table, report_metrics_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Check if a table exists
    async fn table_exists(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        context: &C) -> Result<TableExistsResponse, ApiError>
    {
        info!("table_exists(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, table, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Set or remove properties on a namespace
    async fn update_properties(
        &self,
        prefix: String,
        namespace: String,
        update_namespace_properties_request: Option<models::UpdateNamespacePropertiesRequest>,
        context: &C) -> Result<UpdatePropertiesResponse, ApiError>
    {
        info!("update_properties(\"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, update_namespace_properties_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Commit updates to a table
    async fn update_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        commit_table_request: Option<models::CommitTableRequest>,
        context: &C) -> Result<UpdateTableResponse, ApiError>
    {
        info!("update_table(\"{}\", \"{}\", \"{}\", {:?}) - X-Span-ID: {:?}", prefix, namespace, table, commit_table_request, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Check if a view exists
    async fn view_exists(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<ViewExistsResponse, ApiError>
    {
        info!("view_exists(\"{}\", \"{}\", \"{}\") - X-Span-ID: {:?}", prefix, namespace, view, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// List all catalog configuration settings
    async fn get_config(
        &self,
        warehouse: Option<String>,
        context: &C) -> Result<GetConfigResponse, ApiError>
    {
        info!("get_config({:?}) - X-Span-ID: {:?}", warehouse, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

    /// Get a token using an OAuth2 flow
    async fn get_token(
        &self,
        grant_type: Option<String>,
        scope: Option<String>,
        client_id: Option<String>,
        client_secret: Option<String>,
        requested_token_type: Option<models::TokenType>,
        subject_token: Option<String>,
        subject_token_type: Option<models::TokenType>,
        actor_token: Option<String>,
        actor_token_type: Option<models::TokenType>,
        context: &C) -> Result<GetTokenResponse, ApiError>
    {
        info!("get_token({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}) - X-Span-ID: {:?}", grant_type, scope, client_id, client_secret, requested_token_type, subject_token, subject_token_type, actor_token, actor_token_type, context.get().0.clone());
        Err(ApiError("Generic failure".into()))
    }

}

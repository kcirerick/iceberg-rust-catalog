#![allow(missing_docs, trivial_casts, unused_variables, unused_mut, unused_imports, unused_extern_crates, non_camel_case_types)]
#![allow(unused_imports, unused_attributes)]
#![allow(clippy::derive_partial_eq_without_eq, clippy::disallowed_names)]

use async_trait::async_trait;
use futures::Stream;
use std::error::Error;
use std::task::{Poll, Context};
use swagger::{ApiError, ContextWrapper};
use serde::{Serialize, Deserialize};

type ServiceError = Box<dyn Error + Send + Sync + 'static>;

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.0.1";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CommitTransactionResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, table to load does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Conflict - CommitFailedException, one or more requirements failed. The client may retry.
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// An unknown server-side problem occurred; the commit state is unknown.
    AnUnknownServer
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A gateway or proxy received an invalid response from the upstream server; the commit state is unknown.
    AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
    (models::IcebergErrorResponse)
    ,
    /// A server-side gateway timeout occurred; the commit state is unknown.
    AServer
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable on the client.
    AServer_2
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateNamespaceResponse {
    /// Represents a successful call to create a namespace. Returns the namespace created, as well as any properties that were stored for the namespace, including those the server might have added. Implementations are not required to support namespace properties.
    RepresentsASuccessfulCallToCreateANamespace
    (models::CreateNamespaceResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Acceptable / Unsupported Operation. The server does not support this operation.
    NotAcceptable
    (models::ErrorModel)
    ,
    /// Conflict - The namespace already exists
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateTableResponse {
    /// Table metadata result after creating a table
    TableMetadataResultAfterCreatingATable
    (models::LoadTableResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - The namespace specified does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Conflict - The table already exists
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum CreateViewResponse {
    /// View metadata result when loading a view
    ViewMetadataResultWhenLoadingAView
    (models::LoadViewResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - The namespace specified does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Conflict - The view already exists
    Conflict
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DropNamespaceResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - Namespace to delete does not exist.
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DropTableResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, Table to drop does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum DropViewResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchViewException, view to drop does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListNamespacesResponse {
    /// A list of namespaces
    AListOfNamespaces
    (models::ListNamespacesResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - Namespace provided in the `parent` query parameter is not found.
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListTablesResponse {
    /// A list of table identifiers
    AListOfTableIdentifiers
    (models::ListTablesResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - The namespace specified does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ListViewsResponse {
    /// A list of table identifiers
    AListOfTableIdentifiers
    (models::ListTablesResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - The namespace specified does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum LoadNamespaceMetadataResponse {
    /// Returns a namespace, as well as any properties stored on the namespace if namespace properties are supported by the server.
    ReturnsANamespace
    (models::GetNamespaceResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - Namespace not found
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum LoadTableResponse {
    /// Table metadata result when loading a table
    TableMetadataResultWhenLoadingATable
    (models::LoadTableResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, table to load does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum LoadViewResponse {
    /// View metadata result when loading a view
    ViewMetadataResultWhenLoadingAView
    (models::LoadViewResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchViewException, view to load does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum NamespaceExistsResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - Namespace not found
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RegisterTableResponse {
    /// Table metadata result when loading a table
    TableMetadataResultWhenLoadingATable
    (models::LoadTableResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - The namespace specified does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Conflict - The table already exists
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RenameTableResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, Table to rename does not exist - NoSuchNamespaceException, The target namespace of the new table identifier does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Not Acceptable / Unsupported Operation. The server does not support this operation.
    NotAcceptable
    (models::ErrorModel)
    ,
    /// Conflict - The target identifier to rename to already exists as a table or view
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum RenameViewResponse {
    /// OK
    OK
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchViewException, view to rename does not exist - NoSuchNamespaceException, The target namespace of the new identifier does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Not Acceptable / Unsupported Operation. The server does not support this operation.
    NotAcceptable
    (models::ErrorModel)
    ,
    /// Conflict - The target identifier to rename to already exists as a table or view
    Conflict
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ReplaceViewResponse {
    /// View metadata result when loading a view
    ViewMetadataResultWhenLoadingAView
    (models::LoadViewResult)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchViewException, view to load does not exist
    NotFound
    (models::ErrorModel)
    ,
    /// Conflict - CommitFailedException. The client may retry.
    Conflict
    (models::ErrorModel)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// An unknown server-side problem occurred; the commit state is unknown.
    AnUnknownServer
    (models::ErrorModel)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A gateway or proxy received an invalid response from the upstream server; the commit state is unknown.
    AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
    (models::ErrorModel)
    ,
    /// A server-side gateway timeout occurred; the commit state is unknown.
    AServer
    (models::ErrorModel)
    ,
    /// A server-side problem that might not be addressable on the client.
    AServer_2
    (models::ErrorModel)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ReportMetricsResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, table to load does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum TableExistsResponse {
    /// Success, no content
    Success
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, Table not found
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdatePropertiesResponse {
    /// JSON data response for a synchronous update properties request.
    JSONDataResponseForASynchronousUpdatePropertiesRequest
    (models::UpdateNamespacePropertiesResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - Namespace not found
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Not Acceptable / Unsupported Operation. The server does not support this operation.
    NotAcceptable
    (models::ErrorModel)
    ,
    /// Unprocessable Entity - A property key was included in both `removals` and `updates`
    UnprocessableEntity
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum UpdateTableResponse {
    /// Response used when a table is successfully updated. The table metadata JSON is returned in the metadata field. The corresponding file location of table metadata must be returned in the metadata-location field. Clients can check whether metadata has changed by comparing metadata locations.
    ResponseUsedWhenATableIsSuccessfullyUpdated
    (models::CommitTableResponse)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Not Found - NoSuchTableException, table to load does not exist
    NotFound
    (models::IcebergErrorResponse)
    ,
    /// Conflict - CommitFailedException, one or more requirements failed. The client may retry.
    Conflict
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// An unknown server-side problem occurred; the commit state is unknown.
    AnUnknownServer
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A gateway or proxy received an invalid response from the upstream server; the commit state is unknown.
    AGatewayOrProxyReceivedAnInvalidResponseFromTheUpstreamServer
    (models::IcebergErrorResponse)
    ,
    /// A server-side gateway timeout occurred; the commit state is unknown.
    AServer
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable on the client.
    AServer_2
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum ViewExistsResponse {
    /// Success, no content
    Success
    ,
    /// Bad Request
    BadRequest
    ,
    /// Unauthorized
    Unauthorized
    ,
    /// Not Found
    NotFound
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetConfigResponse {
    /// Server specified configuration values.
    ServerSpecifiedConfigurationValues
    (models::CatalogConfig)
    ,
    /// Indicates a bad request error. It could be caused by an unexpected request body format or other forms of request validation failure, such as invalid json. Usually serves application/json content, although in some cases simple text/plain content might be returned by the server's middleware.
    IndicatesABadRequestError
    (models::IcebergErrorResponse)
    ,
    /// Unauthorized. Authentication is required and has failed or has not yet been provided.
    Unauthorized
    (models::IcebergErrorResponse)
    ,
    /// Forbidden. Authenticated user does not have the necessary permissions.
    Forbidden
    (models::IcebergErrorResponse)
    ,
    /// Credentials have timed out. If possible, the client should refresh credentials and retry.
    CredentialsHaveTimedOut
    (models::IcebergErrorResponse)
    ,
    /// The service is not ready to handle the request. The client should wait and retry.  The service may additionally send a Retry-After header to indicate when to retry.
    TheServiceIsNotReadyToHandleTheRequest
    (models::IcebergErrorResponse)
    ,
    /// A server-side problem that might not be addressable from the client side. Used for server 5xx errors without more specific documentation in individual routes.
    AServer
    (models::IcebergErrorResponse)
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[must_use]
pub enum GetTokenResponse {
    /// OAuth2 token response for client credentials or token exchange
    OAuth
    (models::OAuthTokenResponse)
    ,
    /// OAuth2 error response
    OAuth_2
    (models::OAuthError)
    ,
    /// OAuth2 error response
    OAuth_3
    (models::OAuthError)
    ,
    /// OAuth2 error response
    OAuth_4
    (models::OAuthError)
}

/// API
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait Api<C: Send + Sync> {
    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>> {
        Poll::Ready(Ok(()))
    }

    /// Commit updates to multiple tables in an atomic operation
    async fn commit_transaction(
        &self,
        prefix: String,
        commit_transaction_request: models::CommitTransactionRequest,
        context: &C) -> Result<CommitTransactionResponse, ApiError>;

    /// Create a namespace
    async fn create_namespace(
        &self,
        prefix: String,
        create_namespace_request: Option<models::CreateNamespaceRequest>,
        context: &C) -> Result<CreateNamespaceResponse, ApiError>;

    /// Create a table in the given namespace
    async fn create_table(
        &self,
        prefix: String,
        namespace: String,
        x_iceberg_access_delegation: Option<String>,
        create_table_request: Option<models::CreateTableRequest>,
        context: &C) -> Result<CreateTableResponse, ApiError>;

    /// Create a view in the given namespace
    async fn create_view(
        &self,
        prefix: String,
        namespace: String,
        create_view_request: Option<models::CreateViewRequest>,
        context: &C) -> Result<CreateViewResponse, ApiError>;

    /// Drop a namespace from the catalog. Namespace must be empty.
    async fn drop_namespace(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<DropNamespaceResponse, ApiError>;

    /// Drop a table from the catalog
    async fn drop_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        purge_requested: Option<bool>,
        context: &C) -> Result<DropTableResponse, ApiError>;

    /// Drop a view from the catalog
    async fn drop_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<DropViewResponse, ApiError>;

    /// List namespaces, optionally providing a parent namespace to list underneath
    async fn list_namespaces(
        &self,
        prefix: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        parent: Option<String>,
        context: &C) -> Result<ListNamespacesResponse, ApiError>;

    /// List all table identifiers underneath a given namespace
    async fn list_tables(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        context: &C) -> Result<ListTablesResponse, ApiError>;

    /// List all view identifiers underneath a given namespace
    async fn list_views(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        context: &C) -> Result<ListViewsResponse, ApiError>;

    /// Load the metadata properties for a namespace
    async fn load_namespace_metadata(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<LoadNamespaceMetadataResponse, ApiError>;

    /// Load a table from the catalog
    async fn load_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        x_iceberg_access_delegation: Option<String>,
        snapshots: Option<String>,
        context: &C) -> Result<LoadTableResponse, ApiError>;

    /// Load a view from the catalog
    async fn load_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<LoadViewResponse, ApiError>;

    /// Check if a namespace exists
    async fn namespace_exists(
        &self,
        prefix: String,
        namespace: String,
        context: &C) -> Result<NamespaceExistsResponse, ApiError>;

    /// Register a table in the given namespace using given metadata file location
    async fn register_table(
        &self,
        prefix: String,
        namespace: String,
        register_table_request: Option<models::RegisterTableRequest>,
        context: &C) -> Result<RegisterTableResponse, ApiError>;

    /// Rename a table from its current name to a new name
    async fn rename_table(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        context: &C) -> Result<RenameTableResponse, ApiError>;

    /// Rename a view from its current name to a new name
    async fn rename_view(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        context: &C) -> Result<RenameViewResponse, ApiError>;

    /// Replace a view
    async fn replace_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        commit_view_request: Option<models::CommitViewRequest>,
        context: &C) -> Result<ReplaceViewResponse, ApiError>;

    /// Send a metrics report to this endpoint to be processed by the backend
    async fn report_metrics(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        report_metrics_request: models::ReportMetricsRequest,
        context: &C) -> Result<ReportMetricsResponse, ApiError>;

    /// Check if a table exists
    async fn table_exists(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        context: &C) -> Result<TableExistsResponse, ApiError>;

    /// Set or remove properties on a namespace
    async fn update_properties(
        &self,
        prefix: String,
        namespace: String,
        update_namespace_properties_request: Option<models::UpdateNamespacePropertiesRequest>,
        context: &C) -> Result<UpdatePropertiesResponse, ApiError>;

    /// Commit updates to a table
    async fn update_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        commit_table_request: Option<models::CommitTableRequest>,
        context: &C) -> Result<UpdateTableResponse, ApiError>;

    /// Check if a view exists
    async fn view_exists(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        context: &C) -> Result<ViewExistsResponse, ApiError>;

    /// List all catalog configuration settings
    async fn get_config(
        &self,
        warehouse: Option<String>,
        context: &C) -> Result<GetConfigResponse, ApiError>;

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
        context: &C) -> Result<GetTokenResponse, ApiError>;

}

/// API where `Context` isn't passed on every API call
#[async_trait]
#[allow(clippy::too_many_arguments, clippy::ptr_arg)]
pub trait ApiNoContext<C: Send + Sync> {

    fn poll_ready(&self, _cx: &mut Context) -> Poll<Result<(), Box<dyn Error + Send + Sync + 'static>>>;

    fn context(&self) -> &C;

    /// Commit updates to multiple tables in an atomic operation
    async fn commit_transaction(
        &self,
        prefix: String,
        commit_transaction_request: models::CommitTransactionRequest,
        ) -> Result<CommitTransactionResponse, ApiError>;

    /// Create a namespace
    async fn create_namespace(
        &self,
        prefix: String,
        create_namespace_request: Option<models::CreateNamespaceRequest>,
        ) -> Result<CreateNamespaceResponse, ApiError>;

    /// Create a table in the given namespace
    async fn create_table(
        &self,
        prefix: String,
        namespace: String,
        x_iceberg_access_delegation: Option<String>,
        create_table_request: Option<models::CreateTableRequest>,
        ) -> Result<CreateTableResponse, ApiError>;

    /// Create a view in the given namespace
    async fn create_view(
        &self,
        prefix: String,
        namespace: String,
        create_view_request: Option<models::CreateViewRequest>,
        ) -> Result<CreateViewResponse, ApiError>;

    /// Drop a namespace from the catalog. Namespace must be empty.
    async fn drop_namespace(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<DropNamespaceResponse, ApiError>;

    /// Drop a table from the catalog
    async fn drop_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        purge_requested: Option<bool>,
        ) -> Result<DropTableResponse, ApiError>;

    /// Drop a view from the catalog
    async fn drop_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<DropViewResponse, ApiError>;

    /// List namespaces, optionally providing a parent namespace to list underneath
    async fn list_namespaces(
        &self,
        prefix: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        parent: Option<String>,
        ) -> Result<ListNamespacesResponse, ApiError>;

    /// List all table identifiers underneath a given namespace
    async fn list_tables(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        ) -> Result<ListTablesResponse, ApiError>;

    /// List all view identifiers underneath a given namespace
    async fn list_views(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        ) -> Result<ListViewsResponse, ApiError>;

    /// Load the metadata properties for a namespace
    async fn load_namespace_metadata(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<LoadNamespaceMetadataResponse, ApiError>;

    /// Load a table from the catalog
    async fn load_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        x_iceberg_access_delegation: Option<String>,
        snapshots: Option<String>,
        ) -> Result<LoadTableResponse, ApiError>;

    /// Load a view from the catalog
    async fn load_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<LoadViewResponse, ApiError>;

    /// Check if a namespace exists
    async fn namespace_exists(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<NamespaceExistsResponse, ApiError>;

    /// Register a table in the given namespace using given metadata file location
    async fn register_table(
        &self,
        prefix: String,
        namespace: String,
        register_table_request: Option<models::RegisterTableRequest>,
        ) -> Result<RegisterTableResponse, ApiError>;

    /// Rename a table from its current name to a new name
    async fn rename_table(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        ) -> Result<RenameTableResponse, ApiError>;

    /// Rename a view from its current name to a new name
    async fn rename_view(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        ) -> Result<RenameViewResponse, ApiError>;

    /// Replace a view
    async fn replace_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        commit_view_request: Option<models::CommitViewRequest>,
        ) -> Result<ReplaceViewResponse, ApiError>;

    /// Send a metrics report to this endpoint to be processed by the backend
    async fn report_metrics(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        report_metrics_request: models::ReportMetricsRequest,
        ) -> Result<ReportMetricsResponse, ApiError>;

    /// Check if a table exists
    async fn table_exists(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        ) -> Result<TableExistsResponse, ApiError>;

    /// Set or remove properties on a namespace
    async fn update_properties(
        &self,
        prefix: String,
        namespace: String,
        update_namespace_properties_request: Option<models::UpdateNamespacePropertiesRequest>,
        ) -> Result<UpdatePropertiesResponse, ApiError>;

    /// Commit updates to a table
    async fn update_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        commit_table_request: Option<models::CommitTableRequest>,
        ) -> Result<UpdateTableResponse, ApiError>;

    /// Check if a view exists
    async fn view_exists(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<ViewExistsResponse, ApiError>;

    /// List all catalog configuration settings
    async fn get_config(
        &self,
        warehouse: Option<String>,
        ) -> Result<GetConfigResponse, ApiError>;

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
        ) -> Result<GetTokenResponse, ApiError>;

}

/// Trait to extend an API to make it easy to bind it to a context.
pub trait ContextWrapperExt<C: Send + Sync> where Self: Sized
{
    /// Binds this API to a context.
    fn with_context(self, context: C) -> ContextWrapper<Self, C>;
}

impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ContextWrapperExt<C> for T {
    fn with_context(self: T, context: C) -> ContextWrapper<T, C> {
         ContextWrapper::<T, C>::new(self, context)
    }
}

#[async_trait]
impl<T: Api<C> + Send + Sync, C: Clone + Send + Sync> ApiNoContext<C> for ContextWrapper<T, C> {
    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), ServiceError>> {
        self.api().poll_ready(cx)
    }

    fn context(&self) -> &C {
        ContextWrapper::context(self)
    }

    /// Commit updates to multiple tables in an atomic operation
    async fn commit_transaction(
        &self,
        prefix: String,
        commit_transaction_request: models::CommitTransactionRequest,
        ) -> Result<CommitTransactionResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().commit_transaction(prefix, commit_transaction_request, &context).await
    }

    /// Create a namespace
    async fn create_namespace(
        &self,
        prefix: String,
        create_namespace_request: Option<models::CreateNamespaceRequest>,
        ) -> Result<CreateNamespaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_namespace(prefix, create_namespace_request, &context).await
    }

    /// Create a table in the given namespace
    async fn create_table(
        &self,
        prefix: String,
        namespace: String,
        x_iceberg_access_delegation: Option<String>,
        create_table_request: Option<models::CreateTableRequest>,
        ) -> Result<CreateTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_table(prefix, namespace, x_iceberg_access_delegation, create_table_request, &context).await
    }

    /// Create a view in the given namespace
    async fn create_view(
        &self,
        prefix: String,
        namespace: String,
        create_view_request: Option<models::CreateViewRequest>,
        ) -> Result<CreateViewResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().create_view(prefix, namespace, create_view_request, &context).await
    }

    /// Drop a namespace from the catalog. Namespace must be empty.
    async fn drop_namespace(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<DropNamespaceResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().drop_namespace(prefix, namespace, &context).await
    }

    /// Drop a table from the catalog
    async fn drop_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        purge_requested: Option<bool>,
        ) -> Result<DropTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().drop_table(prefix, namespace, table, purge_requested, &context).await
    }

    /// Drop a view from the catalog
    async fn drop_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<DropViewResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().drop_view(prefix, namespace, view, &context).await
    }

    /// List namespaces, optionally providing a parent namespace to list underneath
    async fn list_namespaces(
        &self,
        prefix: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        parent: Option<String>,
        ) -> Result<ListNamespacesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_namespaces(prefix, page_token, page_size, parent, &context).await
    }

    /// List all table identifiers underneath a given namespace
    async fn list_tables(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        ) -> Result<ListTablesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_tables(prefix, namespace, page_token, page_size, &context).await
    }

    /// List all view identifiers underneath a given namespace
    async fn list_views(
        &self,
        prefix: String,
        namespace: String,
        page_token: Option<String>,
        page_size: Option<i32>,
        ) -> Result<ListViewsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().list_views(prefix, namespace, page_token, page_size, &context).await
    }

    /// Load the metadata properties for a namespace
    async fn load_namespace_metadata(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<LoadNamespaceMetadataResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().load_namespace_metadata(prefix, namespace, &context).await
    }

    /// Load a table from the catalog
    async fn load_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        x_iceberg_access_delegation: Option<String>,
        snapshots: Option<String>,
        ) -> Result<LoadTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().load_table(prefix, namespace, table, x_iceberg_access_delegation, snapshots, &context).await
    }

    /// Load a view from the catalog
    async fn load_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<LoadViewResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().load_view(prefix, namespace, view, &context).await
    }

    /// Check if a namespace exists
    async fn namespace_exists(
        &self,
        prefix: String,
        namespace: String,
        ) -> Result<NamespaceExistsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().namespace_exists(prefix, namespace, &context).await
    }

    /// Register a table in the given namespace using given metadata file location
    async fn register_table(
        &self,
        prefix: String,
        namespace: String,
        register_table_request: Option<models::RegisterTableRequest>,
        ) -> Result<RegisterTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().register_table(prefix, namespace, register_table_request, &context).await
    }

    /// Rename a table from its current name to a new name
    async fn rename_table(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        ) -> Result<RenameTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().rename_table(prefix, rename_table_request, &context).await
    }

    /// Rename a view from its current name to a new name
    async fn rename_view(
        &self,
        prefix: String,
        rename_table_request: models::RenameTableRequest,
        ) -> Result<RenameViewResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().rename_view(prefix, rename_table_request, &context).await
    }

    /// Replace a view
    async fn replace_view(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        commit_view_request: Option<models::CommitViewRequest>,
        ) -> Result<ReplaceViewResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().replace_view(prefix, namespace, view, commit_view_request, &context).await
    }

    /// Send a metrics report to this endpoint to be processed by the backend
    async fn report_metrics(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        report_metrics_request: models::ReportMetricsRequest,
        ) -> Result<ReportMetricsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().report_metrics(prefix, namespace, table, report_metrics_request, &context).await
    }

    /// Check if a table exists
    async fn table_exists(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        ) -> Result<TableExistsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().table_exists(prefix, namespace, table, &context).await
    }

    /// Set or remove properties on a namespace
    async fn update_properties(
        &self,
        prefix: String,
        namespace: String,
        update_namespace_properties_request: Option<models::UpdateNamespacePropertiesRequest>,
        ) -> Result<UpdatePropertiesResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_properties(prefix, namespace, update_namespace_properties_request, &context).await
    }

    /// Commit updates to a table
    async fn update_table(
        &self,
        prefix: String,
        namespace: String,
        table: String,
        commit_table_request: Option<models::CommitTableRequest>,
        ) -> Result<UpdateTableResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().update_table(prefix, namespace, table, commit_table_request, &context).await
    }

    /// Check if a view exists
    async fn view_exists(
        &self,
        prefix: String,
        namespace: String,
        view: String,
        ) -> Result<ViewExistsResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().view_exists(prefix, namespace, view, &context).await
    }

    /// List all catalog configuration settings
    async fn get_config(
        &self,
        warehouse: Option<String>,
        ) -> Result<GetConfigResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_config(warehouse, &context).await
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
        ) -> Result<GetTokenResponse, ApiError>
    {
        let context = self.context().clone();
        self.api().get_token(grant_type, scope, client_id, client_secret, requested_token_type, subject_token, subject_token_type, actor_token, actor_token_type, &context).await
    }

}


#[cfg(feature = "client")]
pub mod client;

// Re-export Client as a top-level name
#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "server")]
pub mod server;

// Re-export router() as a top-level name
#[cfg(feature = "server")]
pub use self::server::Service;

#[cfg(feature = "server")]
pub mod context;

pub mod models;

#[cfg(any(feature = "client", feature = "server"))]
pub(crate) mod header;


use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdSchemasSchemaIdFilesFilePathResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_schemas_schema_id_files_file_path_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse400 {
    /// Information about the error.
    pub error: Option<DeleteApisApiIdVersionsVersionIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse400Error {
    /// Details about the error message.
    pub message: Option<String>,

    /// The type of error.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub delete_apis_api_id_versions_version_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_versions_version_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_versions_version_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteApisApiIdVersionsVersionIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_apis_api_id_versions_version_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse {
    /// The folder's information.
    pub data: Option<DeleteCollectionsCollectionIdFoldersFolderIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponseData {
    /// The folder's ID.
    pub id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse401 {
    pub error: Option<DeleteCollectionsCollectionIdFoldersFolderIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse401Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse404 {
    pub error: Option<DeleteCollectionsCollectionIdFoldersFolderIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse404Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdFoldersFolderIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_collections_collection_id_folders_folder_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse {
    /// The request's information.
    pub data: Option<DeleteCollectionsCollectionIdRequestsRequestIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponseData {
    /// The request's ID.
    pub id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse401 {
    pub error: Option<DeleteCollectionsCollectionIdRequestsRequestIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse401Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse404 {
    pub error: Option<DeleteCollectionsCollectionIdRequestsRequestIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse404Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdRequestsRequestIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_collections_collection_id_requests_request_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse {
    /// Information about the deleted collection.
    pub collection: Option<DeleteCollectionsCollectionIdResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponseCollection {
    /// The deleted collection's ID.
    pub id: Option<String>,

    /// The deleted collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse401 {
    pub error: Option<DeleteCollectionsCollectionIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse404 {
    pub error: Option<DeleteCollectionsCollectionIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse404Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse404ErrorDetails {
    /// The collection ID.
    pub id: Option<String>,

    /// The instance item.
    pub item: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse500 {
    pub error: Option<DeleteCollectionsCollectionIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse {
    /// The response's information.
    pub data: Option<DeleteCollectionsCollectionIdResponsesResponseIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponseData {
    /// The response's ID.
    pub id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse401 {
    pub error: Option<DeleteCollectionsCollectionIdResponsesResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse401Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse404 {
    pub error: Option<DeleteCollectionsCollectionIdResponsesResponseIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse404Error {
    /// Information about the error.
    pub details: Option<DeleteCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCollectionsCollectionIdResponsesResponseIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub delete_collections_collection_id_responses_response_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse {
    pub environment: Option<DeleteEnvironmentsEnvironmentIdResponseEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponseEnvironment {
    /// The deleted environment's ID.
    pub id: Option<String>,

    /// The deleted environment's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse401 {
    pub error: Option<DeleteEnvironmentsEnvironmentIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse404 {
    pub error: Option<DeleteEnvironmentsEnvironmentIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse500 {
    pub error: Option<DeleteEnvironmentsEnvironmentIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEnvironmentsEnvironmentIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse {
    /// Information about the mock server.
    pub mock: Option<DeleteMocksMockIdResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponseMock {
    /// The mock server's ID.
    pub id: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse400 {
    pub error: Option<DeleteMocksMockIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse401 {
    pub error: Option<DeleteMocksMockIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse404 {
    pub error: Option<DeleteMocksMockIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse500 {
    pub error: Option<DeleteMocksMockIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse {
    /// The server response's body that returns when calling the mock server.
    pub body: Option<String>,

    /// The date and time at which the server response was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the server response.
    pub created_by: Option<String>,

    /// The server response's request headers, such as Content-Type, Accept, encoding, and other
    /// information.
    pub headers: Option<Vec<DeleteMocksMockIdServerResponsesServerResponseIdResponseHeadersItem>>,

    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's body language type.
    pub language: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponseLanguage>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code.
    pub status_code: Option<f64>,

    /// The user ID of the user who last updated the server response.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponseHeadersItem {
    /// The request header's key value.
    pub key: Option<String>,

    /// The request header's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteMocksMockIdServerResponsesServerResponseIdResponseLanguage {
    Html,

    Javascript,

    Json,

    Text,

    Xml,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse400 {
    pub error: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse401 {
    pub error: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse404 {
    pub error: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse500 {
    pub error: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdServerResponsesServerResponseIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse {
    pub mock: Option<DeleteMocksMockIdUnpublishResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponseMock {
    /// The mock server's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse400 {
    pub error: Option<DeleteMocksMockIdUnpublishResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse401 {
    pub error: Option<DeleteMocksMockIdUnpublishResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse404 {
    pub error: Option<DeleteMocksMockIdUnpublishResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse500 {
    pub error: Option<DeleteMocksMockIdUnpublishResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMocksMockIdUnpublishResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse {
    pub monitor: Option<DeleteMonitorsMonitorIdResponseMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponseMonitor {
    /// The monitor's ID.
    pub id: Option<String>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse400 {
    pub error: Option<DeleteMonitorsMonitorIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse401 {
    pub error: Option<DeleteMonitorsMonitorIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse500 {
    pub error: Option<DeleteMonitorsMonitorIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteMonitorsMonitorIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse {
    /// The Private API Network element type. The name of the object is the element type.
    pub element_type: Option<DeleteNetworkPrivateElementTypeElementIdResponseElementType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponseElementType {
    /// The element's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse400 {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse401 {
    pub error: Option<DeleteNetworkPrivateElementTypeElementIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse403 {
    pub error: Option<DeleteNetworkPrivateElementTypeElementIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse404 {
    pub error: Option<DeleteNetworkPrivateElementTypeElementIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse500 {
    pub error: Option<DeleteNetworkPrivateElementTypeElementIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteNetworkPrivateElementTypeElementIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse400 {
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteScimV2GroupsGroupIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse {
    /// Information about the deleted workspace.
    pub workspace: Option<DeleteWorkspacesWorkspaceIdResponseWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponseWorkspace {
    /// The workspace's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse400 {
    pub error: Option<DeleteWorkspacesWorkspaceIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse401 {
    pub error: Option<DeleteWorkspacesWorkspaceIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse500 {
    pub error: Option<DeleteWorkspacesWorkspaceIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteWorkspacesWorkspaceIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse {
    /// Information about the collection.
    pub info: Option<GetApisApiIdCollectionsCollectionIdResponseInfo>,

    pub item: Option<Vec<GetApisApiIdCollectionsCollectionIdResponseItemItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdCollectionsCollectionIdResponseInfo {
    /// The collection's ID.
    #[serde(rename = "_postman_id")]
    pub postman_id: Option<String>,

    /// The collection's description.
    pub description: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The collection's JSON schema version.
    pub schema: Option<Schema>,

    /// The date and time at which the collection was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Schema {
    #[serde(rename = "https://schema.getpostman.com/json/collection/v2.1.0/collection.json")]
    HttpsSchemaGetpostmanComJsonCollectionV210CollectionJson,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponseItemItem {
    /// The collection's event information. For a complete list of values, refer to the
    /// `definitions.event` entry in the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub event: Option<Vec<Option<serde_json::Value>>>,

    /// The collection item's ID.
    pub id: Option<String>,

    /// The collection item's human-readable identifier.
    pub name: Option<String>,

    pub request: Option<serde_json::Value>,

    /// The collection's response information. For a complete list of values, refer to the
    /// `definitions.response` entry in the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub response: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_collections_collection_id_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_collections_collection_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_collections_collection_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_collections_collection_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdCollectionsCollectionIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_collections_collection_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse {
    /// The schema file's content.
    pub content: Option<String>,

    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<String>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_file_path_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_file_path_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_file_path_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_file_path_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesFilePathResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_file_path_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse {
    /// The schema's files.
    pub files: Option<Vec<GetApisApiIdSchemasSchemaIdFilesResponseFilesItem>>,

    /// The schema's non-standard meta information.
    pub meta: Option<GetApisApiIdSchemasSchemaIdFilesResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdSchemasSchemaIdFilesResponseFilesItem {
    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<f64>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdSchemasSchemaIdFilesResponseMeta {
    /// The pointer to the next record in the set of paginated results.
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdFilesResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_files_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdSchemasSchemaIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_schemas_schema_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<GetApisApiIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_apis_api_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_apis_api_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_apis_api_id_tags_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_apis_api_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdTasksTaskIdResponse {
    /// The date and time at which the task was created.
    pub created_at: Option<String>,

    pub details: Option<serde_json::Value>,

    /// The task's ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<GetApisApiIdTasksTaskIdResponseMeta>,

    /// The task's current status.
    pub status: Option<GetApisApiIdTasksTaskIdResponseStatus>,

    /// The date and time at which the task was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTasksTaskIdResponseMeta {
    /// The task's action.
    pub action: Option<Action>,

    /// The model for which the task is performing the operation.
    pub model: Option<Model>,

    /// The endpoint URL that created the task.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Create,

    Update,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Model {
    #[serde(rename = "api-version")]
    ApiVersion,

    Collection,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GetApisApiIdTasksTaskIdResponseStatus {
    Completed,

    Failed,

    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTasksTaskIdResponse400 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_tasks_task_id_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTasksTaskIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_tasks_task_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTasksTaskIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_tasks_task_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdTasksTaskIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_tasks_task_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsResponse {
    /// The response's meta information for paginated results.
    pub meta: Option<GetApisApiIdVersionsResponseMeta>,

    pub versions: Option<Vec<GetApisApiIdVersionsResponseVersionsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdVersionsResponseMeta {
    /// The maximum number of records in the paginated response.
    pub limit: Option<f64>,

    /// The Base64-encoded value that points to the next record in the results set.
    pub next_cursor: Option<String>,

    /// The number of records that match the defined criteria.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdVersionsResponseVersionsItem {
    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// The version's release notes.
    pub release_notes: Option<String>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponse {
    /// Information about the API version.
    pub version: Option<GetApisApiIdVersionsVersionIdResponseVersion>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisApiIdVersionsVersionIdResponseVersion {
    pub collections: Option<Vec<GetApisApiIdVersionsVersionIdResponseVersionCollectionsItem>>,

    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// The version's release notes.
    pub release_notes: Option<String>,

    pub schemas: Option<Vec<GetApisApiIdVersionsVersionIdResponseVersionSchemasItem>>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponseVersionCollectionsItem {
    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_version_id_response_version_collections_item_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponseVersionSchemasItem {
    /// The schema's ID.
    pub id: Option<String>,

    /// The schema type.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_version_id_response_version_schemas_item_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_version_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_version_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisApiIdVersionsVersionIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_api_id_versions_version_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse {
    pub apis: Option<Vec<GetApisResponseApisItem>>,

    /// The response's meta information for paginated results.
    pub meta: Option<GetApisResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisResponseApisItem {
    /// The date and time at which the API was created.
    pub created_at: Option<String>,

    /// The Postman ID of the user that created the API.
    pub created_by: Option<f64>,

    /// The API's description.
    pub description: Option<String>,

    /// The API's ID.
    pub id: Option<String>,

    /// The API's name.
    pub name: Option<String>,

    /// The API's short summary.
    pub summary: Option<String>,

    /// The date and time at which the API was updated.
    pub updated_at: Option<String>,

    /// The Postman ID of the user that updated the API.
    pub updated_by: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetApisResponseMeta {
    /// The maximum number of records in the paginated response.
    pub limit: Option<f64>,

    /// The Base64-encoded value that points to the next record in the results set.
    pub next_cursor: Option<String>,

    /// The number of records that match the defined criteria.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse401 {
    pub error: Option<GetApisResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub get_apis_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetApisResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_apis_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse {
    pub trails: Option<Vec<GetAuditLogsResponseTrailsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAuditLogsResponseTrailsItem {
    /// The action performed by the user.
    pub action: Option<String>,

    pub data: Option<GetAuditLogsResponseTrailsItemData>,

    /// The audit event's ID.
    pub id: Option<f64>,

    /// The IP address of the user that performed the action.
    pub ip: Option<String>,

    /// The audit event's description.
    pub message: Option<String>,

    /// The date and time at which the event occurred.
    pub timestamp: Option<String>,

    /// The user agent information.
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponseTrailsItemData {
    /// Information about the user who preformed the audit event.
    pub actor: Option<GetAuditLogsResponseTrailsItemDataActor>,

    /// The user's team information.
    pub team: Option<GetAuditLogsResponseTrailsItemDataTeam>,

    /// Information about the user.
    pub user: Option<GetAuditLogsResponseTrailsItemDataUser>,

    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponseTrailsItemDataActor {
    /// If true, the user is active. If false, the user is deactivated.
    pub active: Option<bool>,

    /// The user's email address.
    pub email: Option<String>,

    pub id: Option<f64>,

    /// The user's name.
    pub name: Option<String>,

    /// The user's username.
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponseTrailsItemDataTeam {
    /// The team's ID.
    pub id: Option<f64>,

    /// The team's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponseTrailsItemDataUser {
    /// The user's email address.
    pub email: Option<String>,

    /// The user's ID.
    pub id: Option<f64>,

    /// The user's name.
    pub name: Option<String>,

    /// The user's username.
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse401 {
    pub error: Option<GetAuditLogsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse500 {
    pub error: Option<GetAuditLogsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAuditLogsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse {
    /// Information about the folder. For a complete list of properties, refer to the
    /// `definitions.folder` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<GetCollectionsCollectionIdFoldersFolderIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The folder's ID.
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponseData {
    /// The collection ID that the folder belongs to.
    pub collection: Option<String>,

    /// The folder's creation date and time.
    pub created_at: Option<String>,

    /// The folder's description.
    pub description: Option<String>,

    /// The folder's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<i64>,

    /// The user ID of the user that last updated the folder.
    pub last_updated_by: Option<String>,

    /// The folder's name.
    pub name: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,

    /// The date and time at which the folder was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse401 {
    pub error: Option<GetCollectionsCollectionIdFoldersFolderIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse401Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse404 {
    pub error: Option<GetCollectionsCollectionIdFoldersFolderIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse404Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdFoldersFolderIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_folders_folder_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse {
    /// Information about the request. For a complete list of properties, refer to the
    /// `definitions.request` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<GetCollectionsCollectionIdRequestsRequestIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The request's ID.
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponseData {
    /// The request's creation date and time.
    pub created_at: Option<String>,

    /// The request's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<i64>,

    /// The user ID of the user that last updated the request.
    pub last_updated_by: Option<String>,

    /// The request's name.
    pub name: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,

    /// The date and time at which the request was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse401 {
    pub error: Option<GetCollectionsCollectionIdRequestsRequestIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse401Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse404 {
    pub error: Option<GetCollectionsCollectionIdRequestsRequestIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse404Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdRequestsRequestIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_requests_request_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse {
    /// For a complete list of this endpoint's possible values, use the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub collection: Option<GetCollectionsCollectionIdResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponseCollection {
    /// An object that contains basic information about the collection.
    pub info: Option<GetCollectionsCollectionIdResponseCollectionInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsCollectionIdResponseCollectionInfo {
    /// The collection's Postman ID.
    #[serde(rename = "_postman_id")]
    pub postman_id: Option<String>,

    /// The collection's description.
    pub description: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// A URL to the collection's schema.
    pub schema: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the collection was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse400 {
    pub error: Option<GetCollectionsCollectionIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse401 {
    pub error: Option<GetCollectionsCollectionIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse500 {
    pub error: Option<GetCollectionsCollectionIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse {
    /// Information about the response. For a complete list of properties, refer to the
    /// `request.responses` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<GetCollectionsCollectionIdResponsesResponseIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The response's ID.
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponseData {
    /// The response's creation date and time.
    pub created_at: Option<String>,

    /// The response's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<i64>,

    /// The user ID of the user that last updated the response.
    pub last_updated_by: Option<String>,

    /// The response's name.
    pub name: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,

    /// The ID of the request that the response belongs to.
    pub request: Option<String>,

    /// The date and time at which the response was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse401 {
    pub error: Option<GetCollectionsCollectionIdResponsesResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse401Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse404 {
    pub error: Option<GetCollectionsCollectionIdResponsesResponseIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse404Error {
    /// Information about the error.
    pub details: Option<GetCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdResponsesResponseIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_responses_response_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<GetCollectionsCollectionIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_collections_collection_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_collections_collection_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_collections_collection_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTransformationsResponse {
    /// The collection's transformed output, in a stringified OpenAPI format.
    pub output: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTransformationsResponse401 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The instance in which the error occurred.
    pub instance: Option<String>,

    /// The error's status code.
    pub status: Option<i64>,

    /// The title of the error message.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_transformations_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTransformationsResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The instance in which the error occurred.
    pub instance: Option<String>,

    /// The error's status code.
    pub status: Option<i64>,

    /// The title of the error message.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_transformations_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsCollectionIdTransformationsResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The instance in which the error occurred.
    pub instance: Option<String>,

    /// The error's status code.
    pub status: Option<i64>,

    /// The title of the error message.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_collections_collection_id_transformations_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse {
    pub collections: Option<Vec<GetCollectionsResponseCollectionsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsResponseCollectionsItem {
    /// The collection's creation date and time.
    pub created_at: Option<String>,

    /// If the collection is
    /// [forked](https://learning.postman.com/docs/collaborating-in-postman/version-control/#forking-postman-entities),
    /// the fork's information.
    pub fork: Option<GetCollectionsResponseCollectionsItemFork>,

    /// The collection's ID.
    pub id: Option<String>,

    /// If true, the collection is publicly available.
    pub is_public: Option<bool>,

    /// The collection's name.
    pub name: Option<String>,

    /// The owner of the collection.
    pub owner: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the collection was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollectionsResponseCollectionsItemFork {
    /// The fork's creation date and time.
    pub created_at: Option<String>,

    /// The unique ID of the fork's source collection.
    pub from: Option<String>,

    /// The fork's label.
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse401 {
    pub error: Option<GetCollectionsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse500 {
    pub error: Option<GetCollectionsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCollectionsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetectedSecretsSecretIdLocationsResponse {
    pub data: Option<Vec<GetDetectedSecretsSecretIdLocationsResponseDataItem>>,

    pub meta: Option<GetDetectedSecretsSecretIdLocationsResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDetectedSecretsSecretIdLocationsResponseDataItem {
    /// The date and time at which the secret was detected.
    pub detected_at: Option<String>,

    /// If true, the resource in which the secret was found was deleted.
    pub is_resource_deleted: Option<bool>,

    /// The ID of the user who leaked the secret.
    pub leaked_by: Option<f64>,

    /// The location where the secret was found.
    pub location: Option<String>,

    /// The number of times the secret occurs in the location.
    pub occurrences: Option<f64>,

    /// The parent resource's unique ID. If the resource is a request, folder, or example, this
    /// value is a collection ID. If the resource is a collection, globals, or environment, this
    /// is the resource's ID.
    pub parent_resource_id: Option<String>,

    /// The unique ID of the resource where the secret was detected.
    pub resource_id: Option<String>,

    /// The type of resource in which the secret was detected.
    pub resource_type: Option<DatumResourceType>,

    /// The URL to the resource that contains the secret.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatumResourceType {
    Api,

    Collection,

    Environment,

    Example,

    Folder,

    Globals,

    Request,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDetectedSecretsSecretIdLocationsResponseMeta {
    /// The history of the secret's resolution status changes.
    pub activity_feed: Option<Vec<GetDetectedSecretsSecretIdLocationsResponseMetaActivityFeedItem>>,

    /// The pointer to the first record of the set of paginated results.
    pub cursor: Option<String>,

    /// The maximum number of rows to return in the response.
    pub limit: Option<f64>,

    pub next_cursor: Option<serde_json::Value>,

    /// The secret's obfuscated value.
    pub obfuscated_secret: Option<String>,

    /// The secret's SHA-256 hash.
    pub secret_hash: Option<String>,

    /// The type of thesecret.
    pub secret_type: Option<String>,

    /// The total number of discovered secret locations.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDetectedSecretsSecretIdLocationsResponseMetaActivityFeedItem {
    /// The date and time at which the resolution status was last updated.
    pub resolved_at: Option<String>,

    /// The ID of the user that updated the secret's resolution status.
    pub resolved_by: Option<f64>,

    /// The secret's current resolution status:
    /// - `ACTIVE` — The secret is active.
    /// - `FALSE_POSITIVE` — The discovered secret is not an actual secret.
    /// - `REVOKED` — The secret is valid, but the user rotated their key to resolve the issue.
    /// - `ACCEPTED_RISK` — The Secret Scanner found the secret, but user accepts the risk of
    /// publishing it.
    pub status: Option<ResolutionEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResolutionEnum {
    #[serde(rename = "ACCEPTED_RISK")]
    AcceptedRisk,

    Active,

    #[serde(rename = "FALSE_POSITIVE")]
    FalsePositive,

    Revoked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetectedSecretsSecretIdLocationsResponse400 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_detected_secrets_secret_id_locations_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetectedSecretsSecretIdLocationsResponse401 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_detected_secrets_secret_id_locations_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetectedSecretsSecretIdLocationsResponse403 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_detected_secrets_secret_id_locations_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetectedSecretsSecretIdLocationsResponse500 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_detected_secrets_secret_id_locations_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse {
    pub environment: Option<GetEnvironmentsEnvironmentIdResponseEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentsEnvironmentIdResponseEnvironment {
    /// The date and time at which the environment was created.
    pub created_at: Option<String>,

    /// The environment's ID.
    pub id: Option<String>,

    /// If true, the environment is public.
    pub is_public: Option<bool>,

    /// The environment's name.
    pub name: Option<String>,

    /// The ID of environment's owner.
    pub owner: Option<String>,

    /// The date and time at which the environment was last updated.
    pub updated_at: Option<String>,

    /// Information about the environment's variables.
    pub values: Option<Vec<Vec<GetEnvironmentsEnvironmentIdResponseEnvironmentValuesItemItem>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponseEnvironmentValuesItemItem {
    /// If true, the variable is enabled.
    pub enabled: Option<bool>,

    /// The variable's name.
    pub key: Option<String>,

    /// The variable type.
    #[serde(rename = "type")]
    pub get_environments_environment_id_response_environment_values_item_item_type: Option<PurpleType>,

    /// The variable's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurpleType {
    Any,

    Default,

    Secret,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse400 {
    pub error: Option<GetEnvironmentsEnvironmentIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse401 {
    pub error: Option<GetEnvironmentsEnvironmentIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse500 {
    pub error: Option<GetEnvironmentsEnvironmentIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsEnvironmentIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse {
    pub environments: Option<Vec<GetEnvironmentsResponseEnvironmentsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEnvironmentsResponseEnvironmentsItem {
    /// The date and time at which the environment was created.
    pub created_at: Option<String>,

    /// The environment's ID.
    pub id: Option<String>,

    /// If true, the environment is public.
    pub is_public: Option<bool>,

    /// The environment's name.
    pub name: Option<String>,

    /// The environment owner's ID.
    pub owner: Option<String>,

    /// The environment's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the environment was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse401 {
    pub error: Option<GetEnvironmentsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse404 {
    pub error: Option<GetEnvironmentsResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse500 {
    pub error: Option<GetEnvironmentsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEnvironmentsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse {
    /// Information about operations and their usage limits. The API does not return this object
    /// for users with the [Guest
    /// role](https://learning.postman.com/docs/collaborating-in-postman/roles-and-permissions/#team-roles).
    pub operations: Option<Vec<GetMeResponseOperationsItem>>,

    /// Information about the authenticated user.
    pub user: Option<GetMeResponseUser>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponseOperationsItem {
    /// The operation's limit value.
    pub limit: Option<f64>,

    /// The operation's name.
    pub name: Option<String>,

    /// The operation's overage value.
    pub overage: Option<f64>,

    /// The operation's current usage value.
    pub usage: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponseUser {
    /// The user's avatar image URL.
    pub avatar: Option<String>,

    /// The user's email address.
    pub email: Option<String>,

    /// The user's full name.
    pub full_name: Option<String>,

    /// The user's Postman ID.
    pub id: Option<f64>,

    /// If true, the user's information is publicly available.
    pub is_public: Option<bool>,

    /// The team ID the user is assigned to. This returns a `0` value if the user is not assigned
    /// to a team.
    pub team_id: Option<i64>,

    /// The user's username.
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse401 {
    pub error: Option<GetMeResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse500 {
    pub error: Option<GetMeResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMeResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GetMocksMockIdCallLogsResponse {
    pub call_logs: Option<Vec<GetMocksMockIdCallLogsResponseCallLogsItem>>,

    /// The response's non-standard meta information.
    pub meta: Option<GetMocksMockIdCallLogsResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdCallLogsResponseCallLogsItem {
    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's request information.
    pub request: Option<GetMocksMockIdCallLogsResponseCallLogsItemRequest>,

    /// The server response's response information.
    pub response: Option<GetMocksMockIdCallLogsResponseCallLogsItemResponse>,

    /// The server response's name.
    pub response_name: Option<String>,

    /// The date and time at which the server response was served.
    pub served_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemRequest {
    /// The request's body information.
    pub body: Option<GetMocksMockIdCallLogsResponseCallLogsItemRequestBody>,

    /// The request's headers.
    pub headers: Option<GetMocksMockIdCallLogsResponseCallLogsItemRequestHeaders>,

    /// The request method.
    pub method: Option<String>,

    /// The request's path.
    pub path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemRequestBody {
    /// The request body's contents.
    pub data: Option<String>,

    /// The request body's media type (mode).
    pub mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemRequestHeaders {
    /// The request header's name.
    pub key: Option<String>,

    /// The request header's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemResponse {
    /// The response's body information.
    pub body: Option<GetMocksMockIdCallLogsResponseCallLogsItemResponseBody>,

    /// The response's headers.
    pub headers: Option<GetMocksMockIdCallLogsResponseCallLogsItemResponseHeaders>,

    /// The response's status code.
    pub status_code: Option<f64>,

    /// The type of response.
    #[serde(rename = "type")]
    pub get_mocks_mock_id_call_logs_response_call_logs_item_response_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemResponseBody {
    /// The response body's contents.
    pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemResponseHeaders {
    /// The response header's description information.
    pub description: Option<GetMocksMockIdCallLogsResponseCallLogsItemResponseHeadersDescription>,

    /// The response header's name.
    pub key: Option<String>,

    /// The response header's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponseCallLogsItemResponseHeadersDescription {
    /// The response header description's content.
    pub content: Option<String>,

    /// The response header description's media type.
    #[serde(rename = "type")]
    pub get_mocks_mock_id_call_logs_response_call_logs_item_response_headers_description_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdCallLogsResponseMeta {
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse400 {
    pub error: Option<GetMocksMockIdCallLogsResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse401 {
    pub error: Option<GetMocksMockIdCallLogsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse404 {
    pub error: Option<GetMocksMockIdCallLogsResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse500 {
    pub error: Option<GetMocksMockIdCallLogsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdCallLogsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse {
    pub mock: Option<GetMocksMockIdResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdResponseMock {
    /// The mock's associated collection unique ID.
    pub collection: Option<String>,

    /// Information about the mock server's configuration.
    pub config: Option<GetMocksMockIdResponseMockConfig>,

    /// The date and time at which the mock server was created.
    pub created_at: Option<String>,

    /// If true, the mock server is not active. Mock servers deactivate when a linked collection
    /// or environment is deleted.
    pub deactivated: Option<bool>,

    /// The mock server's associated environment ID.
    pub environment: Option<String>,

    /// The mock server's ID.
    pub id: Option<String>,

    /// If true, the mock server is public.
    pub is_public: Option<bool>,

    /// The mock server URL.
    pub mock_url: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// The ID of mock server's owner.
    pub owner: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the mock server was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdResponseMockConfig {
    /// A list of the mock server's headers.
    pub headers: Option<Vec<Option<serde_json::Value>>>,

    /// If true, match the request body.
    pub match_body: Option<bool>,

    /// If true, match query parameters.
    pub match_query_params: Option<bool>,

    /// If true, use wildcard variable matching.
    pub match_wildcards: Option<bool>,

    /// The ID of mock server's default response for requests. All calls to the mock server will
    /// return the defined response.
    pub server_response_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse401 {
    pub error: Option<GetMocksMockIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse404 {
    pub error: Option<GetMocksMockIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse500 {
    pub error: Option<GetMocksMockIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse401 {
    pub error: Option<GetMocksMockIdServerResponsesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse404 {
    pub error: Option<GetMocksMockIdServerResponsesResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse500 {
    pub error: Option<GetMocksMockIdServerResponsesResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdServerResponsesResponseItem {
    /// The date and time at which the server response was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the server response.
    pub created_by: Option<String>,

    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code.
    pub status_code: Option<f64>,

    /// The date and time at which the server response was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who last updated the server response.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse400 {
    pub error: Option<GetMocksMockIdServerResponsesServerResponseIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse401 {
    pub error: Option<GetMocksMockIdServerResponsesServerResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse404 {
    pub error: Option<GetMocksMockIdServerResponsesServerResponseIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse500 {
    pub error: Option<GetMocksMockIdServerResponsesServerResponseIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksMockIdServerResponsesServerResponseIdResponseItem {
    /// The date and time at which the server response was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the server response.
    pub created_by: Option<String>,

    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code.
    pub status_code: Option<f64>,

    /// The date and time at which the server response was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who last updated the server response.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse {
    pub mocks: Option<Vec<GetMocksResponseMocksItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksResponseMocksItem {
    /// The mock's associated collection unique ID.
    pub collection: Option<String>,

    /// Information about the mock server's configuration.
    pub config: Option<GetMocksResponseMocksItemConfig>,

    /// The date and time at which the mock server was created.
    pub created_at: Option<String>,

    /// The mock server's associated environment ID.
    pub environment: Option<String>,

    /// The mock server's ID.
    pub id: Option<String>,

    /// If true, the mock server is public.
    pub is_public: Option<bool>,

    /// The mock server URL.
    pub mock_url: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// The ID of mock server's owner.
    pub owner: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the mock server was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMocksResponseMocksItemConfig {
    /// Information about the mock server's simulated network delay settings. This returns a null
    /// value if there are no configured network delay settings.
    pub delay: Option<GetMocksResponseMocksItemConfigDelay>,

    /// A list of the mock server's headers.
    pub headers: Option<Vec<String>>,

    /// If true, match the request body.
    pub match_body: Option<bool>,

    /// If true, match query parameters.
    pub match_query_params: Option<bool>,

    /// If true, use wildcard variable matching.
    pub match_wildcards: Option<bool>,

    /// The ID of mock server's default response for requests. All calls to the mock server will
    /// return the defined response.
    pub server_response_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponseMocksItemConfigDelay {
    /// The configured delay, in milliseconds.
    pub duration: Option<i64>,

    /// The simulated fixed network delay value:
    ///
    /// - `1` — 2G (300 ms).
    /// - `2` — 3G (100 ms).
    ///
    /// The object does not return this value for custom delay values.
    pub preset: Option<Preset>,

    /// The type of simulated delay value:
    ///
    /// - `fixed` — The delay value is a fixed value.
    #[serde(rename = "type")]
    pub get_mocks_response_mocks_item_config_delay_type: Option<GetMocksResponseMocksItemConfigDelayType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GetMocksResponseMocksItemConfigDelayType {
    Fixed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Preset {
    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse401 {
    pub error: Option<GetMocksResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse500 {
    pub error: Option<GetMocksResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMocksResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse {
    pub monitor: Option<GetMonitorsMonitorIdResponseMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitorsMonitorIdResponseMonitor {
    /// The monitor's associated collection unique ID.
    pub collection_uid: Option<String>,

    /// A list of the monitor's [geographic
    /// regions](https://learning.postman.com/docs/monitoring-your-api/setting-up-monitor/#adding-regions).
    pub distribution: Option<Vec<Option<serde_json::Value>>>,

    /// The monitor's associated environment unique ID.
    pub environment_uid: Option<String>,

    /// The monitor's ID.
    pub id: Option<String>,

    /// Information about the monitor's previous run.
    pub last_run: Option<GetMonitorsMonitorIdResponseMonitorLastRun>,

    /// The monitor's name.
    pub name: Option<String>,

    /// Information about the monitor's notification settings.
    pub notifications: Option<GetMonitorsMonitorIdResponseMonitorNotifications>,

    /// Information about the monitor's option settings.
    pub options: Option<GetMonitorsMonitorIdResponseMonitorOptions>,

    /// The ID of monitor's owner.
    pub owner: Option<f64>,

    /// Information about the monitor's schedule.
    pub schedule: Option<GetMonitorsMonitorIdResponseMonitorSchedule>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitorsMonitorIdResponseMonitorLastRun {
    /// The date and time at which the monitor's previous run completed.
    pub finished_at: Option<String>,

    /// The date and time at which the monitor's previous run started.
    pub started_at: Option<String>,

    /// Information about the monitor's stats.
    pub stats: Option<GetMonitorsMonitorIdResponseMonitorLastRunStats>,

    /// The monitor's status after its last run.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponseMonitorLastRunStats {
    /// Information about the monitor's assertions.
    pub assertions: Option<GetMonitorsMonitorIdResponseMonitorLastRunStatsAssertions>,

    /// Information about the monitor's requests.
    pub requests: Option<GetMonitorsMonitorIdResponseMonitorLastRunStatsRequests>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponseMonitorLastRunStatsAssertions {
    /// The total number of test failures.
    pub failed: Option<f64>,

    /// The total number of tests performed.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponseMonitorLastRunStatsRequests {
    /// The total number of requests.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitorsMonitorIdResponseMonitorNotifications {
    pub on_error: Option<Vec<GetMonitorsMonitorIdResponseMonitorNotificationsOnErrorItem>>,

    pub on_failure: Option<Vec<GetMonitorsMonitorIdResponseMonitorNotificationsOnFailureItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponseMonitorNotificationsOnErrorItem {
    /// The email address of the user to notify on monitor error.
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponseMonitorNotificationsOnFailureItem {
    /// The email address of the user to notify on monitor failure.
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitorsMonitorIdResponseMonitorOptions {
    /// If true, follow redirects enabled.
    pub follow_redirects: Option<bool>,

    /// The monitor's request delay value.
    pub request_delay: Option<f64>,

    /// The monitor's request timeout value.
    pub request_timeout: Option<f64>,

    /// If true, strict SSL enabled.
    #[serde(rename = "strictSSL")]
    pub strict_ssl: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMonitorsMonitorIdResponseMonitorSchedule {
    /// The monitor's cron frequency value.
    pub cron: Option<String>,

    /// The date and time of monitor's next scheduled run.
    pub next_run: Option<String>,

    /// The monitor's timezone.
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse401 {
    pub error: Option<GetMonitorsMonitorIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse404 {
    pub error: Option<GetMonitorsMonitorIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse500 {
    pub error: Option<GetMonitorsMonitorIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsMonitorIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse {
    pub monitors: Option<Vec<GetMonitorsResponseMonitorsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponseMonitorsItem {
    /// The monitor's ID.
    pub id: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// The ID of the monitor's owner.
    pub owner: Option<String>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse401 {
    pub error: Option<GetMonitorsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse500 {
    pub error: Option<GetMonitorsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMonitorsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse {
    /// The response's non-standard meta information.
    pub meta: Option<GetNetworkPrivateNetworkEntityRequestAllResponseMeta>,

    /// Information about the requests to add elements to the Private API Network.
    pub requests: Option<Vec<GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponseMeta {
    /// The maximum number of items returned.
    pub limit: Option<i64>,

    /// The zero-based offset of the first item returned.
    pub offset: Option<i64>,

    /// The total count of items found.
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItem {
    /// The date and time at which the request was created.
    pub created_at: Option<String>,

    /// The ID of the user who created the request.
    pub created_by: Option<i64>,

    /// Information about the requested element.
    pub element: Option<GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItemElement>,

    /// The request's ID.
    pub id: Option<i64>,

    /// The user's optional message included in the request.
    pub message: Option<String>,

    /// Information about the response to the request. This object only returns when the network
    /// manager denied a request with a message.
    pub response: Option<GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItemResponse>,

    /// The request's status.
    pub status: Option<PurpleStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItemElement {
    /// The element's ID.
    pub id: Option<String>,

    /// The element's name.
    pub name: Option<String>,

    /// The element's short summary.
    pub summary: Option<String>,

    /// The element type.
    #[serde(rename = "type")]
    pub get_network_private_network_entity_request_all_response_requests_item_element_type: Option<EntityTypeEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityTypeEnum {
    Api,

    Collection,

    Workspace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponseRequestsItemResponse {
    /// The date and time at which the network manager denied the request.
    pub created_at: Option<String>,

    /// The network manager's user ID.
    pub created_by: Option<i64>,

    /// The network manager's request response message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurpleStatus {
    Denied,

    Pending,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse400 {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse401 {
    pub error: Option<GetNetworkPrivateNetworkEntityRequestAllResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse403 {
    pub error: Option<GetNetworkPrivateNetworkEntityRequestAllResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse500 {
    pub error: Option<GetNetworkPrivateNetworkEntityRequestAllResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateNetworkEntityRequestAllResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse {
    /// Information about a Private API Network's folder elements. Elements are APIs,
    /// collections, and workspaces.
    pub elements: Option<Vec<GetNetworkPrivateResponseElementsItem>>,

    /// Information about the Private API Network's folders.
    pub folders: Option<Vec<GetNetworkPrivateResponseFoldersItem>>,

    /// The response's non-standard meta information.
    pub meta: Option<GetNetworkPrivateResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateResponseElementsItem {
    /// The date and time at which the element was published to Private API Network. This value
    /// is the same as the `updatedAt` value.
    pub added_at: Option<String>,

    /// The user ID of the user who published the element.
    pub added_by: Option<i64>,

    /// The date and time at which the element was created.
    pub created_at: Option<String>,

    /// The user who created the element.
    pub created_by: Option<i64>,

    /// The element's description.
    pub description: Option<String>,

    /// The element's HREF.
    pub href: Option<String>,

    /// The element's ID.
    pub id: Option<String>,

    /// The element's name.
    pub name: Option<String>,

    /// The element's parent folder ID.
    pub parent_folder_id: Option<i64>,

    /// The element's summary.
    pub summary: Option<String>,

    /// The element's type.
    #[serde(rename = "type")]
    pub get_network_private_response_elements_item_type: Option<String>,

    /// The date and time at which the element was last updated.
    pub updated_at: Option<String>,

    /// The user who updated the element.
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateResponseFoldersItem {
    /// The date and time at which the folder was created.
    pub created_at: Option<String>,

    /// The user who created the folder.
    pub created_by: Option<i64>,

    /// The folder's description.
    pub description: Option<String>,

    /// The folder's ID.
    pub id: Option<i64>,

    /// The folder's name.
    pub name: Option<String>,

    /// The folder's parent folder ID.
    pub parent_folder_id: Option<i64>,

    /// The element's type. This value is always `folder`.
    #[serde(rename = "type")]
    pub get_network_private_response_folders_item_type: Option<String>,

    /// The date and time at which the folder was updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who updated the folder.
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNetworkPrivateResponseMeta {
    /// The maximum number of elements returned. If the value exceeds the maximum value of
    /// `1000`, then the system uses the `1000` value.
    pub limit: Option<i64>,

    /// The zero-based offset of the first item returned.
    pub offset: Option<i64>,

    /// The total count of the `elements` and `folders` items.
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse401 {
    pub error: Option<GetNetworkPrivateResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse403 {
    pub error: Option<GetNetworkPrivateResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse500 {
    pub error: Option<GetNetworkPrivateResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNetworkPrivateResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2GroupsGroupIdResponse {
    /// The group's external ID.
    pub external_id: Option<String>,

    /// The group's SCIM ID.
    pub id: Option<String>,

    /// Information about the group's members.
    pub members: Option<GetScimV2GroupsGroupIdResponseMembers>,

    /// The response's non-standard meta information.
    pub meta: Option<GetScimV2GroupsGroupIdResponseMeta>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The group's display name.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponseMembers {
    pub display: Option<String>,

    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2GroupsGroupIdResponseMeta {
    /// The date and time at which the group was created.
    pub created: Option<String>,

    /// The date and time at which the group was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    /// The error status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsGroupIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2GroupsResponse {
    /// The number of items per response page.
    pub items_per_page: Option<f64>,

    /// A list of group resources.
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<ScimGroupResource>>,

    pub schemas: Option<Vec<String>>,

    /// The index entry by which the returned results begin.
    pub start_index: Option<f64>,

    /// The total number of results found.
    pub total_results: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroupResource {
    /// The group's display name.
    pub display_name: Option<String>,

    /// The group's external ID.
    pub external_id: Option<String>,

    /// The group's SCIM ID.
    pub id: Option<String>,

    /// Information about the group's members.
    pub members: Option<ScimGroupResourceMembers>,

    /// The response's non-standard meta information.
    pub meta: Option<ScimGroupResourceMeta>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScimGroupResourceMembers {
    pub display: Option<String>,

    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroupResourceMeta {
    /// The date and time at which the group was created.
    pub created: Option<String>,

    /// The date and time at which the group was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsResponse400 {
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2GroupsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ResourceTypesResponse401 {
    pub error: Option<GetScimV2ResourceTypesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ResourceTypesResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ResourceTypesResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ResourceTypesResponseItem {
    /// The resource's description.
    pub description: Option<String>,

    /// The resource's endpoint.
    pub endpoint: Option<String>,

    /// The resource's ID.
    pub id: Option<String>,

    /// The resource's friendly name.
    pub name: Option<String>,

    /// The resource's schema URI.
    pub schema: Option<String>,

    /// Information about the resource's schema extensions.
    pub schema_extensions: Option<Vec<GetScimV2ResourceTypesResponseItemSchemaExtensionsItem>>,

    /// A list of SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ResourceTypesResponseItemSchemaExtensionsItem {
    /// If true, the resource must include this schema extension.
    pub required: Option<bool>,

    /// The resource extension's URI.
    pub schema: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ServiceProviderConfigResponse {
    /// A list of authentication schemes.
    pub authentication_schemes: Option<Vec<GetScimV2ServiceProviderConfigResponseAuthenticationSchemesItem>>,

    pub bulk: Option<GetScimV2ServiceProviderConfigResponseBulk>,

    pub change_password: Option<GetScimV2ServiceProviderConfigResponseChangePassword>,

    /// A link to the URI's documentation.
    pub documentation_uri: Option<String>,

    pub etag: Option<GetScimV2ServiceProviderConfigResponseEtag>,

    pub filter: Option<GetScimV2ServiceProviderConfigResponseFilter>,

    /// The response's non-standard meta information.
    pub meta: Option<GetScimV2ServiceProviderConfigResponseMeta>,

    pub patch: Option<GetScimV2ServiceProviderConfigResponsePatch>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    pub sort: Option<GetScimV2ServiceProviderConfigResponseSort>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ServiceProviderConfigResponseAuthenticationSchemesItem {
    /// The scheme's description.
    pub description: Option<String>,

    /// The scheme's friendly name.
    pub name: Option<String>,

    /// A link to the scheme's specification documentation.
    pub spec_uri: Option<String>,

    /// The scheme's type.
    #[serde(rename = "type")]
    pub get_scim_v2_service_provider_config_response_authentication_schemes_item_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ServiceProviderConfigResponseBulk {
    pub max_operations: Option<f64>,

    pub max_payload_size: Option<f64>,

    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponseChangePassword {
    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponseEtag {
    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ServiceProviderConfigResponseFilter {
    pub max_results: Option<f64>,

    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2ServiceProviderConfigResponseMeta {
    pub location: Option<String>,

    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponsePatch {
    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponseSort {
    pub supported: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2ServiceProviderConfigResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2UsersResponse {
    /// The number of items per response page.
    pub items_per_page: Option<f64>,

    /// A list of user resources.
    #[serde(rename = "Resources")]
    pub resources: Option<Vec<ScimUserResource>>,

    pub schemas: Option<Vec<String>>,

    /// The index entry by which the returned results begin.
    pub start_index: Option<f64>,

    /// The total number of results found.
    pub total_results: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUserResource {
    /// If true, the team member is active.
    pub active: Option<bool>,

    /// The team member's external ID.
    pub external_id: Option<String>,

    /// The team member's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<ScimUserResourceMeta>,

    /// Information about the Postman team member.
    pub name: Option<ScimUserResourceName>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The team member's SCIM username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUserResourceMeta {
    /// The date and time at which the team member was created.
    pub created: Option<String>,

    /// The date and time at which the team member was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUserResourceName {
    /// The team member's last name.
    pub family_name: Option<String>,

    /// The team member's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersResponse400 {
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersResponse403 {
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2UsersUserIdResponse {
    /// If true, the team member is active.
    pub active: Option<bool>,

    /// The team member's external ID.
    pub external_id: Option<String>,

    /// The team member's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<GetScimV2UsersUserIdResponseMeta>,

    /// Information about the Postman team member.
    pub name: Option<GetScimV2UsersUserIdResponseName>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The team member's SCIM username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2UsersUserIdResponseMeta {
    /// The date and time at which the team member was created.
    pub created: Option<String>,

    /// The date and time at which the team member was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetScimV2UsersUserIdResponseName {
    /// The team member's last name.
    pub family_name: Option<String>,

    /// The team member's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    /// The error status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    /// The error status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScimV2UsersUserIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponse {
    pub data: Option<Vec<GetSecretTypesResponseDataItem>>,

    pub meta: Option<GetSecretTypesResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponseDataItem {
    /// The ID of the secret type.
    pub id: Option<String>,

    /// The name of the secret type.
    pub name: Option<String>,

    /// The origin of the secret type:
    /// - `DEFAULT` — Supported by default in Postman.
    /// - `TEAM_REGEX` — A custom regex added by an Admin or Super Admin user in the **Configure
    /// Alerts** section of the [**Team
    /// Settings**](https://learning.postman.com/docs/administration/team-settings/) interface.
    #[serde(rename = "type")]
    pub get_secret_types_response_data_item_type: Option<DatumType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatumType {
    Default,

    #[serde(rename = "TEAM_REGEX")]
    TeamRegex,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponseMeta {
    /// The total number of supported secrets.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponse401 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_secret_types_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponse403 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_secret_types_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSecretTypesResponse500 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub get_secret_types_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponse {
    /// An object containing the paginated elements.
    pub data: Option<GetTagsSlugEntitiesResponseData>,

    /// The response's pagination information.
    pub meta: Option<GetTagsSlugEntitiesResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponseData {
    /// A list of the Postman elements that contain the given tag.
    pub entities: Vec<GetTagsSlugEntitiesResponseDataEntitiesItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTagsSlugEntitiesResponseDataEntitiesItem {
    /// The element's unique ID.
    pub entity_id: Option<String>,

    /// The type of Postman element.
    pub entity_type: Option<EntityTypeEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTagsSlugEntitiesResponseMeta {
    /// The number of tagged elements returned in the response.
    pub count: i64,

    /// The pagination cursor that points to the next record in the results set.
    pub next_cursor: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTagsSlugEntitiesResponse400 {
    /// A short summary of the problem.
    pub error: Option<String>,

    /// Information about the error.
    pub message: Option<String>,

    /// The error's HTTP status code.
    pub status_code: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_tags_slug_entities_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_tags_slug_entities_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_tags_slug_entities_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTagsSlugEntitiesResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_tags_slug_entities_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse {
    pub workspaces: Option<Vec<GetWorkspacesResponseWorkspacesItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponseWorkspacesItem {
    /// The workspace's ID.
    pub id: Option<String>,

    /// The workspace's name.
    pub name: Option<String>,

    /// The type of workspace.
    #[serde(rename = "type")]
    pub get_workspaces_response_workspaces_item_type: Option<VisibilityEnum>,

    /// The workspace's visibility.
    /// [Visibility](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility)
    /// determines who can access the workspace:
    ///
    /// - `personal` — Only you can access the workspace.
    /// - `team` — All team members can access the workspace.
    /// - `private` — Only invited team members can access the workspace ([Professional and
    /// Enterprise plans only](https://www.postman.com/pricing)).
    /// - `public` — Everyone can access the workspace.
    /// - `partner` — Only invited team members and
    /// [partners](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/partner-workspaces/)
    /// can access the workspace ([Enterprise Ultimate plans](https://www.postman.com/pricing)
    /// only).
    pub visibility: Option<VisibilityEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityEnum {
    Partner,

    Personal,

    Private,

    Public,

    Team,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse401 {
    pub error: Option<GetWorkspacesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse500 {
    pub error: Option<GetWorkspacesResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdGlobalVariablesResponse {
    /// A list of the workspace's global variables.
    pub values: Option<Vec<GlobalVariable>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalVariable {
    /// If true, the variable is enabled.
    pub enabled: Option<bool>,

    /// The variable's name.
    pub key: Option<String>,

    /// The [type](https://learning.postman.com/docs/sending-requests/variables/#variable-types)
    /// of variable.
    #[serde(rename = "type")]
    pub global_variable_type: Option<GlobalVariableType>,

    /// The variable's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GlobalVariableType {
    Default,

    Secret,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdGlobalVariablesResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The type of error.
    pub title: Option<String>,

    /// The generic description for the error's class.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_global_variables_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse {
    /// Information about the workspace.
    pub workspace: Option<GetWorkspacesWorkspaceIdResponseWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkspacesWorkspaceIdResponseWorkspace {
    /// The workspace's APIs.
    pub apis: Option<Vec<GetWorkspacesWorkspaceIdResponseWorkspaceApisItem>>,

    /// The workspace's collections.
    pub collections: Option<Vec<GetWorkspacesWorkspaceIdResponseWorkspaceCollectionsItem>>,

    /// The date and time at which the workspace was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the workspace.
    pub created_by: Option<String>,

    /// The workspace's description.
    pub description: Option<String>,

    /// The workspace's environments.
    pub environments: Option<Vec<GetWorkspacesWorkspaceIdResponseWorkspaceEnvironmentsItem>>,

    /// The workspace's ID.
    pub id: Option<String>,

    /// The workspace's mock servers.
    pub mocks: Option<Vec<GetWorkspacesWorkspaceIdResponseWorkspaceMocksItem>>,

    /// The workspace's monitors.
    pub monitors: Option<Vec<GetWorkspacesWorkspaceIdResponseWorkspaceMonitorsItem>>,

    /// The workspace's name.
    pub name: Option<String>,

    /// The type of workspace.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_response_workspace_type: Option<VisibilityEnum>,

    /// The date and time at which the workspace was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who last updated the workspace.
    pub updated_by: Option<String>,

    /// The workspace's visibility.
    /// [Visibility](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility)
    /// determines who can access the workspace:
    ///
    /// - `personal` — Only you can access the workspace.
    /// - `team` — All team members can access the workspace.
    /// - `private` — Only invited team members can access the workspace ([Professional and
    /// Enterprise plans only](https://www.postman.com/pricing)).
    /// - `public` — Everyone can access the workspace.
    /// - `partner` — Only invited team members and
    /// [partners](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/partner-workspaces/)
    /// can access the workspace ([Enterprise Ultimate plans](https://www.postman.com/pricing)
    /// only).
    pub visibility: Option<VisibilityEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponseWorkspaceApisItem {
    /// The API's ID.
    pub id: Option<String>,

    /// The API's name.
    pub name: Option<String>,

    /// The API's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponseWorkspaceCollectionsItem {
    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponseWorkspaceEnvironmentsItem {
    /// The environment's ID.
    pub id: Option<String>,

    /// The environment's name.
    pub name: Option<String>,

    /// The environment's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponseWorkspaceMocksItem {
    /// If true, the mock server is not active. Mock servers deactivate when a linked collection
    /// or environment is deleted.
    pub deactivated: Option<bool>,

    /// The mock server's ID.
    pub id: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponseWorkspaceMonitorsItem {
    /// The monitor's ID.
    pub id: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse401 {
    pub error: Option<GetWorkspacesWorkspaceIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse404 {
    pub error: Option<GetWorkspacesWorkspaceIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWorkspacesWorkspaceIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,

    /// The error's HTTP status code.
    pub status_code: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse500 {
    pub error: Option<GetWorkspacesWorkspaceIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<GetWorkspacesWorkspaceIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_tags_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWorkspacesWorkspaceIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub get_workspaces_workspace_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanElementCreated {
    /// The date and time at which the element was added.
    pub added_at: Option<String>,

    /// The user who added the element.
    pub added_by: Option<i64>,

    /// The date and time at which the element was created.
    pub created_at: Option<String>,

    /// The user who created the element.
    pub created_by: Option<i64>,

    /// The element's description.
    pub description: Option<String>,

    /// A list of the element's environments.
    pub environments: Option<Vec<String>>,

    /// The element's Postman URL.
    pub href: Option<String>,

    /// The element's ID or UID.
    pub id: Option<String>,

    /// The element's name.
    pub name: Option<String>,

    /// The parent folder's ID.
    pub parent_folder_id: Option<i64>,

    /// The element's summary.
    pub summary: Option<String>,

    /// The element's type.
    #[serde(rename = "type")]
    pub pan_element_created_type: Option<String>,

    /// The date and time at which the element was last updated.
    pub updated_at: Option<String>,

    /// The user who last updated the element.
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PanFolderCreated {
    /// The date and time at which the element was created.
    pub created_at: Option<String>,

    /// The user who created the folder.
    pub created_by: Option<i64>,

    /// The folder's description.
    pub description: Option<String>,

    /// The folder's ID.
    pub id: Option<i64>,

    /// The folder's name.
    pub name: Option<String>,

    /// The parent folder ID.
    pub parent_folder_id: Option<i64>,

    /// The folder's type. This is always the `folder` value.
    #[serde(rename = "type")]
    pub pan_folder_created_type: Option<String>,

    /// The date and time at which the folder was updated.
    pub updated_at: Option<String>,

    /// The user who updated the folder.
    pub updated_by: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdBody {
    pub collection: Option<PatchCollectionsCollectionIdBodyCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdBodyCollection {
    pub auth: Option<serde_json::Value>,

    pub events: Option<serde_json::Value>,

    /// An object that contains the collection's updated name and description.
    pub info: Option<PatchCollectionsCollectionIdBodyCollectionInfo>,

    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdBodyCollectionInfo {
    /// The collection's updated description.
    pub description: Option<String>,

    /// The collection's updated name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse {
    pub collection: Option<PatchCollectionsCollectionIdResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponseCollection {
    /// The collection's updated description.
    pub description: Option<String>,

    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's updated name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse400 {
    pub error: Option<PatchCollectionsCollectionIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse401 {
    pub error: Option<PatchCollectionsCollectionIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse403 {
    pub error: Option<PatchCollectionsCollectionIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse404 {
    pub error: Option<PatchCollectionsCollectionIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse404Error {
    /// Information about the error.
    pub details: Option<PatchCollectionsCollectionIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse404ErrorDetails {
    /// The collection ID.
    pub id: Option<String>,

    /// The instance item.
    pub item: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse500 {
    pub error: Option<PatchCollectionsCollectionIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchCollectionsCollectionIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdBody {
    /// Information about the group update operation.
    #[serde(rename = "Operations")]
    pub operations: Option<Vec<PatchScimV2GroupsGroupIdBodyOperationsItem>>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdBodyOperationsItem {
    /// The operation to perform.
    pub op: Option<PurpleOp>,

    /// The operation's path. Include this value when you update a group's members.
    pub path: Option<String>,

    /// The performed operation's value.
    pub value: Option<PatchScimV2GroupsGroupIdBodyOperationsItemValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PurpleOp {
    Add,

    Remove,

    Replace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2GroupsGroupIdBodyOperationsItemValue {
    /// The group's name.
    pub display_name: Option<String>,

    /// The group's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2GroupsGroupIdResponse {
    /// The group's name.
    pub display_name: Option<String>,

    /// The group's external ID.
    pub external_id: Option<String>,

    /// The group's ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<PatchScimV2GroupsGroupIdResponseMeta>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2GroupsGroupIdResponseMeta {
    /// The date and time at which the group was created.
    pub created: Option<String>,

    /// The date and time at which the group was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2GroupsGroupIdResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// A list of SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2GroupsGroupIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdBody {
    /// Information about the user update operation.
    #[serde(rename = "Operations")]
    pub operations: Option<Vec<PatchScimV2UsersUserIdBodyOperationsItem>>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdBodyOperationsItem {
    /// The operation to perform.
    pub op: Option<FluffyOp>,

    /// The performed operation's value.
    pub value: Option<PatchScimV2UsersUserIdBodyOperationsItemValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluffyOp {
    Replace,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdBodyOperationsItemValue {
    /// Sets the user's `active` state:
    /// - `true` — Activates the user. This lets them authenticate in to your Postman team.
    /// - `false` — Removes the user from your Postman team and deactivates the account. This
    /// blocks the user from authenticating in to Postman.
    pub active: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2UsersUserIdResponse {
    /// If true, the team member is active.
    pub active: Option<bool>,

    /// The team member's external ID.
    pub external_id: Option<String>,

    /// The team member's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<PatchScimV2UsersUserIdResponseMeta>,

    /// Information about the Postman team member.
    pub name: Option<PatchScimV2UsersUserIdResponseName>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The team member's SCIM username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2UsersUserIdResponseMeta {
    /// The date and time at which the team member was created.
    pub created: Option<String>,

    /// The date and time at which the team member was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchScimV2UsersUserIdResponseName {
    /// The team member's last name.
    pub family_name: Option<String>,

    /// The team member's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse400 {
    pub detail: Option<String>,

    pub schemas: Option<Vec<String>>,

    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PatchScimV2UsersUserIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdCollectionsResponse {
    /// The collection's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdCollectionsResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_collections_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdCollectionsResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_collections_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdCollectionsResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_collections_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdCollectionsResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_collections_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasBody {
    /// The list of files that are part of the schema.
    pub files: Vec<PostApisApiIdSchemasBodyFilesItem>,

    /// The schema's type.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_body_type: PostApisApiIdSchemasBodyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasBodyFilesItem {
    /// The schema file's stringified contents.
    pub content: Option<String>,

    /// The schema's file path.
    pub path: Option<String>,

    /// Information about the schema's root file.
    pub root: Option<PostApisApiIdSchemasBodyFilesItemRoot>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasBodyFilesItemRoot {
    /// If true, tag the file as the root file. The root tag is only allowed for protobuf
    /// specifications.
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostApisApiIdSchemasBodyType {
    #[serde(rename = "asyncapi:2")]
    Asyncapi2,

    Graphql,

    #[serde(rename = "openapi:1")]
    Openapi1,

    #[serde(rename = "openapi:2")]
    Openapi2,

    #[serde(rename = "openapi:3")]
    Openapi3,

    #[serde(rename = "openapi:3_1")]
    Openapi3_1,

    #[serde(rename = "proto:2")]
    Proto2,

    #[serde(rename = "proto:3")]
    Proto3,

    #[serde(rename = "raml:0_8")]
    Raml0_8,

    #[serde(rename = "raml:1")]
    Raml1,

    #[serde(rename = "wsdl:1")]
    Wsdl1,

    #[serde(rename = "wsdl:2")]
    Wsdl2,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostApisApiIdSchemasResponse {
    /// The date and time at which the schema was created.
    pub created_at: Option<String>,

    /// The user ID of the user that created the schema.
    pub created_by: Option<String>,

    /// The list of the schema's files.
    pub files: Option<Vec<PostApisApiIdSchemasResponseFilesItem>>,

    /// The schema's ID.
    pub id: Option<String>,

    /// The schema's type.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response_type: Option<PostApisApiIdSchemasBodyType>,

    /// The date and time at which the schema was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that updated the schema.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostApisApiIdSchemasResponseFilesItem {
    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<String>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse400 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse403 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdSchemasResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_schemas_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostApisApiIdVersionsResponse {
    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// Information about the API version release. For example, changelog notes.
    pub release_notes: Option<String>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdVersionsResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_versions_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdVersionsResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_versions_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdVersionsResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_versions_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdVersionsResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_api_id_versions_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisApiIdVersionsResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_api_id_versions_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisBody {
    /// The API's description. This supports Markdown formatting.
    pub description: Option<String>,

    /// The API's name.
    pub name: String,

    /// The API's short summary.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostApisResponse {
    /// The date and time at which the API was created.
    pub created_at: Option<String>,

    /// The Postman ID of the user that created the API.
    pub created_by: Option<f64>,

    /// The API's description.
    pub description: Option<String>,

    /// The API's ID.
    pub id: Option<String>,

    /// The API's name.
    pub name: Option<String>,

    /// The API's short summary.
    pub summary: Option<String>,

    /// The date and time at which the API was updated.
    pub updated_at: Option<String>,

    /// The Postman ID of the user that updated the API.
    pub updated_by: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisResponse400 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub post_apis_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostApisResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_apis_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsBody {
    /// For a complete list of values, refer to the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub collection: Option<PostCollectionsBodyCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsBodyCollection {
    /// An object that contains basic information about the collection. For a complete list of
    /// values, refer to the `definitions.info` entry in the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub info: PostCollectionsBodyCollectionInfo,

    /// Information about the collection's HTTP requests and responses. For a complete list of
    /// values, refer to the `definitions.item` entry in the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub item: Option<Vec<PostCollectionsBodyCollectionItemItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsBodyCollectionInfo {
    /// The collection's name.
    pub name: String,

    /// A URL to the collection's schema.
    pub schema: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsBodyCollectionItemItem {
    pub request: Option<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersBody {
    /// The ID of a folder in which to create the folder.
    pub folder: Option<String>,

    /// The folder's name. It is recommended that you pass the `name` property in the request
    /// body. If you do not, the system uses a null value. As a result, this creates a folder
    /// with a blank name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse {
    /// Information about the collection folder. For a complete list of properties, refer to the
    /// [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PostCollectionsCollectionIdFoldersResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCollectionsCollectionIdFoldersResponseData {
    /// The collection ID that the folder belongs to.
    pub collection: Option<String>,

    /// The folder's creation date and time.
    pub created_at: Option<String>,

    /// The folder's description.
    pub description: Option<String>,

    /// Information about the folder.
    pub folder: Option<String>,

    /// A list of folders.
    pub folders: Option<Vec<String>>,

    /// A list of folder IDs and their order in the collection.
    #[serde(rename = "folders_order")]
    pub folders_order: Option<Vec<String>>,

    /// The folder's ID.
    pub id: Option<String>,

    /// The user ID of the user that last updated the folder.
    pub last_updated_by: Option<String>,

    /// The folder's name.
    pub name: Option<String>,

    /// A list of request IDs and their order in the created folder.
    pub order: Option<Vec<String>>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,

    /// A list of requests.
    pub requests: Option<Vec<String>>,

    /// The date and time at which the folder was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse400 {
    pub error: Option<PostCollectionsCollectionIdFoldersResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse400Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdFoldersResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse401 {
    pub error: Option<PostCollectionsCollectionIdFoldersResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse401Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdFoldersResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdFoldersResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_collections_collection_id_folders_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsBody {
    /// The request's name. It is recommended that you pass the `name` property in the request
    /// body. If you do not, the system uses a null value. As a result, this creates a request
    /// with a blank name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse {
    /// Information about the created request. For a complete list of properties, refer to the
    /// `definitions.request` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PostCollectionsCollectionIdRequestsResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCollectionsCollectionIdRequestsResponseData {
    /// The collection ID that the request belongs to.
    pub collection: Option<String>,

    /// The requeset's creation date and time.
    pub created_at: Option<String>,

    /// Information about the request's parent folder.
    pub folder: Option<String>,

    /// The request's ID.
    pub id: Option<String>,

    /// The user ID of the user that last updated the folder.
    pub last_updated_by: Option<String>,

    /// The request's name.
    pub name: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,

    /// A list of the request's responses.
    pub responses: Option<Vec<String>>,

    /// A list of response IDs and their order in the folder.
    #[serde(rename = "responses_order")]
    pub responses_order: Option<Vec<String>>,

    /// The date and time at which the request was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse400 {
    pub error: Option<PostCollectionsCollectionIdRequestsResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse400Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdRequestsResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse401 {
    pub error: Option<PostCollectionsCollectionIdRequestsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse401Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdRequestsResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdRequestsResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_collections_collection_id_requests_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesBody {
    /// The response's name. It is recommended that you pass the `name` property in the request
    /// body. If you do not, the system uses a null value. As a result, this creates a response
    /// with a blank name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse {
    /// Information about the response. For a complete list of properties, refer to the
    /// `definitions.request.responses` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PostCollectionsCollectionIdResponsesResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCollectionsCollectionIdResponsesResponseData {
    /// The date and time at which the response was created.
    pub created_at: Option<String>,

    /// The response's ID.
    pub id: Option<String>,

    /// The user ID of the user who last updated the response.
    pub last_updated_by: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,

    /// The request ID of the response's associated request.
    pub request: Option<String>,

    /// The date and time at which the response was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse400 {
    pub error: Option<PostCollectionsCollectionIdResponsesResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse400Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdResponsesResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse401 {
    pub error: Option<PostCollectionsCollectionIdResponsesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse401Error {
    /// Information about the error.
    pub details: Option<PostCollectionsCollectionIdResponsesResponse401ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsCollectionIdResponsesResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_collections_collection_id_responses_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdBody {
    /// The fork's label.
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse {
    /// Information about the forked collection.
    pub collection: Option<PostCollectionsForkCollectionIdResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponseCollection {
    /// Information about the collection's fork.
    pub fork: Option<PostCollectionsForkCollectionIdResponseCollectionFork>,

    /// The forked collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The forked collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostCollectionsForkCollectionIdResponseCollectionFork {
    /// The fork's creation date and time.
    pub created_at: Option<String>,

    /// The unique ID of fork's source collection.
    pub from: Option<String>,

    /// The fork's label.
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse401 {
    pub error: Option<PostCollectionsForkCollectionIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse404 {
    pub error: Option<PostCollectionsForkCollectionIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse404Error {
    /// Information about the error.
    pub details: Option<PostCollectionsForkCollectionIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse404ErrorDetails {
    /// The collection ID.
    pub id: Option<String>,

    /// The instance item.
    pub item: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse500 {
    pub error: Option<PostCollectionsForkCollectionIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsForkCollectionIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeBody {
    /// The destination collection's unique ID.
    pub destination: String,

    /// The forked collection's unique ID.
    pub source: String,

    /// The fork's merge strategy:
    ///
    /// - `deleteSource` — The system deletes the forked collection after a successful merge into
    /// the destination collection.
    /// - `updateSourceWithDestination` — The system only merges the forked collection into the
    /// destination collection.
    pub strategy: Option<Strategy>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Strategy {
    #[serde(rename = "deleteSource")]
    DeleteSource,

    #[serde(rename = "updateSourceWithDestination")]
    UpdateSourceWithDestination,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse {
    pub collection: Option<PostCollectionsMergeResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponseCollection {
    /// The source collection's ID.
    pub id: Option<String>,

    /// The source collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse401 {
    pub error: Option<PostCollectionsMergeResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse404 {
    pub error: Option<PostCollectionsMergeResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse404Error {
    /// Information about the error.
    pub details: Option<PostCollectionsMergeResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse404ErrorDetails {
    /// The collection ID.
    pub id: Option<String>,

    /// The instance item.
    pub item: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse500 {
    pub error: Option<PostCollectionsMergeResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsMergeResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse {
    pub collection: Option<PostCollectionsResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponseCollection {
    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse400 {
    pub error: Option<PostCollectionsResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse400Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse401 {
    pub error: Option<PostCollectionsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse500 {
    pub error: Option<PostCollectionsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostCollectionsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDetectedSecretsQueriesBody {
    /// If true, return secrets with a `resolved` status.
    pub resolved: Option<bool>,

    /// A list of secrets types to query. For a list of valid IDs, use the GET `/secret-types`
    /// endpoint.
    pub secret_types: Option<Vec<String>>,

    /// A list of the types of resolution statuses to query.
    pub statuses: Option<Vec<Resolution>>,

    /// A list of workspaces IDs to query.
    pub workspace_ids: Option<Vec<String>>,

    /// A list of workspace [visibility
    /// settings](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility)
    /// to query. This currently supports the `team` and `public` settings.
    pub workspace_visiblities: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Resolution {
    #[serde(rename = "ACCEPTED_RISK")]
    AcceptedRisk,

    #[serde(rename = "FALSE_POSITIVE")]
    FalsePositive,

    Revoked,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostDetectedSecretsQueriesResponse {
    pub data: Option<Vec<PostDetectedSecretsQueriesResponseDataItem>>,

    /// The response's meta information for paginated results.
    pub meta: Option<PostDetectedSecretsQueriesResponseMeta>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDetectedSecretsQueriesResponseDataItem {
    /// The date and time at which the secret was first detected.
    pub detected_at: Option<String>,

    /// The secret's obfuscated value.
    pub obfuscated_secret: Option<String>,

    /// The number of times the secret was found in the workspace.
    pub occurrences: Option<f64>,

    /// The secret's current status:
    /// - `ACTIVE` — The secret is active.
    /// - `FALSE_POSITIVE` — The discovered secret is not an actual secret.
    /// - `REVOKED` — The secret is valid, but the user rotated their key to resolve the issue.
    /// - `ACCEPTED_RISK` — The Secret Scanner found the secret, but user accepts the risk of
    /// publishing it.
    pub resolution: Option<ResolutionEnum>,

    /// The SHA-256 hash of the detected secret.
    pub secret_hash: Option<String>,

    /// The detected secret's ID.
    pub secret_id: Option<String>,

    /// The type of the secret.
    pub secret_type: Option<String>,

    /// The ID of the workspace that contains the secret.
    pub workspace_id: Option<String>,

    /// The workspace's [visibility
    /// setting](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility).
    pub workspace_visibility: Option<WorkspaceVisibility>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceVisibility {
    Personal,

    Private,

    Public,

    Team,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostDetectedSecretsQueriesResponseMeta {
    /// The maximum number of records in the paginated response.
    pub limit: Option<f64>,

    /// The Base64-encoded value that points to the next record in the results set.
    pub next_cursor: Option<String>,

    /// The number of records that match the defined criteria. This will only be present if the
    /// `include` query parameter is specified with the `meta.total` value.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostDetectedSecretsQueriesResponse400 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_detected_secrets_queries_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostDetectedSecretsQueriesResponse401 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_detected_secrets_queries_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostDetectedSecretsQueriesResponse403 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_detected_secrets_queries_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostDetectedSecretsQueriesResponse500 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub post_detected_secrets_queries_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsBody {
    pub environment: Option<PostEnvironmentsBodyEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsBodyEnvironment {
    /// The environment's name.
    pub name: String,

    /// Information about the environment's variables.
    pub values: Option<Vec<Vec<PostEnvironmentsBodyEnvironmentValuesItemItem>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsBodyEnvironmentValuesItemItem {
    /// If true, the variable is enabled.
    pub enabled: Option<bool>,

    /// The variable's name.
    pub key: Option<String>,

    /// The variable type.
    #[serde(rename = "type")]
    pub post_environments_body_environment_values_item_item_type: Option<PurpleType>,

    /// The variable's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse {
    pub environment: Option<PostEnvironmentsResponseEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponseEnvironment {
    /// The environment's ID.
    pub id: Option<String>,

    /// The environment's name.
    pub name: Option<String>,

    /// The environment's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse400 {
    pub error: Option<PostEnvironmentsResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse400Error {
    pub details: Option<Vec<String>>,

    pub message: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse401 {
    pub error: Option<PostEnvironmentsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse403 {
    pub error: Option<PostEnvironmentsResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse500 {
    pub error: Option<PostEnvironmentsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostEnvironmentsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse {
    pub collections: Option<Vec<PostImportOpenapiResponseCollectionsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponseCollectionsItem {
    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse400 {
    pub error: Option<PostImportOpenapiResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse400Error {
    /// Information about the error.
    pub details: Option<PostImportOpenapiResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse400ErrorDetails {
    /// The parameter name.
    pub param: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse401 {
    pub error: Option<PostImportOpenapiResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse500 {
    pub error: Option<PostImportOpenapiResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostImportOpenapiResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksBody {
    pub mock: Option<PostMocksBodyMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksBodyMock {
    /// The mock's associated collection unique ID.
    pub collection: String,

    /// The mock server's associated environment ID.
    pub environment: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// If true, the mock server is set private. By default, mock servers are public and can
    /// receive requests from anyone and anywhere.
    pub private: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse {
    pub mock: Option<PostMocksMockIdPublishResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponseMock {
    /// The mock server's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse400 {
    pub error: Option<PostMocksMockIdPublishResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse401 {
    pub error: Option<PostMocksMockIdPublishResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse404 {
    pub error: Option<PostMocksMockIdPublishResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse500 {
    pub error: Option<PostMocksMockIdPublishResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdPublishResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMocksMockIdServerResponsesBody {
    pub server_response: Option<PostMocksMockIdServerResponsesBodyServerResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMocksMockIdServerResponsesBodyServerResponse {
    /// The server response's body that returns when calling the mock server.
    pub body: Option<String>,

    /// The server response's request headers, such as Content-Type, Accept, encoding, and other
    /// information.
    pub headers: Option<Vec<PostMocksMockIdServerResponsesBodyServerResponseHeadersItem>>,

    /// The server response's body language type.
    pub language: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponseLanguage>,

    /// The server response's name.
    pub name: String,

    /// The server response's 5xx HTTP response code. This property **only** accepts 5xx values.
    pub status_code: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesBodyServerResponseHeadersItem {
    /// The request header's key value.
    pub key: Option<String>,

    /// The request header's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse400 {
    pub error: Option<PostMocksMockIdServerResponsesResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse400Error {
    /// Information about the error.
    pub details: Option<PostMocksMockIdServerResponsesResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse400ErrorDetails {
    /// Information about the missing parameter.
    pub param: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse401 {
    pub error: Option<PostMocksMockIdServerResponsesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse500 {
    pub error: Option<PostMocksMockIdServerResponsesResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksMockIdServerResponsesResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMocksMockIdServerResponsesResponseItem {
    /// The date and time at which the server response was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the server response.
    pub created_by: Option<String>,

    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code.
    pub status_code: Option<f64>,

    /// The date and time at which the server response was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who last updated the server response.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse {
    pub mock: Option<PostMocksResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMocksResponseMock {
    /// The mock's associated collection unique ID.
    pub collection: Option<String>,

    /// Information about the mock server's configuration.
    pub config: Option<PostMocksResponseMockConfig>,

    /// The date and time at which the mock server was created.
    pub created_at: Option<String>,

    /// The mock's associated environment unique ID.
    pub environment: Option<String>,

    /// The mock server's ID.
    pub id: Option<String>,

    /// The mock server URL.
    pub mock_url: Option<String>,

    /// The ID of mock server's owner.
    pub owner: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the mock server was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMocksResponseMockConfig {
    /// Information about the mock server's simulated network delay settings. This returns a null
    /// value if there are no configured network delay settings.
    pub delay: Option<PostMocksResponseMockConfigDelay>,

    /// A list of the mock server's headers.
    pub headers: Option<Vec<String>>,

    /// If true, match the request body.
    pub match_body: Option<bool>,

    /// If true, match query parameters.
    pub match_query_params: Option<bool>,

    /// If true, use wildcard variable matching.
    pub match_wildcards: Option<bool>,

    /// The ID of mock server's default response for requests. All calls to the mock server will
    /// return the defined response.
    pub server_response_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponseMockConfigDelay {
    /// The configured delay, in milliseconds.
    pub duration: Option<i64>,

    /// The simulated fixed network delay value:
    ///
    /// - `1` — 2G (300 ms).
    /// - `2` — 3G (100 ms).
    ///
    /// The object does not return this value for custom delay values.
    pub preset: Option<Preset>,

    /// The type of simulated delay value:
    ///
    /// - `fixed` — The delay value is a fixed value.
    #[serde(rename = "type")]
    pub post_mocks_response_mock_config_delay_type: Option<GetMocksResponseMocksItemConfigDelayType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse400 {
    pub error: Option<PostMocksResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse400Error {
    /// Information about the error.
    pub details: Option<PostMocksResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse400ErrorDetails {
    /// Information about the missing parameter.
    pub param: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse401 {
    pub error: Option<PostMocksResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse500 {
    pub error: Option<PostMocksResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMocksResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsBody {
    pub monitor: Option<PostMonitorsBodyMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsBodyMonitor {
    /// The monitor's associated collection unique ID.
    pub collection: Option<String>,

    /// The monitor's associated environment unique ID.
    pub environment: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// Information about the monitor's schedule.
    pub schedule: Option<PostMonitorsBodyMonitorSchedule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsBodyMonitorSchedule {
    /// The monitor's run frequency, based on the given cron pattern.
    /// At this time you can only create monitors with limited schedules. For information about
    /// the available schedules, see our [Postman Monitors](https://monitor.getpostman.com)
    /// collection.
    pub cron: Option<String>,

    /// The monitor's [timezone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse {
    /// Information about the monitor run.
    pub run: Option<PostMonitorsMonitorIdRunResponseRun>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRun {
    /// Information about the monitor run's executions.
    pub executions: Option<Vec<PostMonitorsMonitorIdRunResponseRunExecutionsItem>>,

    /// If the monitor run failed, information about the run's failures.
    pub failures: Option<Vec<Option<serde_json::Value>>>,

    /// Information about the monitor.
    pub info: Option<PostMonitorsMonitorIdRunResponseRunInfo>,

    /// Information about the monitor run's stats.
    pub stats: Option<PostMonitorsMonitorIdRunResponseRunStats>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunExecutionsItem {
    /// The execution ID.
    pub id: Option<f64>,

    /// Information about the executed item.
    pub item: Option<PostMonitorsMonitorIdRunResponseRunExecutionsItemItem>,

    /// Information about the monitor run's requests.
    pub request: Option<PostMonitorsMonitorIdRunResponseRunExecutionsItemRequest>,

    /// Information about the monitor run's response.
    pub response: Option<PostMonitorsMonitorIdRunResponseRunExecutionsItemResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunExecutionsItemItem {
    /// The executed item's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunExecutionsItemRequest {
    pub body: Option<serde_json::Value>,

    pub headers: Option<serde_json::Value>,

    /// The request method.
    pub method: Option<String>,

    /// The date and time of the request.
    pub timestamp: Option<String>,

    /// The request's URL.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMonitorsMonitorIdRunResponseRunExecutionsItemResponse {
    pub body: Option<serde_json::Value>,

    /// The response's HTTP status code.
    pub code: Option<f64>,

    pub headers: Option<serde_json::Value>,

    /// The response size, in bytes.
    pub response_size: Option<f64>,

    /// The response time, in milliseconds.
    pub response_time: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostMonitorsMonitorIdRunResponseRunInfo {
    /// The monitor's associated collection unique ID.
    pub collection_uid: Option<String>,

    /// The monitor's associated environment unique ID.
    pub environment_uid: Option<String>,

    /// The date and time at which the monitor's run completed.
    pub finished_at: Option<String>,

    /// The monitor's run job ID.
    pub job_id: Option<String>,

    /// The monitor's ID.
    pub monitor_id: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// The date and time at which the monitor run began.
    pub started_at: Option<String>,

    /// The monitor run's status.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunStats {
    /// The monitor run's assertions stats.
    pub assertions: Option<PostMonitorsMonitorIdRunResponseRunStatsAssertions>,

    /// The monitor run's request stats.
    pub requests: Option<PostMonitorsMonitorIdRunResponseRunStatsRequests>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunStatsAssertions {
    /// The total number of test failures.
    pub failed: Option<f64>,

    /// The total number of tests performed.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponseRunStatsRequests {
    /// The number of request failures.
    pub failed: Option<f64>,

    /// The total number of requests.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse401 {
    pub error: Option<PostMonitorsMonitorIdRunResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse500 {
    pub error: Option<PostMonitorsMonitorIdRunResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsMonitorIdRunResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse {
    pub monitor: Option<PostMonitorsResponseMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponseMonitor {
    /// The monitor's ID.
    pub id: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse400 {
    pub error: Option<PostMonitorsResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse400Error {
    pub details: Option<serde_json::Value>,

    pub message: Option<String>,

    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse401 {
    pub error: Option<PostMonitorsResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse403 {
    pub error: Option<PostMonitorsResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse403Error {
    /// The error description.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse500 {
    pub error: Option<PostMonitorsResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMonitorsResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse401 {
    pub error: Option<PostNetworkPrivateResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse403 {
    pub error: Option<PostNetworkPrivateResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse404 {
    pub error: Option<PostNetworkPrivateResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse500 {
    pub error: Option<PostNetworkPrivateResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostNetworkPrivateResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsBody {
    /// If true, activates the user. This lets them authenticate in to your Postman team.
    pub active: Option<bool>,

    /// The user's external ID.
    pub external_id: Option<String>,

    /// A list of groups to which to assign the user to.
    pub groups: Option<Vec<String>>,

    /// The user's IETF locale.
    pub locale: Option<String>,

    /// Information about the user's name.
    pub name: Option<PostScimV2GroupsBodyName>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,

    /// The user's username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsBodyName {
    /// The user's last name.
    pub family_name: Option<String>,

    /// The user's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsResponse {
    /// If true, the user is active.
    pub active: Option<bool>,

    /// The user's external ID.
    pub external_id: Option<String>,

    /// The user's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<PostScimV2GroupsResponseMeta>,

    pub name: Option<PostScimV2GroupsResponseName>,

    /// The SCIM schema URI.
    pub schemas: Option<Vec<String>>,

    /// The user's username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsResponseMeta {
    /// The date and time at which the user was created.
    pub created: Option<String>,

    /// The date and time at which the user was last modified.
    pub last_modified: Option<String>,

    /// The SCIM resource type.
    pub resource_type: Option<MetaResourceType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MetaResourceType {
    User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsResponseName {
    /// The user's last name.
    pub family_name: Option<String>,

    /// The user's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// A list of SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2GroupsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2GroupsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2GroupsResponse409 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2GroupsResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2GroupsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersBody {
    /// If true, activates the user. This lets them authenticate in to your Postman team.
    pub active: Option<bool>,

    /// The user's external ID.
    pub external_id: Option<String>,

    /// A list of groups to which to assign the user to.
    pub groups: Option<Vec<String>>,

    /// The user's IETF locale.
    pub locale: Option<String>,

    /// Information about the user's name.
    pub name: Option<PostScimV2UsersBodyName>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,

    /// The user's username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersBodyName {
    /// The user's last name.
    pub family_name: Option<String>,

    /// The user's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersResponse {
    /// If true, the user is active.
    pub active: Option<bool>,

    /// The user's external ID.
    pub external_id: Option<String>,

    /// The user's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<PostScimV2UsersResponseMeta>,

    pub name: Option<PostScimV2UsersResponseName>,

    /// The SCIM schema URI.
    pub schemas: Option<Vec<String>>,

    /// The user's username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersResponseMeta {
    /// The date and time at which the user was created.
    pub created: Option<String>,

    /// The date and time at which the user was last modified.
    pub last_modified: Option<String>,

    /// The SCIM resource type.
    pub resource_type: Option<MetaResourceType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersResponseName {
    /// The user's last name.
    pub family_name: Option<String>,

    /// The user's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// A list of SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2UsersResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2UsersResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostScimV2UsersResponse409 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2UsersResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostScimV2UsersResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationBody {
    pub schema: Option<PostSecurityApiValidationBodySchema>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationBodySchema {
    /// The definition format.
    pub language: SchemaLanguage,

    /// The stringified API definition.
    pub schema: String,

    /// The definition type.
    #[serde(rename = "type")]
    pub post_security_api_validation_body_schema_type: FluffyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaLanguage {
    Json,

    Yaml,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FluffyType {
    Openapi2,

    Openapi3,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse {
    /// Information about each issue discovered in the analysis. Each object includes the
    /// violation's severity and category, the location of the issue, data paths, and other
    /// information. This returns an empty object if there are no issues present in the schema.
    ///
    /// If there are issues, this returns the `possibleFixUrl` response in each warning object.
    /// This provides a link to documentation you can use to resolve the warning.
    pub warnings: Option<Vec<Option<serde_json::Value>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse400 {
    pub error: Option<PostSecurityApiValidationResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse401 {
    pub error: Option<PostSecurityApiValidationResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse500 {
    pub error: Option<PostSecurityApiValidationResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostSecurityApiValidationResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksBody {
    pub webhook: Option<PostWebhooksBodyWebhook>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksBodyWebhook {
    /// The unique ID of the collection to trigger when calling this webhook.
    pub collection: Option<String>,

    /// The webhook's name. On success, the system creates a new monitor with this name in the
    /// **Monitors** tab.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse {
    /// Information about the webhook.
    pub webhook: Option<PostWebhooksResponseWebhook>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostWebhooksResponseWebhook {
    /// The unique ID of the webhook's associated collection.
    pub collection: Option<String>,

    /// The webhook's ID.
    pub id: Option<String>,

    /// The webhook's name.
    pub name: Option<String>,

    /// The webhook's unique ID.
    pub uid: Option<String>,

    /// The webhook's URL.
    pub webhook_url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse401 {
    pub error: Option<PostWebhooksResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse500 {
    pub error: Option<PostWebhooksResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWebhooksResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesBody {
    /// Information about the workspace.
    pub workspace: Option<PostWorkspacesBodyWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesBodyWorkspace {
    /// The workspace's description.
    pub description: Option<String>,

    /// The workspace's name.
    pub name: String,

    /// The type of workspace:
    ///
    /// - `personal`
    /// - `private` — Private workspaces are available on Postman [Professional and Enterprise
    /// plans](https://www.postman.com/pricing).
    /// - `public`
    /// - `team`
    /// - `partner` — [Partner
    /// Workspaces](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/partner-workspaces/)
    /// are available on Postman [Enterprise Ultimate plans](https://www.postman.com/pricing).
    #[serde(rename = "type")]
    pub post_workspaces_body_workspace_type: VisibilityEnum,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse {
    /// Information about the created workspace.
    pub workspace: Option<PostWorkspacesResponseWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponseWorkspace {
    /// The workspace's ID.
    pub id: Option<String>,

    /// The workspace's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse400 {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse401 {
    pub error: Option<PostWorkspacesResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse404 {
    pub error: Option<PostWorkspacesResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse500 {
    pub error: Option<PostWorkspacesResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostWorkspacesResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdBody {
    /// The API's description. This supports Markdown formatting.
    pub description: Option<String>,

    /// The API's name.
    pub name: String,

    /// The API's short summary.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse {
    /// The created task ID. You can use this ID to track the status of syncing an API collection
    /// with an API schema.
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse400 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_collections_collection_id_sync_with_schema_tasks_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_collections_collection_id_sync_with_schema_tasks_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_collections_collection_id_sync_with_schema_tasks_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_collections_collection_id_sync_with_schema_tasks_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_collections_collection_id_sync_with_schema_tasks_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutApisApiIdResponse {
    /// The date and time at which the API was created.
    pub created_at: Option<String>,

    /// The user ID of the user that created the API.
    pub created_by: Option<String>,

    /// The API's description. This supports Markdown formatting.
    pub description: Option<String>,

    /// The API's ID.
    pub id: Option<String>,

    /// The API's name.
    pub name: String,

    /// The API's summary.
    pub summary: Option<String>,

    /// The date and time at which the API was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that updated the API.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathBody {
    /// The schema file's content.
    pub content: String,

    /// The schema file's name.
    pub name: Option<String>,

    /// Information about the schema's root file. This tag only applies to protobuf
    /// specifications.
    pub root: Option<PutApisApiIdSchemasSchemaIdFilesFilePathBodyRoot>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathBodyRoot {
    /// If true, tag the file as the root file.
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse {
    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<String>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse400 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse422 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response422_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdSchemasSchemaIdFilesFilePathResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_schemas_schema_id_files_file_path_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsBody {
    /// A list of the associated tags as slugs.
    pub tags: Vec<PutApisApiIdTagsBodyTagsItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsBodyTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<PutApisApiIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_apis_api_id_tags_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_apis_api_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_apis_api_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_apis_api_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutApisApiIdVersionsVersionIdBody {
    /// The version's name.
    pub name: String,

    /// The version's Markdown-supported release notes.
    pub release_notes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutApisApiIdVersionsVersionIdResponse {
    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// The version's release notes.
    pub release_notes: Option<String>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdVersionsVersionIdResponse401 {
    /// Details about the error.
    pub detail: Option<String>,

    /// The error instance.
    pub instance: Option<String>,

    /// The HTTP status code.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The error type.
    #[serde(rename = "type")]
    pub put_apis_api_id_versions_version_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdVersionsVersionIdResponse403 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_versions_version_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdVersionsVersionIdResponse404 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_versions_version_id_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutApisApiIdVersionsVersionIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_apis_api_id_versions_version_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdBody {
    /// For a complete list of values, refer to the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub collection: Option<PutCollectionsCollectionIdBodyCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdBodyCollection {
    /// An object that contains basic information about the collection. For a complete list of
    /// values, refer to the `definitions.info` entry in the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub info: PutCollectionsCollectionIdBodyCollectionInfo,

    /// Information about the collection's contents, such as folders, requests, and responses.
    /// For a complete list of values, refer to the `/tmp/.tmpopFoDz/item.json-group` entry in
    /// the [collection.json schema
    /// file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
    pub item: Vec<PutCollectionsCollectionIdBodyCollectionItemItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdBodyCollectionInfo {
    /// The collection's description.
    pub description: Option<String>,

    /// The collection's name.
    pub name: String,

    /// A URL to the collection's schema.
    pub schema: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdBodyCollectionItemItem {
    /// The collection item's ID.
    pub id: Option<String>,

    /// The collection item's name.
    pub name: Option<String>,

    /// The collection item's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdBody {
    /// The folder's description.
    pub description: Option<String>,

    /// The folder's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse {
    /// The folder's updated information, including the updated properties. For a complete list
    /// of properties, refer to the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PutCollectionsCollectionIdFoldersFolderIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponseData {
    /// The collection ID that the folder belongs to.
    pub collection: Option<String>,

    /// The folder's creation date and time.
    pub created_at: Option<String>,

    /// The folder's description.
    pub description: Option<String>,

    /// Information about the folder.
    pub folder: Option<String>,

    /// The folder's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<f64>,

    /// The user ID of the user that last updated the folder.
    pub last_updated_by: Option<String>,

    /// The folder's name.
    pub name: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,

    /// The date and time at which the folder was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse400 {
    pub error: Option<PutCollectionsCollectionIdFoldersFolderIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse400Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdFoldersFolderIdResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse401 {
    pub error: Option<PutCollectionsCollectionIdFoldersFolderIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse401Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse404 {
    pub error: Option<PutCollectionsCollectionIdFoldersFolderIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse404Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The folder's ID.
    pub model_id: Option<String>,

    /// The user ID of the folder's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdFoldersFolderIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_collections_collection_id_folders_folder_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdBody {
    /// The request's method.
    pub method: Option<Method>,

    /// The request's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Method {
    #[serde(rename = "COPY")]
    Copy,

    #[serde(rename = "DELETE")]
    Delete,

    #[serde(rename = "GET")]
    Get,

    #[serde(rename = "HEAD")]
    Head,

    #[serde(rename = "LINK")]
    Link,

    #[serde(rename = "LOCK")]
    Lock,

    #[serde(rename = "OPTIONS")]
    Options,

    #[serde(rename = "PATCH")]
    Patch,

    #[serde(rename = "POST")]
    Post,

    #[serde(rename = "PROPFIND")]
    Propfind,

    #[serde(rename = "PURGE")]
    Purge,

    #[serde(rename = "PUT")]
    Put,

    #[serde(rename = "UNLINK")]
    Unlink,

    #[serde(rename = "UNLOCK")]
    Unlock,

    #[serde(rename = "VIEW")]
    View,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse {
    /// Information about the updated request. For a complete list of properties, refer to the
    /// `definitions.request` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PutCollectionsCollectionIdRequestsRequestIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub revision: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponseData {
    /// The request's creation date and time.
    pub created_at: Option<String>,

    /// The request's description.
    pub description: Option<String>,

    /// The request's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<f64>,

    /// The user ID of the user that last updated the request.
    pub last_updated_by: Option<String>,

    /// The request's name.
    pub name: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,

    /// The date and time at which the request was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse400 {
    pub error: Option<PutCollectionsCollectionIdRequestsRequestIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse400Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdRequestsRequestIdResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse401 {
    pub error: Option<PutCollectionsCollectionIdRequestsRequestIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse401Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails>,

    /// The error's message.
    pub message: Option<String>,

    /// The error's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse404 {
    pub error: Option<PutCollectionsCollectionIdRequestsRequestIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse404Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The request's ID.
    pub model_id: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdRequestsRequestIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_collections_collection_id_requests_request_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse {
    pub collection: Option<PutCollectionsCollectionIdResponseCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponseCollection {
    /// The collection's ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,

    /// The collection's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse400 {
    pub error: Option<PutCollectionsCollectionIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse400Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse401 {
    pub error: Option<PutCollectionsCollectionIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse403 {
    pub error: Option<PutCollectionsCollectionIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse404 {
    pub error: Option<PutCollectionsCollectionIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse404Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse404ErrorDetails {
    /// The collection ID.
    pub id: Option<String>,

    /// The instance item.
    pub item: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse500 {
    pub error: Option<PutCollectionsCollectionIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCollectionsCollectionIdResponsesResponseIdBody {
    /// The response's name.
    pub name: Option<String>,

    /// The response's HTTP response code information.
    pub response_code: Option<PutCollectionsCollectionIdResponsesResponseIdBodyResponseCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdBodyResponseCode {
    /// The response's HTTP response status code.
    pub code: Option<f64>,

    /// The name of the status code.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse {
    /// Information about the updated response. For a complete list of request properties, refer
    /// to the `definitions.request` property in the [collection.json schema
    /// file](https://schema.postman.com/collection/json/v1.0.0/draft-07/collection.json).
    pub data: Option<PutCollectionsCollectionIdResponsesResponseIdResponseData>,

    pub meta: Option<serde_json::Value>,

    /// The request's ID.
    pub model_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponseData {
    /// The request's creation date and time.
    pub created_at: Option<String>,

    /// The request's ID.
    pub id: Option<String>,

    /// An internal revision ID. Its value increments each time the resource changes. You can use
    /// this ID to track whether there were changes since the last time you fetched the resource.
    pub last_revision: Option<i64>,

    /// The user ID of the user that last updated the request.
    pub last_updated_by: Option<String>,

    /// The request's name.
    pub name: Option<String>,

    /// The user ID of the request's owner.
    pub owner: Option<String>,

    /// The date and time at which the request was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse400 {
    pub error: Option<PutCollectionsCollectionIdResponsesResponseIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse400Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdResponsesResponseIdResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse400ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse401 {
    pub error: Option<PutCollectionsCollectionIdResponsesResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse401Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse401ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse404 {
    pub error: Option<PutCollectionsCollectionIdResponsesResponseIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse404Error {
    /// Information about the error.
    pub details: Option<PutCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse404ErrorDetails {
    /// The resource name.
    pub model: Option<String>,

    /// The response's ID.
    pub model_id: Option<String>,

    /// The user ID of the response's owner.
    pub owner: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdResponsesResponseIdResponse500 {
    /// Details about the error message.
    pub detail: Option<String>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_collections_collection_id_responses_response_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsBody {
    /// A list of the associated tags as slugs.
    pub tags: Vec<PutCollectionsCollectionIdTagsBodyTagsItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsBodyTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<PutCollectionsCollectionIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_collections_collection_id_tags_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_collections_collection_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_collections_collection_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_collections_collection_id_tags_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutCollectionsCollectionIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_collections_collection_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDetectedSecretsSecretIdBody {
    /// The secret's updated resolution status:
    /// - `FALSE_POSITIVE` — The discovered secret is not an actual secret.
    /// - `REVOKED` — The secret is valid, but the user rotated their key to resolve the issue.
    /// - `ACCEPTED_RISK` — The Secret Scanner found the secret, but user accepts the risk of
    /// publishing it.
    pub resolution: Resolution,

    /// The ID of the workspace that contains the secret.
    pub workspace_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDetectedSecretsSecretIdResponse {
    /// The history of the secret's resolution status changes.
    pub history: Option<Vec<PutDetectedSecretsSecretIdResponseHistoryItem>>,

    /// The secret's current resolution status:
    /// - `ACTIVE` — The secret is active.
    /// - `FALSE_POSITIVE` — The discovered secret is not an actual secret.
    /// - `REVOKED` — The secret is valid, but the user rotated their key to resolve the issue.
    /// - `ACCEPTED_RISK` — The Secret Scanner found the secret, but user accepts the risk of
    /// publishing it.
    pub resolution: Option<ResolutionEnum>,

    /// The SHA-256 hash of the detected secret.
    pub secret_hash: Option<String>,

    /// The ID of the workspace that contains the secret.
    pub workspace_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutDetectedSecretsSecretIdResponseHistoryItem {
    /// The ID of the user that updated the secret's resolution status.
    pub actor: Option<f64>,

    /// The date and time at which the resolution status was updated.
    pub created_at: Option<String>,

    /// The secret's updated resolution status:
    /// - `ACTIVE` — The secret is active.
    /// - `FALSE_POSITIVE` — The discovered secret is not an actual secret.
    /// - `REVOKED` — The secret is valid, but the user rotated their key to resolve the issue.
    /// - `ACCEPTED_RISK` — The Secret Scanner found the secret, but user accepts the risk of
    /// publishing it.
    pub resolution: Option<ResolutionEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutDetectedSecretsSecretIdResponse401 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_detected_secrets_secret_id_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutDetectedSecretsSecretIdResponse403 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_detected_secrets_secret_id_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutDetectedSecretsSecretIdResponse500 {
    /// The instance identifying the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The HTTP status code generated by the origin server.
    pub status: Option<f64>,

    /// The generic description for the error's class.
    pub title: Option<String>,

    /// The type of error.
    #[serde(rename = "type")]
    pub put_detected_secrets_secret_id_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdBody {
    pub environment: Option<PutEnvironmentsEnvironmentIdBodyEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdBodyEnvironment {
    /// The environment's name.
    pub name: String,

    /// Information about the environment's variables.
    pub values: Option<Vec<Vec<PutEnvironmentsEnvironmentIdBodyEnvironmentValuesItemItem>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdBodyEnvironmentValuesItemItem {
    /// If true, the variable is enabled.
    pub enabled: Option<bool>,

    /// The variable's name.
    pub key: Option<String>,

    /// The variable type.
    #[serde(rename = "type")]
    pub put_environments_environment_id_body_environment_values_item_item_type: Option<PurpleType>,

    /// The variable's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse {
    pub environment: Option<PutEnvironmentsEnvironmentIdResponseEnvironment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponseEnvironment {
    pub id: Option<String>,

    pub name: Option<String>,

    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse400 {
    pub error: Option<PutEnvironmentsEnvironmentIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse400Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse401 {
    pub error: Option<PutEnvironmentsEnvironmentIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse500 {
    pub error: Option<PutEnvironmentsEnvironmentIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutEnvironmentsEnvironmentIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdBody {
    pub mock: Option<PutMocksMockIdBodyMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdBodyMock {
    /// The mock server's configuration settings.
    pub config: Option<PutMocksMockIdBodyMockConfig>,

    /// The mock server's description.
    pub description: Option<String>,

    /// The associated environment's unique ID.
    pub environment: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// If true, the mock server is set private. By default, mock servers are public and can
    /// receive requests from anyone and anywhere.
    pub private: Option<bool>,

    /// The API's version tag ID.
    pub version_tag: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdBodyMockConfig {
    /// The server response ID. This sets the given server response as the default response for
    /// each request.
    ///
    /// To deactivate a server response, pass a null value.
    pub server_response_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse {
    pub mock: Option<PutMocksMockIdResponseMock>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdResponseMock {
    /// The mock's associated collection unique ID.
    pub collection: Option<String>,

    /// Information about the mock server's configuration.
    pub config: Option<PutMocksMockIdResponseMockConfig>,

    /// The date and time at which the mock server was created.
    pub created_at: Option<String>,

    /// The mock server's associated environment ID.
    pub environment: Option<String>,

    /// The mock server's ID.
    pub id: Option<String>,

    /// The mock server URL.
    pub mock_url: Option<String>,

    /// The mock server's name.
    pub name: Option<String>,

    /// The ID of mock server's owner.
    pub owner: Option<String>,

    /// The mock server's unique ID.
    pub uid: Option<String>,

    /// The date and time at which the mock server was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdResponseMockConfig {
    /// A list of the mock server's headers.
    pub headers: Option<Vec<Option<serde_json::Value>>>,

    /// If true, match the request body.
    pub match_body: Option<bool>,

    /// If true, match query parameters.
    pub match_query_params: Option<bool>,

    /// If true, use wildcard variable matching.
    pub match_wildcards: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse401 {
    pub error: Option<PutMocksMockIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse404 {
    pub error: Option<PutMocksMockIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse404Error {
    /// Information about the error.
    pub details: Option<Vec<String>>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse500 {
    pub error: Option<PutMocksMockIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdServerResponsesServerResponseIdBody {
    pub server_response: Option<PutMocksMockIdServerResponsesServerResponseIdBodyServerResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdServerResponsesServerResponseIdBodyServerResponse {
    /// The server response's body that returns when calling the mock server.
    pub body: Option<String>,

    /// The server response's request headers, such as Content-Type, Accept, encoding, and other
    /// information.
    pub headers: Option<Vec<PutMocksMockIdServerResponsesServerResponseIdBodyServerResponseHeadersItem>>,

    /// The server response's body language type.
    pub language: Option<DeleteMocksMockIdServerResponsesServerResponseIdResponseLanguage>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code. This property **only** accepts 5xx values.
    pub status_code: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdBodyServerResponseHeadersItem {
    /// The request header's key value.
    pub key: Option<String>,

    /// The request header's value.
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse400 {
    pub error: Option<PutMocksMockIdServerResponsesServerResponseIdResponse400Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse400Error {
    /// Information about the error.
    pub details: Option<PutMocksMockIdServerResponsesServerResponseIdResponse400ErrorDetails>,

    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse400ErrorDetails {
    /// Information about the missing parameter.
    pub param: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse401 {
    pub error: Option<PutMocksMockIdServerResponsesServerResponseIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse500 {
    pub error: Option<PutMocksMockIdServerResponsesServerResponseIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutMocksMockIdServerResponsesServerResponseIdResponseItem {
    /// The date and time at which the server response was created.
    pub created_at: Option<String>,

    /// The user ID of the user who created the server response.
    pub created_by: Option<String>,

    /// The server response's ID.
    pub id: Option<String>,

    /// The server response's name.
    pub name: Option<String>,

    /// The server response's 5xx HTTP response code.
    pub status_code: Option<f64>,

    /// The date and time at which the server response was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user who last updated the server response.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdBody {
    pub monitor: Option<PutMonitorsMonitorIdBodyMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdBodyMonitor {
    /// The monitor's name.
    pub name: Option<String>,

    /// Information about the monitor's schedule.
    pub schedule: Option<PutMonitorsMonitorIdBodyMonitorSchedule>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdBodyMonitorSchedule {
    pub cron: Option<serde_json::Value>,

    /// The monitor's [timezone](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse {
    pub monitor: Option<PutMonitorsMonitorIdResponseMonitor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponseMonitor {
    /// The monitor's ID.
    pub id: Option<String>,

    /// The monitor's name.
    pub name: Option<String>,

    /// The monitor's unique ID.
    pub uid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse401 {
    pub error: Option<PutMonitorsMonitorIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse404 {
    pub error: Option<PutMonitorsMonitorIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse500 {
    pub error: Option<PutMonitorsMonitorIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutMonitorsMonitorIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse401 {
    pub error: Option<PutNetworkPrivateElementTypeElementIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse403 {
    pub error: Option<PutNetworkPrivateElementTypeElementIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse404 {
    pub error: Option<PutNetworkPrivateElementTypeElementIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse500 {
    pub error: Option<PutNetworkPrivateElementTypeElementIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateElementTypeElementIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdBody {
    /// The response to the user's request.
    pub response: Option<PutNetworkPrivateNetworkEntityRequestRequestIdBodyResponse>,

    /// The request's status.
    pub status: PutNetworkPrivateNetworkEntityRequestRequestIdBodyStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdBodyResponse {
    /// A message that details why the user's request was denied.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PutNetworkPrivateNetworkEntityRequestRequestIdBodyStatus {
    Approved,

    Denied,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse {
    /// Information about the Private API Network request.
    pub request: Option<Vec<PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItem {
    /// The date and time at which the request was created.
    pub created_at: Option<String>,

    /// The ID of the user who created the request.
    pub created_by: Option<i64>,

    /// Information about the requested element.
    pub element: Option<PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItemElement>,

    /// The request's ID.
    pub id: Option<i64>,

    /// The user's optional message included in the request.
    pub message: Option<String>,

    /// Information about the response to the element's request. This object only returns when
    /// the network manager denied a request with a message.
    pub response: Option<PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItemResponse>,

    /// The request's status.
    pub status: Option<PutNetworkPrivateNetworkEntityRequestRequestIdBodyStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItemElement {
    /// The date and time at which the element was approved and added to the Private API Network.
    pub created_at: Option<String>,

    /// The user ID of the user who requested to add the element to the Private API Network.
    pub created_by: Option<i64>,

    /// The element's ID.
    pub id: Option<String>,

    /// The element's name.
    pub name: Option<String>,

    /// The element's short summary.
    pub summary: Option<String>,

    /// The element type.
    #[serde(rename = "type")]
    pub put_network_private_network_entity_request_request_id_response_request_item_element_type: Option<EntityTypeEnum>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponseRequestItemResponse {
    /// The date and time at which the network manager denied the request.
    pub created_at: Option<String>,

    /// The network manager's user ID.
    pub created_by: Option<i64>,

    /// The network manager's request response message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse400 {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<Name>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Name {
    #[serde(rename = "invalidParamsError")]
    InvalidParamsError,

    #[serde(rename = "requestAlreadyResponded")]
    RequestAlreadyResponded,

    #[serde(rename = "requestEntityAlreadyPublished")]
    RequestEntityAlreadyPublished,

    #[serde(rename = "RequestNotFound")]
    RequestNotFound,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse401 {
    pub error: Option<PutNetworkPrivateNetworkEntityRequestRequestIdResponse401Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse401Error {
    /// The error message.
    pub message: Option<String>,

    /// The error message.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse403 {
    pub error: Option<PutNetworkPrivateNetworkEntityRequestRequestIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse500 {
    pub error: Option<PutNetworkPrivateNetworkEntityRequestRequestIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutNetworkPrivateNetworkEntityRequestRequestIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdBody {
    /// Information about the user's name.
    pub name: Option<PutScimV2UsersUserIdBodyName>,

    /// The SCIM schema resource URI.
    pub schemas: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScimV2UsersUserIdBodyName {
    /// The user's last name.
    pub family_name: Option<String>,

    /// The user's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScimV2UsersUserIdResponse {
    /// If true, the team member is active.
    pub active: Option<bool>,

    /// The team member's external ID.
    pub external_id: Option<String>,

    /// The team member's SCIM ID.
    pub id: Option<String>,

    /// The response's non-standard meta information.
    pub meta: Option<PutScimV2UsersUserIdResponseMeta>,

    /// Information about the Postman team member.
    pub name: Option<PutScimV2UsersUserIdResponseName>,

    /// A list of schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The team member's SCIM username.
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScimV2UsersUserIdResponseMeta {
    /// The date and time at which the team member was created.
    pub created: Option<String>,

    /// The date and time at which the team member was last modified.
    pub last_modified: Option<String>,

    /// The resource type.
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScimV2UsersUserIdResponseName {
    /// The team member's last name.
    pub family_name: Option<String>,

    /// The team member's first name.
    pub given_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PutScimV2UsersUserIdResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// A list of SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The SCIM type.
    pub scim_type: Option<String>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdResponse429 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutScimV2UsersUserIdResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The SCIM schema resource URIs.
    pub schemas: Option<Vec<String>>,

    /// The HTTP status code.
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdBody {
    pub workspace: Option<PutWorkspacesWorkspaceIdBodyWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdBodyWorkspace {
    /// The new workspace description.
    pub description: Option<String>,

    /// The workspace's new name.
    pub name: Option<String>,

    /// The new workspace visibility
    /// [type](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility).
    /// This property does **not** support the following workspace visibility changes:
    /// - `private` to `public`
    /// - `public` to `private`
    /// - `private` to `personal`
    /// - `team` to `personal`
    /// - `public` to `personal` for a team user
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_body_workspace_type: Option<WorkspaceVisibility>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdGlobalVariablesBody {
    /// A list of the workspace's global variables.
    pub values: Option<Vec<GlobalVariable>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdGlobalVariablesResponse {
    /// A list of the workspace's global variables.
    pub values: Option<Vec<Vec<GlobalVariable>>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdGlobalVariablesResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The type of error.
    pub title: Option<String>,

    /// The generic description for the error's class.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_global_variables_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse {
    /// Information about the updated workspace.
    pub workspace: Option<PutWorkspacesWorkspaceIdResponseWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponseWorkspace {
    /// The workspace's ID.
    pub id: Option<String>,

    /// The workspace's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse400 {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse403 {
    pub error: Option<PutWorkspacesWorkspaceIdResponse403Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse403Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse404 {
    pub error: Option<PutWorkspacesWorkspaceIdResponse404Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse404Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse429 {
    /// The error name.
    pub error: Option<String>,

    /// The error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse500 {
    pub error: Option<PutWorkspacesWorkspaceIdResponse500Error>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdResponse500Error {
    /// The error message.
    pub message: Option<String>,

    /// The error name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsBody {
    /// A list of the associated tags as slugs.
    pub tags: Vec<PutWorkspacesWorkspaceIdTagsBodyTagsItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsBodyTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse {
    /// A list of associated tags.
    pub tags: Option<Vec<PutWorkspacesWorkspaceIdTagsResponseTagsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponseTagsItem {
    /// The tag's ID within a team or individual (non-team) user scope.
    pub slug: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse400 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_tags_response400_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse401 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_tags_response401_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse403 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_tags_response403_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse404 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_tags_response404_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PutWorkspacesWorkspaceIdTagsResponse500 {
    /// Information about the error.
    pub detail: Option<String>,

    /// The URI reference that identifies the specific occurrence of the problem.
    pub instance: Option<String>,

    /// The error's HTTP status code.
    pub status: Option<i64>,

    /// A short summary of the problem.
    pub title: Option<String>,

    /// The URI reference ([RFC 3986](https://www.rfc-editor.org/rfc/rfc3986)) that identifies
    /// the type of problem.
    #[serde(rename = "type")]
    pub put_workspaces_workspace_id_tags_response500_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiBase {
    /// The date and time at which the API was created.
    pub created_at: Option<String>,

    /// The Postman ID of the user that created the API.
    pub created_by: Option<f64>,

    /// The API's description.
    pub description: Option<String>,

    /// The API's ID.
    pub id: Option<String>,

    /// The API's name.
    pub name: Option<String>,

    /// The API's short summary.
    pub summary: Option<String>,

    /// The date and time at which the API was updated.
    pub updated_at: Option<String>,

    /// The Postman ID of the user that updated the API.
    pub updated_by: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDetails {
    /// The API's collections.
    pub collections: Option<Vec<ApiDetailsCollectionsItem>>,

    /// Information about the API's Git repository integration.
    pub git_info: Option<ApiDetailsGitInfo>,

    /// The API's schemas.
    pub schemas: Option<Vec<ApiDetailsSchemasItem>>,

    /// The API's versions.
    pub versions: Option<Vec<ApiDetailsVersionsItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDetailsCollectionsItem {
    /// The collection's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiDetailsGitInfo {
    /// The API definition's collection repository folder location.
    pub collection_folder: Option<String>,

    /// The domain at which the Git repository is hosted.
    pub domain: Option<String>,

    /// The organization that owns the repository.
    pub organization: Option<String>,

    /// The repository's name.
    pub repository: Option<String>,

    /// The API definition's repository folder location. This returns an empty string if the
    /// connected repository uses a root file.
    pub schema_folder: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDetailsSchemasItem {
    /// The schema's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiDetailsVersionsItem {
    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiVersionGitLinked {
    /// The branch ID.
    pub branch: Option<String>,

    /// A list of the version's collections.
    pub collections: Option<Vec<CreateApiVersionGitLinkedCollectionsItem>>,

    /// The version's name.
    pub name: Option<String>,

    /// Information about the API version release. For example, changelog notes.
    pub release_notes: Option<String>,

    /// A list of the version's schemas.
    pub schemas: Option<Vec<CreateApiVersionGitLinkedSchemasItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiVersionGitLinkedCollectionsItem {
    /// Path to a collection in the Git repository.
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiVersionGitLinkedSchemasItem {
    /// The path to the root directory where schemas are stored in the Git repository.
    pub directory_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiVersionNonGitLinked {
    /// A list of the version's collections.
    pub collections: Option<Vec<CreateApiVersionNonGitLinkedCollectionsItem>>,

    /// The version's name.
    pub name: Option<String>,

    /// Information about the API version release. For example, changelog notes.
    pub release_notes: Option<String>,

    /// A list of the version's schemas.
    pub schemas: Option<Vec<CreateApiVersionNonGitLinkedSchemasItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiVersionNonGitLinkedCollectionsItem {
    /// The collection's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiVersionNonGitLinkedSchemasItem {
    /// The schema's ID.
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiSchema {
    /// The list of files that are part of the schema.
    pub files: Vec<CreateApiSchemaFilesItem>,

    /// The schema's type.
    #[serde(rename = "type")]
    pub create_api_schema_type: PostApisApiIdSchemasBodyType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiSchemaFilesItem {
    /// The serilalized content of the schema.
    pub content: Option<String>,

    /// The schema's file path.
    pub path: Option<String>,

    /// Information about the schema's root file.
    pub root: Option<CreateApiSchemaFilesItemRoot>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateApiSchemaFilesItemRoot {
    /// If true, tag the file as the root file. The root tag is only allowed for protobuf
    /// specifications.
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePanApi {
    pub api: Option<CreatePanApiApi>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePanApiApi {
    /// The API's ID.
    pub id: String,

    /// The API's parent folder ID.
    pub parent_folder_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePanCollection {
    pub collection: Option<CreatePanCollectionCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePanCollectionCollection {
    /// A list of environments to add to the collection.
    pub environments: Option<Vec<String>>,

    /// The collection's ID.
    pub id: String,

    /// The collection's parent folder ID.
    pub parent_folder_id: i64,

    /// The collection's summary.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePanFolder {
    pub folder: Option<CreatePanFolderFolder>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePanFolderFolder {
    /// The folder's description.
    pub description: Option<String>,

    /// The folder's name.
    pub name: String,

    /// The folder's parent folder ID. This value defaults to `0`.
    pub parent_folder_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreatePanWorkspace {
    pub workspace: Option<CreatePanWorkspaceWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreatePanWorkspaceWorkspace {
    /// The workspace's ID.
    pub id: String,

    /// The workspace's parent folder ID.
    pub parent_folder_id: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUpdateApi {
    /// The API's description. This supports Markdown formatting.
    pub description: Option<String>,

    /// The API's name.
    pub name: String,

    /// The API's short summary.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitInfo {
    /// The API definition's collection repository folder location.
    pub collection_folder: Option<String>,

    /// The domain at which the Git repository is hosted.
    pub domain: Option<String>,

    /// The organization that owns the repository.
    pub organization: Option<String>,

    /// The repository's name.
    pub repository: Option<String>,

    /// The API definition's repository folder location. This returns an empty string if the
    /// connected repository uses a root file.
    pub schema_folder: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportExportFile {
    /// A file containing a valid user's export .zip file.
    pub input: String,

    /// The `file` type value.
    #[serde(rename = "type")]
    pub import_export_file_type: ImportExportFileType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportExportFileType {
    File,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonSchema {
    pub input: Option<serde_json::Value>,

    pub options: Option<serde_json::Value>,

    /// The OpenAPI definition type.
    #[serde(rename = "type")]
    pub json_schema_type: Option<JsonSchemaType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JsonSchemaType {
    Json,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonStringified {
    /// The stringified OpenAPI definition.
    pub input: Option<String>,

    pub options: Option<serde_json::Value>,

    /// The OpenAPI definition type.
    #[serde(rename = "type")]
    pub json_stringified_type: Option<JsonSchemaType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// The maximum number of records in the paginated response.
    pub limit: Option<f64>,

    /// The Base64-encoded value that points to the next record in the results set.
    pub next_cursor: Option<String>,

    /// The number of records that match the defined criteria.
    pub total: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFileBase {
    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<String>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaFileContents {
    /// The schema file's content.
    pub content: Option<String>,

    /// The date and time at which the file was created.
    pub created_at: Option<String>,

    /// The user Id of the user that created the file.
    pub created_by: Option<String>,

    /// The schema file's ID.
    pub id: Option<String>,

    /// The schema file's name.
    pub name: Option<String>,

    /// The file system path to the schema file.
    pub path: Option<String>,

    /// The date and time at which the file was last updated.
    pub updated_at: Option<String>,

    /// The user ID of the user that last updated the file.
    pub updated_by: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskError {
    pub error: Option<TaskErrorError>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskErrorError {
    /// The task's error message.
    pub message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskResource {
    pub resources: Option<Vec<TaskResourceResourcesItem>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskResourceResourcesItem {
    /// The task's ID.
    pub id: Option<String>,

    /// The task's assigned resource URL.
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePanApi {
    pub api: Option<UpdatePanApiApi>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePanApiApi {
    /// The API's new parent folder ID.
    pub parent_folder_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePanCollection {
    pub collection: Option<UpdatePanCollectionCollection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePanCollectionCollection {
    /// The collection's updated environments.
    pub environments: Option<UpdatePanCollectionCollectionEnvironments>,

    /// The collection's new parent folder ID.
    pub parent_folder_id: Option<i64>,

    /// The collection's updated summary.
    pub summary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePanCollectionCollectionEnvironments {
    #[serde(rename = "$add")]
    pub add: Option<Vec<String>>,

    #[serde(rename = "$remove")]
    pub remove: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePanFolder {
    pub folder: Option<UpdatePanFolderFolder>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePanFolderFolder {
    /// The folder's updated description.
    pub description: Option<String>,

    /// The folder's new name.
    pub name: Option<String>,

    /// The folder's new parent folder ID.
    pub parent_folder_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdatePanWorkspace {
    pub workspace: Option<UpdatePanWorkspaceWorkspace>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePanWorkspaceWorkspace {
    /// The workspace's new parent folder ID.
    pub parent_folder_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionBase {
    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// The version's release notes.
    pub release_notes: Option<String>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionExtended {
    /// A list of the version's collections.
    pub collections: Option<Vec<VersionExtendedCollectionsItem>>,

    /// The date and time at which the version was created.
    pub created_at: Option<String>,

    /// The version's ID.
    pub id: Option<String>,

    /// The version's name.
    pub name: Option<String>,

    /// The version's release notes.
    pub release_notes: Option<String>,

    /// A list of the version's API schemas.
    pub schemas: Option<Vec<VersionExtendedSchemasItem>>,

    /// The date and time at which the version was last updated.
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionExtendedCollectionsItem {
    /// The collection's unique ID.
    pub id: Option<String>,

    /// The collection's name.
    pub name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersionExtendedSchemasItem {
    /// The schema's unique ID.
    pub id: Option<String>,

    /// The type of schema.
    #[serde(rename = "type")]
    pub version_extended_schemas_item_type: Option<PostApisApiIdSchemasBodyType>,
}

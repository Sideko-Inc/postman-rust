# postman_api rust 

 The Postman API enables you to programmatically access data stored in your Postman account.

For a comprehensive set of examples of requests and responses, see the [**Postman API** collection](https://www.postman.com/postman/workspace/postman-public-workspace/documentation/12959542-c8142d51-e97c-46b6-bd77-52bb66712c9a).

## Important

- You must pass an `Accept` header with the `application/vnd.api.v10+json` value to use v10 and higher endpoints. While some of these endpoints may appear the same as the deprecated Postman v9 endpoints, they will use the v10 behavior when you send this `Accept` header. For more information, see [About v9 and v10 APIs](https://learning.postman.com/docs/developer/postman-api/intro-api/#about-v9-and-v10-apis).
- To use the **API** endpoints, you must first [update your APIs to the v10 format](https://learning.postman.com/docs/designing-and-developing-your-api/creating-an-api/#upgrading-an-api).

## Getting started

You can get started with the Postman API by [forking the Postman API collection](https://learning.postman.com/docs/collaborating-in-postman/version-control/#creating-a-fork) to your workspace. You can then use Postman to send requests.

## About the Postman API

- You must use a valid API Key to send requests to the API endpoints.
- The API has [rate and usage limits](https://learning.postman.com/docs/developer/postman-api/postman-api-rate-limits/).
- The API only responds to HTTPS-secured communications. Any requests sent via HTTP return an HTTP `301` redirect to the corresponding HTTPS resources.
- The API returns requests responses in [JSON format](https://en.wikipedia.org/wiki/JSON). When an API request returns an error, it is sent in the JSON response as an error key.
- The request method (verb) determines the nature of action you intend to perform. A request made using the `GET` method implies that you want to fetch something from Postman. The `POST` method implies you want to save something new to Postman.
- For all requests, API calls respond with their corresponding [HTTP status codes](https://en.wikipedia.org/wiki/List_of_HTTP_status_codes). In the Postman client, the status code also provides help text that details the possible meaning of the response code.

### IDs and UIDs

All items in Postman, such as collections, workspaces, and APIs, have IDs and UIDs:

- An ID is the unique ID assigned to a Postman item. For example, `ec29121c-5203-409f-9e84-e83ffc10f226`.
- The UID is the **full** ID of a Postman item. This value is the item's unique ID concatenated with the user ID. For example, in the `12345678-ec29121c-5203-409f-9e84-e83ffc10f226` UID:
    - `12345678` is the user's ID.
    - `ec29121c-5203-409f-9e84-e83ffc10f226` is the item's ID.

### 503 response

An HTTP `503 Service Unavailable` response from our servers indicates there is an unexpected spike in API access traffic. The server is usually operational within the next five minutes. If the outage persists or you receive any other form of an HTTP `5XX` error, [contact support](https://support.postman.com/hc/en-us/requests/new/).

## Authentication

Postman uses API keys for authentication. The API key tells the API server that the request came from you. Everything that you have access to in Postman is accessible with your API key. You can [generate](https://learning.postman.com/docs/developer/postman-api/authentication/#generate-a-postman-api-key) a Postman API key in the [**API keys**](https://postman.postman.co/settings/me/api-keys) section of your Postman account settings.

You must include an API key in each request to the Postman API with the `X-Api-Key` request header. In Postman, you can store your API key as an [environment variable](https://www.getpostman.com/docs/environments). The Postman API [collection](https://www.getpostman.com/docs/collections) will use it to make API calls.

### Authentication error response

If an API key is missing, malformed, or invalid, you will receive an HTTP `401 Unauthorized` response code.

### Using the API key as a query parameter

Requests that accept the `X-Api-Key` request header also accept the API key when you send it as the `apikey` query parameter. An API key sent as part of the header has a higher priority when you send the key as both a request header and a query parameter.

## Rate and usage limits

API access [rate limits](https://learning.postman.com/docs/developer/postman-api/postman-api-rate-limits/) apply at a per-API key basis in unit time. The limit is **300 requests per minute**. Also, depending on your [plan](https://www.postman.com/pricing/), you may have usage limits. If you exceed either limit, your request will return an HTTP `429 Too Many Requests` status code.

Each API response returns the following set of headers to help you identify your use status:

| Header | Description |
| ------ | ----------- |
| `X-RateLimit-Limit` | The maximum number of requests that the consumer is permitted to make per minute. |
| `X-RateLimit-Remaining` | The number of requests remaining in the current rate limit window. |
| `X-RateLimit-Reset` | The time at which the current rate limit window resets in UTC epoch seconds. |

## Support

For help regarding accessing the Postman API, you can:

- Visit [Postman Support](https://support.postman.com/hc/en-us) or our [Community and Support](https://www.postman.com/community/) sites.
- Reach out to the [Postman community](https://community.postman.com/).
- Submit a help request to [Postman support](https://support.postman.com/hc/en-us/requests/new/).

## Policies

- [Postman Terms of Service](http://www.postman.com/legal/terms/)
- [Postman Privacy Policy](https://www.postman.com/legal/privacy-policy/)
 

 # Authentication 
 Get an API key from [postman](https://web.postman.co/)


And pass it to your SDK
 
 ```rust
use postman_api::SidekoClient;
let client = SidekoClient::new(String::from("API_KEY")).unwrap();
```

# delete_api
Deletes an API.
```rust
let response = client.delete_api(DeleteApisApiIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# delete_schema_file
Deletes a file in an API schema.
```rust
let response = client.delete_schema_file(DeleteApisApiIdSchemasSchemaIdFilesFilePathRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    schema_id: "5381f010-c4c1-11ed-afa1-0242ac120002".to_string(),
    file_path: "postman/collection/c1.json".to_string(),
});
```
# delete_api_version
Deletes an API version.

**Note:**

This endpoint returns an HTTP `404 Not Found` response when an API version is pending publication.

```rust
let response = client.delete_api_version(DeleteApisApiIdVersionsVersionIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    version_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# delete_collection
Deletes a collection.
```rust
let response = client.delete_collection(DeleteCollectionsCollectionIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# delete_collection_folder
Deletes a folder in a collection.
```rust
let response = client.delete_collection_folder(DeleteCollectionsCollectionIdFoldersFolderIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    folder_id: "65a99e60-8e0a-4b6e-b79c-7d8264cc5caa".to_string(),
});
```
# delete_collection_request
Deletes a request in a collection.
```rust
let response = client.delete_collection_request(DeleteCollectionsCollectionIdRequestsRequestIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    request_id: "c82dd02c-4870-4907-8fcb-593a876cf05b".to_string(),
});
```
# delete_collection_response
Deletes a response in a collection.
```rust
let response = client.delete_collection_response(DeleteCollectionsCollectionIdResponsesResponseIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    response_id: "cc364734-7dfd-4bfc-897d-be763dcdbb07".to_string(),
});
```
# delete_environment
Deletes an environment.
```rust
let response = client.delete_environment(DeleteEnvironmentsEnvironmentIdRequest{
    environment_id: "5daabc50-8451-43f6-922d-96b403b4f28e".to_string(),
});
```
# delete_mock
Deletes a mock server.
```rust
let response = client.delete_mock(DeleteMocksMockIdRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# delete_mock_server_response
Deletes a mock server's server response.
```rust
let response = client.delete_mock_server_response(DeleteMocksMockIdServerResponsesServerResponseIdRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
    server_response_id: "965cdd16-fe22-4d96-a161-3d05490ac421".to_string(),
});
```
# unpublish_mock
Unpublishes a mock server. Unpublishing a mock server sets its **Access Control** configuration setting to private.
```rust
let response = client.unpublish_mock(DeleteMocksMockIdUnpublishRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# delete_monitor
Deletes a monitor.
```rust
let response = client.delete_monitor(DeleteMonitorsMonitorIdRequest{
    monitor_id: "1e6b6cc1-c760-48e0-968f-4bfaeeae9af1".to_string(),
});
```
# remove_element_or_folder
Removes an element or delete a folder from your [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/).

**Note:**

Removing an API, collection, or workspace element does **not** delete it. It only removes it from the Private API Network folder.
```rust
let response = client.remove_element_or_folder(DeleteNetworkPrivateElementTypeElementIdRequest{
    element_type: "api".to_string(),
    element_id: "5360b75f-447e-467c-9299-12fd6c92450d".to_string(),
});
```
# delete_group
Deletes a group in Postman.

User accounts that were in the deleted group are deactivated in Postman if the app is assigned to the user only with the deleted group.

User accounts and the data corresponding to them are **not** deleted. To permanently delete user accounts and their data, [contact Postman support](https://www.postman.com/support/).

```rust
let response = client.delete_group(DeleteScimV2GroupsGroupIdRequest{
    group_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# delete_workspace
Deletes an existing workspace.

### Important

If you delete a workspace that has a linked collection or environment with another workspace, this will delete the collection and environment in **all** workspaces.

```rust
let response = client.delete_workspace(DeleteWorkspacesWorkspaceIdRequest{
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# get_all_apis
Gets information about all APIs in a workspace.

**Note:**

This endpoint only returns APIs created or migrated in Postman v10 and higher.

```rust
let response = client.get_all_apis(GetApisRequest{
    workspace_id: "9a7bb368-c4c4-11ed-afa1-0242ac120002".to_string(),
    created_by: Some(12345678),
    cursor: Some("T1RBME5UQXo=".to_string()),
    description: Some("This is an API for testing purposes".to_string()),
    limit: Some(10),
});
```
# get_an_api
Gets information about an API.

**Note:**

- Git-connected APIs will **only** return the `versions` and `gitInfo` query responses. This is because schema and collection information is stored in the connected Git repository. The `gitInfo` object only lists the repository and folder locations of the files.
- API viewers can only use the `versions` option in the `include` query parameter.

```rust
let response = client.get_an_api(GetApisApiIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    include: "schemas,collections",
});
```
# get_collection
Gets a collection attached to an API. You can use the `versionId` query parameter to get a collection published in a version.

**Note:**

The `versionId` query parameter is a required parameter for API viewers.

```rust
let response = client.get_collection(GetApisApiIdCollectionsCollectionIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    collection_id: "12345678-61867bcc-c4c1-11ed-afa1-0242ac120002".to_string(),
    version_id: Some("12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string()),
});
```
# get_schema
Gets information about API schema. You can use the `versionId` query parameter to get a schema published in an API version.

You can use this API to do the following:

- Get a schema's metadata.
- Get all the files in a schema. This only returns the first file in the schema. The endpoint response contains a link to the next set of response results.
- Get a schema's contents in multi-file or bundled format.

**Note:**

The `versionId` query parameter is a **required** parameter for API viewers.

```rust
let response = client.get_schema(GetApisApiIdSchemasSchemaIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    schema_id: "5381f010-c4c1-11ed-afa1-0242ac120002".to_string(),
    bundled: Some(true),
    version_id: Some("12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string()),
});
```
# get_schema_files
Gets the files in an API schema. You can use the `versionId` query parameter to get schema files published in an API version.

**Note:**

The `versionId` query parameter is a required parameter for API viewers.

```rust
let response = client.get_schema_files(GetApisApiIdSchemasSchemaIdFilesRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    schema_id: "5381f010-c4c1-11ed-afa1-0242ac120002".to_string(),
    cursor: Some("T1RBME5UQXo=".to_string()),
    limit: Some(10),
    version_id: Some("12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string()),
});
```
# get_schema_file_contents
Gets an API schema file contents at the defined path. You can use the `versionId` query parameter to get schema file contents published in an API version.

**Note:**

The `versionId` query parameter is a required parameter for API viewers.

```rust
let response = client.get_schema_file_contents(GetApisApiIdSchemasSchemaIdFilesFilePathRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    schema_id: "5381f010-c4c1-11ed-afa1-0242ac120002".to_string(),
    file_path: "postman/collection/c1.json".to_string(),
    version_id: Some("12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string()),
});
```
# get_api_tags
Gets all the tags associated with an API.
```rust
let response = client.get_api_tags(GetApisApiIdTagsRequest{
    api_id: "12345678-6fd634a3-79ba-451d-8f07-56a953f96667".to_string(),
});
```
# get_status_of_an_async_task
Gets the status of an asynchronous task.
```rust
let response = client.get_status_of_an_async_task(GetApisApiIdTasksTaskIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    task_id: "90ca9f5a-c4c4-21ed-afa1-0242ac120002".to_string(),
});
```
# get_all_versions
Gets all the published versions of an API.
```rust
let response = client.get_all_versions(GetApisApiIdVersionsRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    cursor: Some("T1RBME5UQXo=".to_string()),
    limit: Some(10),
});
```
# get_api_version
Gets information about an API version.

**Note:**

- For API editors, this endpoint returns an HTTP `302 Found` status code when the version status is pending. It also returns the `/apis/{apiId}/tasks/{taskId}` task status response header.
- For API viewers, this endpoint returns an HTTP `404 Not Found` when the version status is pending.

```rust
let response = client.get_api_version(GetApisApiIdVersionsVersionIdRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    version_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# get_audit_logs
Gets a list of your team's generated audit events. For a complete list of all audit events, read our [Utilizing audit logs](https://learning.postman.com/docs/administration/audit-logs/) documentation.
```rust
let response = client.get_audit_logs(GetAuditLogsRequest{
    cursor: Some("string".to_string()),
    limit: Some(123),
    order_by: Some("string".to_string()),
    since: Some("1970-01-01".to_string()),
    until: Some("1970-01-01".to_string()),
});
```
# all_collections
Gets all of your [collections](https://www.getpostman.com/docs/collections). The response includes all of your subscribed collections.
```rust
let response = client.all_collections(GetCollectionsRequest{
    name: Some("Test Collection".to_string()),
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# single_collection
Gets information about a collection. For a complete list of this endpoint's possible values, use the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json).
```rust
let response = client.single_collection(GetCollectionsCollectionIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    access_key: Some("PMAT-XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string()),
});
```
# get_collection_folder
Gets information about a folder in a collection.
```rust
let response = client.get_collection_folder(GetCollectionsCollectionIdFoldersFolderIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    folder_id: "65a99e60-8e0a-4b6e-b79c-7d8264cc5caa".to_string(),
    ids: Some(true),
    populate: Some(true),
    uid: Some(true),
});
```
# get_collection_request
Gets information about a request in a collection.
```rust
let response = client.get_collection_request(GetCollectionsCollectionIdRequestsRequestIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    request_id: "c82dd02c-4870-4907-8fcb-593a876cf05b".to_string(),
    ids: Some("string".to_string()),
    populate: Some(true),
    uid: Some(true),
});
```
# get_collection_response
Gets information about a response in a collection.
```rust
let response = client.get_collection_response(GetCollectionsCollectionIdResponsesResponseIdRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    response_id: "cc364734-7dfd-4bfc-897d-be763dcdbb07".to_string(),
    ids: Some(true),
    populate: Some(true),
    uid: Some(true),
});
```
# get_collection_tags
Gets all the tags associated with a collection.
```rust
let response = client.get_collection_tags(GetCollectionsCollectionIdTagsRequest{
    collection_id: "12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# transform_collection_to_open_api
Transforms an existing Postman Collection into a stringified OpenAPI definition.

**Note:**

This does **not** create an API.

```rust
let response = client.transform_collection_to_open_api(GetCollectionsCollectionIdTransformationsRequest{
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# get_detected_secrets_locations
Gets the locations of secrets detected by Postman's [Secret Scanner](https://learning.postman.com/docs/administration/secret-scanner/).
```rust
let response = client.get_detected_secrets_locations(GetDetectedSecretsSecretIdLocationsRequest{
    secret_id: "ODk0MTU2".to_string(),
    workspace_id: "e361eeb4-00dd-4225-9774-6146a2555999".to_string(),
    cursor: Some("T1RBME5UQXo=".to_string()),
    limit: Some(10),
});
```
# all_environments
Gets information about all of your [environments](https://learning.postman.com/docs/sending-requests/managing-environments/).
```rust
let response = client.all_environments(GetEnvironmentsRequest{
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# single_environment
Gets information about an environment.
```rust
let response = client.single_environment(GetEnvironmentsEnvironmentIdRequest{
    environment_id: "5daabc50-8451-43f6-922d-96b403b4f28e".to_string(),
});
```
# api_key_owner
Gets information about the authenticated user.

**Note:**

This API returns a different response for users with the [Guest role](https://learning.postman.com/docs/collaborating-in-postman/roles-and-permissions/#team-roles).

```rust
let response = client.api_key_owner();
```
# get_mocks
Gets all mock servers. By default, this endpoint returns only mock servers you created across all workspaces.

**Note:**

If you pass both the `teamId` and `workspace` query parameters, this endpoint only accepts the `workspace` query.

```rust
let response = client.get_mocks(GetMocksRequest{
    team_id: Some("1b96f65f-8d23-4e1d-b5e2-055992c3b8cb".to_string()),
    workspace: Some("1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string()),
});
```
# get_mock
Gets information about a mock server.
```rust
let response = client.get_mock(GetMocksMockIdRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# get_mock_call_logs
Gets a mock server's call logs. You can get a maximum of 6.5MB of call logs or a total of 100 call logs, whichever limit is met first in one API call.

Call logs contain exchanged request and response data made to mock servers. The logs provide visibility into how the mock servers are being used. You can log data to debug, test, analyze, and more, depending upon the use case.

```rust
let response = client.get_mock_call_logs(GetMocksMockIdCallLogsRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
    cursor: Some("eyJzY2hlbWUiOiJjdXJzb3JfcGFnaW5hdGlvbklkIiwiZGlyZWN0aW9uVHlwZSI6Im5leHQiLCJwaXZvdCI6InBhZ2luYXRpb25JZCIsInZhbHVlIjoxNjQyNDAwMzU2MDAwNTc5fQ==".to_string()),
    direction: Some("asc".to_string()),
    include: Some("string".to_string()),
    limit: Some(3),
    request_method: Some("post".to_string()),
    request_path: Some("/animals?type=Dog".to_string()),
    response_status_code: Some(500),
    response_type: Some("success".to_string()),
    since: Some("2022-06-01T00:00:00.000Z".to_string()),
    sort: Some("updatedAt".to_string()),
    until: Some("2022-06-15T00:00:00.000Z".to_string()),
});
```
# get_mock_server_responses
Gets all of a mock server's server responses.
```rust
let response = client.get_mock_server_responses(GetMocksMockIdServerResponsesRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# get_mock_server_response
Gets information about a server response.
```rust
let response = client.get_mock_server_response(GetMocksMockIdServerResponsesServerResponseIdRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
    server_response_id: "965cdd16-fe22-4d96-a161-3d05490ac421".to_string(),
});
```
# all_monitors
Gets all monitors.
```rust
let response = client.all_monitors(GetMonitorsRequest{
    workspace: Some("1e6b6cc1-c760-48e0-968f-4bfaeeae9af1".to_string()),
});
```
# single_monitor
Gets information about a monitor.
```rust
let response = client.single_monitor(GetMonitorsMonitorIdRequest{
    monitor_id: "1e6b6cc1-c760-48e0-968f-4bfaeeae9af1".to_string(),
});
```
# get_all_elements_and_folders
Gets information about the folders and their elements added to your [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/).

**Note:**

The `limit` and `offset` parameters are separately applied to elements and folders. For example, if you query a `limit` value of `10` and an `offset` value `0`, the endpoint returns 10 elements and 10 folders for a total of 20 items. The `totalCount` property in the `meta` response is the total count of **both** elements and folders.
```rust
let response = client.get_all_elements_and_folders(GetNetworkPrivateRequest{
    added_by: Some(12345678),
    created_by: Some(12345678),
    description: Some("payments".to_string()),
    direction: Some("asc".to_string()),
    limit: Some(10),
    name: Some("billing".to_string()),
    offset: Some(5),
    parent_folder_id: Some(1),
    since: Some("2022-09-28T13:48:09.000Z".to_string()),
    sort: Some("updatedAt".to_string()),
    summary: Some("payments".to_string()),
    type_field: Some("api".to_string()),
    until: Some("2022-10-28T13:48:09.000Z".to_string()),
});
```
# get_all_add_element_requests
Gets a list requests to add elements to the [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/).
```rust
let response = client.get_all_add_element_requests(GetNetworkPrivateNetworkEntityRequestAllRequest{
    direction: Some("asc".to_string()),
    limit: Some(10),
    name: Some("Test api".to_string()),
    offset: Some(5),
    requested_by: Some(12345678),
    since: Some("2022-09-28T13:48:09.000Z".to_string()),
    sort: Some("updatedAt".to_string()),
    status: Some("pending".to_string()),
    type_field: Some("api".to_string()),
    until: Some("2022-10-28T13:48:09.000Z".to_string()),
});
```
# fetch_all_group_resources
Gets information about all Postman team members.
```rust
let response = client.fetch_all_group_resources(GetScimV2GroupsRequest{
    count: Some(2),
    filter: Some("displayName eq \"Test-API\"".to_string()),
    start_index: Some(1),
});
```
# fetch_group_resource
Gets information about a Postman group within the team.
```rust
let response = client.fetch_group_resource(GetScimV2GroupsGroupIdRequest{
    group_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# get_resource_types
Gets all the resource types supported by Postman's SCIM API.
```rust
let response = client.get_resource_types();
```
# service_provider_config
Gets the Postman SCIM API configuration information. This includes a list of supported operations.
```rust
let response = client.service_provider_config();
```
# fetch_all_user_resources
Gets information about all Postman team members.
```rust
let response = client.fetch_all_user_resources(GetScimV2UsersRequest{
    count: Some(50),
    filter: Some("userName eq \"taylor-lee%40example.com\"".to_string()),
    start_index: Some(1),
});
```
# fetch_user_resource
Gets information about a Postman team member.
```rust
let response = client.fetch_user_resource(GetScimV2UsersUserIdRequest{
    user_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# get_secret_types
Gets the metadata of the secret types supported by Postman's [Secret Scanner](https://learning.postman.com/docs/administration/secret-scanner/). You can use a secret type's ID in the response to query data with the POST `/detected-secrets/{secretId}` endpoint.
```rust
let response = client.get_secret_types();
```
# get_tagged_entities
Gets Postman elements (entities) by a given tag. Tags enable you to organize and search [workspaces](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#tagging-a-workspace), [APIs](https://learning.postman.com/docs/designing-and-developing-your-api/managing-apis/#tagging-apis), and [collections](https://learning.postman.com/docs/collections/using-collections/#tagging-a-collection) that contain shared tags.

**Note:**

Tagging is available on [Postman Enterprise plans](https://www.postman.com/pricing/).
```rust
let response = client.get_tagged_entities(GetTagsSlugEntitiesRequest{
    slug: "needs-review".to_string(),
    cursor: Some("eyJpZCI6ODYsImVudGl0eVR5cGUiOiJhcGkifQ==".to_string()),
    direction: Some("desc".to_string()),
    entity_type: Some("collection".to_string()),
    limit: Some(2),
});
```
# all_workspaces
Gets all [workspaces](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/creating-workspaces/). The response includes your workspaces and any workspaces that you have access to.

**Note:**

This endpoint's response contains the visibility field. Visibility determines who can access the workspace:

- `personal` — Only you can access the workspace.
- `team` — All team members can access the workspace.
- `private` — Only invited team members can access the workspace ([Professional and Enterprise plans only](https://www.postman.com/pricing)).
- `public` — Everyone can access the workspace.
- `partner` — Only invited team members and [partners](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/partner-workspaces/) can access the workspace ([Enterprise Ultimate plans](https://www.postman.com/pricing) only).

```rust
let response = client.all_workspaces(GetWorkspacesRequest{
    type_field: Some("team".to_string()),
});
```
# single_workspace
Gets information about a workspace.

**Note:**

This endpoint's response contains the `visibility` field. [Visibility](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/managing-workspaces/#changing-workspace-visibility) determines who can access the workspace:

- `personal` — Only you can access the workspace.
- `team` — All team members can access the workspace.
- `private` — Only invited team members can access the workspace ([Professional and Enterprise plans only](https://www.postman.com/pricing)).
- `public` — Everyone can access the workspace.
- `partner` — Only invited team members and [partners](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/partner-workspaces/) can access the workspace ([Enterprise Ultimate plans](https://www.postman.com/pricing) only).

### Important

We have **deprecated** the `name` and `uid` responses in the following array of objects:

- `collections`
- `environments`
- `mocks`
- `monitors`
- `apis`

```rust
let response = client.single_workspace(GetWorkspacesWorkspaceIdRequest{
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# get_workspace_global_variables
Gets a workspace's global [variables](https://learning.postman.com/docs/sending-requests/variables/#variable-scopes).
```rust
let response = client.get_workspace_global_variables(GetWorkspacesWorkspaceIdGlobalVariablesRequest{
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# get_workspace_tags
Gets all the tags associated with a workspace.
```rust
let response = client.get_workspace_tags(GetWorkspacesWorkspaceIdTagsRequest{
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# patch_collection
Updates specific collection information, such as its name, events, or its variables. For more information about the `auth`, `variables`, and `events` properties, refer to the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json):

- For `variables`, refer to `"#/definitions/variable"`.
- For `auth`, refer to `"#/definitions/auth-attribute"`.
- For `events`, refer to `"#/definitions/event"`.

For more information about the Collection Format, see the [Postman Collection Format documentation](https://learning.postman.com/collection-format/getting-started/overview/).

```rust
let data_val = serde_json::json!({});
let data: PatchCollectionsCollectionIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.patch_collection(PatchCollectionsCollectionIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# patch_scim_v2_groups_group_id
Updates a group's information. Using this endpoint you can:

- Update a group's name.
- Add or remove members from a Postman group.

```rust
let data_val = serde_json::json!({});
let data: PatchScimV2GroupsGroupIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.patch_scim_v2_groups_group_id(PatchScimV2GroupsGroupIdRequest{
    data,
    group_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# update_user_state
Updates a user's active state in Postman.

### Reactivating users

By setting the `active` property from `false` to `true`, this reactivates an account. This allows the account to authenticate in to Postman and adds the account back on to your Postman team.

```rust
let data_val = serde_json::json!({});
let data: PatchScimV2UsersUserIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_user_state(PatchScimV2UsersUserIdRequest{
    data,
    user_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# create_api
Creates an API.
```rust
let data_val = serde_json::json!({
  "name": "Test API"
});
let data: PostApisBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_api(PostApisRequest{
    data,
    workspace_id: "9a7bb368-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# add_collection
Adds a collection to an API. To do this, use the following `operationType` values:

- `COPY_COLLECTION` — Copies a collection from the workspace and adds it to an API.
- `CREATE_NEW` — Creates a new collection by providing the new collection's content.
- `GENERATE_FROM_SCHEMA` — Generates the collection from an API schema.
    - `options` — An object that contains advanced creation options and their values. You can find a complete list of properties and their values in Postman's OpenAPI 3.0 to Postman Collection v2.1.0 Converter OPTIONS documentation. These properties are case-sensitive.

```rust
let data = serde_json::json!({});

let response = client.add_collection(PostApisApiIdCollectionsRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# create_api_schema
Creates a schema for an API.
```rust
let data_val = serde_json::json!({
  "files": [
    {}
  ],
  "type": "openapi:3"
});
let data: PostApisApiIdSchemasBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_api_schema(PostApisApiIdSchemasRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# create_api_version
Creates a new API version asynchronously and immediately returns an HTTP `202 Accepted` response. The response contains a polling link to the task status API in the `Location` header.

This endpoint is equivalent to publishing a version in Postman app, which is the snapshot of API collections and schema at a given point in time.

```rust
let data = serde_json::json!({
  "collections": [
    {}
  ],
  "name": "v1",
  "schemas": [
    {}
  ]
});

let response = client.create_api_version(PostApisApiIdVersionsRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# create_collection
Creates a collection using the [Postman Collection v2 schema format](https://schema.postman.com/json/collection/v2.1.0/docs/index.html).

For more information about the Collection Format, see the [Postman Collection Format documentation](https://learning.postman.com/collection-format/getting-started/overview/).

**Note:**

- For a complete list of available property values for this endpoint, use the following references available in the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json):
    - `info` object — Use the `definitions.info` entry.
    - `item` object — Use the `definitions.items` entry.
- For all other possible values, refer to the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json).

```rust
let data_val = serde_json::json!({});
let data: PostCollectionsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_collection(PostCollectionsRequest{
    data,
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# create_a_fork
Creates a [fork](https://learning.postman.com/docs/collaborating-in-postman/version-control/#creating-a-fork) from an existing collection into a workspace.
```rust
let data_val = serde_json::json!({
  "label": "Test Fork"
});
let data: PostCollectionsForkCollectionIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_a_fork(PostCollectionsForkCollectionIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    workspace: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# merge_a_fork
Merges a forked collection back into its destination collection.
```rust
let data_val = serde_json::json!({
  "destination": "12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2",
  "source": "12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2"
});
let data: PostCollectionsMergeBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.merge_a_fork(PostCollectionsMergeRequest{
    data,
});
```
# create_collection_folder
Creates a folder in a collection. For a complete list of properties, refer to "Folder" in the [collection.json schema file](https://schema.postman.com/collection/json/v2.1.0/draft-07/collection.json).

You can use this endpoint to to import requests and responses into a newly-created folder. To do this, include the `requests` field and the list of request objects in the request body. For more information, see the provided example.

**Note:**

It is recommended that you pass the `name` property in the request body. If you do not, the system uses a null value. As a result, this creates a folder with a blank name.

```rust
let data_val = serde_json::json!({});
let data: PostCollectionsCollectionIdFoldersBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_collection_folder(PostCollectionsCollectionIdFoldersRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# create_collection_request
Creates a request in a collection. For a complete list of properties, see the [Collection Format Request documentation](https://learning.postman.com/collection-format/reference/request/).

**Note:**

It is recommended that you pass the `name` property in the request body. If you do not, the system uses a null value. As a result, this creates a request with a blank name.

```rust
let data_val = serde_json::json!({});
let data: PostCollectionsCollectionIdRequestsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_collection_request(PostCollectionsCollectionIdRequestsRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    folder_id: Some("string".to_string()),
});
```
# create_collection_response
Creates a request response in a collection. For a complete list of properties, see the [Collection Format Response documentation](https://learning.postman.com/collection-format/reference/response/#reference-diagram).

**Note:**

It is recommended that you pass the `name` property in the request body. If you do not, the system uses a null value. As a result, this creates a response with a blank name.

```rust
let data_val = serde_json::json!({});
let data: PostCollectionsCollectionIdResponsesBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_collection_response(PostCollectionsCollectionIdResponsesRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    request_id: "string".to_string(),
});
```
# detected_secrets_queries
Returns all secrets detected by Postman's [Secret Scanner](https://learning.postman.com/docs/administration/secret-scanner/), grouped by workspace. If you pass an empty request body, this endpoint returns all results.
```rust
let data_val = serde_json::json!({});
let data: PostDetectedSecretsQueriesBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.detected_secrets_queries(PostDetectedSecretsQueriesRequest{
    data,
    cursor: Some("T1RBME5UQXo=".to_string()),
    include: Some("meta.total".to_string()),
    limit: Some(10),
});
```
# create_environment
Creates an environment.
```rust
let data_val = serde_json::json!({});
let data: PostEnvironmentsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_environment(PostEnvironmentsRequest{
    data,
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# import_external_api_specification
Imports an OpenAPI definition into Postman as a new [Postman Collection](https://learning.postman.com/docs/getting-started/creating-the-first-collection/).
```rust
let data = serde_json::json!({});

let response = client.import_external_api_specification(PostImportOpenapiRequest{
    data,
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# create_mock
**In Postman v10 and higher you cannot create mocks for collections added to an API definition.**

Creates a mock server in a collection.

```rust
let data_val = serde_json::json!({});
let data: PostMocksBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_mock(PostMocksRequest{
    data,
    workspace_id: Some("1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string()),
});
```
# publish_mock
Publishes a mock server. Publishing a mock server sets its **Access Control** configuration setting to public.
```rust
let response = client.publish_mock(PostMocksMockIdPublishRequest{
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# create_server_response
Creates a server response. Server responses let you simulate 5xx server-level responses, such as 500 or 503.

Server-level responses are agnostic to application-level logic. Server responses let you simulate this behavior on a mock server. You do not need to define each error for all exposed paths on the mock server.

If you set a server response as active, then all the calls to the mock server return with that active server response.

**Note:**

You can create multiple server responses for a mock server, but only one mock server can be set as active.

```rust
let data_val = serde_json::json!({});
let data: PostMocksMockIdServerResponsesBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_server_response(PostMocksMockIdServerResponsesRequest{
    data,
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# create_monitor
**In Postman v10 and higher you cannot create monitors for collections added to an API definition.**

Creates a monitor.

```rust
let data_val = serde_json::json!({});
let data: PostMonitorsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_monitor(PostMonitorsRequest{
    data,
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# run_a_monitor
Runs a monitor and returns its run results.
```rust
let response = client.run_a_monitor(PostMonitorsMonitorIdRunRequest{
    monitor_id: "1e6b6cc1-c760-48e0-968f-4bfaeeae9af1".to_string(),
});
```
# post_element_or_folder
Publishes a element or creates a folder in your [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/). An element is a Postman API, collection, or workspace.
```rust
let data = serde_json::json!({});

let response = client.post_element_or_folder(PostNetworkPrivateRequest{
    data,
});
```
# create_group
Creates a new user group in Postman and creates a new account for each group member.

Each account is added to your Postman team and authentication is activated for each user. If an existing Postman account uses an email that matches a group member's email ID, an [email invite](https://postman.postman.co/docs/administration/managing-your-team/managing-your-team/#invites) to join your Postman team is sent to that user. Once the user accepts the invite, they'll be added to your team.

By default, the system assigns new users the developer role. You can [update user roles in Postman](https://learning.postman.com/docs/administration/managing-your-team/managing-your-team/#managing-team-roles).

```rust
let data_val = serde_json::json!({});
let data: PostScimV2GroupsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_group(PostScimV2GroupsRequest{
    data,
});
```
# create_user
Creates a new user account in Postman and adds the user to your organization's Postman team. If the account does not already exist, this also activates the user so they can authenticate in to your Postman team.

If the account already exists, the system sends the user an [email invite](https://learning.postman.com/docs/administration/managing-your-team/managing-your-team/#inviting-users) to join the Postman team. The user joins the team once they accept the invite.

By default, the system assigns new users the developer role. You can [update user roles in Postman](https://learning.postman.com/docs/administration/managing-your-team/managing-your-team/#managing-team-roles).

```rust
let data_val = serde_json::json!({});
let data: PostScimV2UsersBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_user(PostScimV2UsersRequest{
    data,
});
```
# schema_security_validation
Performs an analysis on the given definition and returns any issues based on your [predefined rulesets](https://learning.postman.com/docs/api-governance/configurable-rules/configurable-rules-overview/). This endpoint can help you understand the violations' impact and offers solutions to help you resolve any errors. You can include this endpoint to your CI/CD process to automate schema validation.

For more information, see our [Rule violations in the API definition](https://learning.postman.com/docs/api-governance/api-definition/api-definition-warnings/) documentation.

**Note:**

- The maximum allowed size of the definition is 10 MB.
- You must [import and enable](https://learning.postman.com/docs/api-governance/configurable-rules/configuring-api-security-rules/) [Postman's OWASP security rules](https://postman.postman.co/api-governance/libraries/postman_owasp/view) in Postman for this endpoint to return any security rule violations.

```rust
let data_val = serde_json::json!({});
let data: PostSecurityApiValidationBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.schema_security_validation(PostSecurityApiValidationRequest{
    data,
});
```
# create_webhook
Creates a webhook that triggers a collection with a custom payload. You can get the webhook's URL from the `webhookUrl` property in the endpoint's response.
```rust
let data_val = serde_json::json!({});
let data: PostWebhooksBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_webhook(PostWebhooksRequest{
    data,
    workspace_id: Some("e361eeb4-00dd-4225-9774-6146a2555999".to_string()),
});
```
# create_workspace
Creates a new [workspace](https://learning.postman.com/docs/collaborating-in-postman/using-workspaces/creating-workspaces/).

### Important

We **deprecated** linking collections or environments between workspaces. We do **not** recommend that you do this.

If you have a linked collection or environment, note the following:

- The endpoint does **not** create a clone of a collection or environment.
- Any changes you make to a linked collection or environment changes them in **all** workspaces.
- If you delete a collection or environment linked between workspaces, the system deletes it in **all** the workspaces.

```rust
let data_val = serde_json::json!({});
let data: PostWorkspacesBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_workspace(PostWorkspacesRequest{
    data,
});
```
# update_an_api
Updates an API.
```rust
let data_val = serde_json::json!({
  "name": "Test API"
});
let data: PutApisApiIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_an_api(PutApisApiIdRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
});
```
# sync_collection_with_schema
Syncs a collection attached to an API with the API schema.

This is an asynchronous endpoint that returns an HTTP `202 Accepted` response. The response contains a polling link to the `/apis/{apiId}/tasks/{taskId}` endpoint in the `Location` header.

**Note:**

This endpoint only supports the OpenAPI 3 schema type.

```rust
let response = client.sync_collection_with_schema(PutApisApiIdCollectionsCollectionIdSyncWithSchemaTasksRequest{
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    collection_id: "12345678-61867bcc-c4c1-11ed-afa1-0242ac120002".to_string(),
});
```
# create_or_update_schema_file
Creates or updates an API schema file.

**Note:**

- If the provided file path exists, the file will be updated with the new contents.
- If the provided file path does **not** exist, then a new schema file will be created.
- If the file path contains a `/` (forward slash) character, then a folder is created. For example, if the file path is the `dir/schema.json` value, then a `dir` folder is created with the `schema.json` file inside.

```rust
let data_val = serde_json::json!({
  "content": "string"
});
let data: PutApisApiIdSchemasSchemaIdFilesFilePathBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.create_or_update_schema_file(PutApisApiIdSchemasSchemaIdFilesFilePathRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    schema_id: "5381f010-c4c1-11ed-afa1-0242ac120002".to_string(),
    file_path: "postman/collection/c1.json".to_string(),
});
```
# update_api_tags
Updates an API's associated tags. This endpoint replaces all existing tags with those you pass in the request body.
```rust
let data_val = serde_json::json!({
  "tags": [
    {
      "slug": "needs-review"
    }
  ]
});
let data: PutApisApiIdTagsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_api_tags(PutApisApiIdTagsRequest{
    data,
    api_id: "12345678-6fd634a3-79ba-451d-8f07-56a953f96667".to_string(),
});
```
# update_api_version
Updates an API version.

**Note:**

This endpoint returns an HTTP `404 Not Found` response when an API version is pending publication.

```rust
let data_val = serde_json::json!({
  "name": "Release 1.5"
});
let data: PutApisApiIdVersionsVersionIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_api_version(PutApisApiIdVersionsVersionIdRequest{
    data,
    api_id: "90ca9f5a-c4c4-11ed-afa1-0242ac120002".to_string(),
    version_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# put_collection
Replaces the contents of a collection using the [Postman Collection v2 schema format](https://schema.postman.com/json/collection/v2.1.0/docs/index.html). Include the collection's ID values in the request body. If you do not, the endpoint removes the existing items and creates new items.

For a complete list of available property values for this endpoint, use the following references available in the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json):
- `info` object — Use `"#/definitions/info"`.
- `item` object — Use `"#/definitions/item"`.

For all other possible values, refer to the [collection.json schema file](https://schema.postman.com/json/collection/v2.1.0/collection.json). For more information about the Collection Format, see the [Postman Collection Format documentation](https://learning.postman.com/collection-format/getting-started/overview/).

**Note**

To copy another collection's contents to the given collection, **remove** all ID values before you pass it in this endpoint. If you do not, this endpoint returns an error. These values include the `id`, `uid`, and `postman_id` values.

```rust
let data_val = serde_json::json!({});
let data: PutCollectionsCollectionIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.put_collection(PutCollectionsCollectionIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# update_collection_folder
Updates a folder in a collection. For a complete list of properties, refer to "Folder" in the [collection.json schema file](https://schema.postman.com/collection/json/v2.1.0/draft-07/collection.json).

**Note:**

This endpoint acts like a PATCH method. It only updates the values that you pass in the request body (for example, the `name` property). The endpoint does **not** update the entire resource.

```rust
let data_val = serde_json::json!({});
let data: PutCollectionsCollectionIdFoldersFolderIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_collection_folder(PutCollectionsCollectionIdFoldersFolderIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    folder_id: "65a99e60-8e0a-4b6e-b79c-7d8264cc5caa".to_string(),
});
```
# update_collection_request
Updates a request in a collection. For a complete list of properties, see the [Collection Format Request documentation](https://learning.postman.com/collection-format/reference/request/).

**Note:**

- You must pass a collection ID (`12ece9e1-2abf-4edc-8e34-de66e74114d2`), not a collection(`12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2`), in this endpoint.
- This endpoint does not support changing the folder of a request.

```rust
let data_val = serde_json::json!({});
let data: PutCollectionsCollectionIdRequestsRequestIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_collection_request(PutCollectionsCollectionIdRequestsRequestIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    request_id: "c82dd02c-4870-4907-8fcb-593a876cf05b".to_string(),
});
```
# update_collection_response
Updates a response in a collection. For a complete list of properties, see the [Collection Format Response documentation](https://learning.postman.com/collection-format/reference/response/#reference-diagram).

**Note:**

- You must pass a collection ID (`12ece9e1-2abf-4edc-8e34-de66e74114d2`), not a collection UID (`12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2`), in this endpoint.
- This endpoint acts like a PATCH method. It only updates the values that you pass in the request body (for example, the `name` property). The endpoint does **not** update the entire resource.

```rust
let data_val = serde_json::json!({});
let data: PutCollectionsCollectionIdResponsesResponseIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_collection_response(PutCollectionsCollectionIdResponsesResponseIdRequest{
    data,
    collection_id: "12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
    response_id: "cc364734-7dfd-4bfc-897d-be763dcdbb07".to_string(),
});
```
# update_collection_tags
Updates a collection's associated tags. This endpoint replaces all existing tags with those you pass in the request body.
```rust
let data_val = serde_json::json!({
  "tags": [
    {
      "slug": "needs-review"
    }
  ]
});
let data: PutCollectionsCollectionIdTagsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_collection_tags(PutCollectionsCollectionIdTagsRequest{
    data,
    collection_id: "12345678-12ece9e1-2abf-4edc-8e34-de66e74114d2".to_string(),
});
```
# update_detected_secret_resolutions
Updates the resolution status of a secret detected in a workspace.
```rust
let data_val = serde_json::json!({
  "resolution": "ACCEPTED_RISK",
  "workspaceId": "e361eeb4-00dd-4225-9774-6146a2555999"
});
let data: PutDetectedSecretsSecretIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_detected_secret_resolutions(PutDetectedSecretsSecretIdRequest{
    data,
    secret_id: "ODk0MTU2".to_string(),
});
```
# update_environment
Updates an environment.
```rust
let data_val = serde_json::json!({});
let data: PutEnvironmentsEnvironmentIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_environment(PutEnvironmentsEnvironmentIdRequest{
    data,
    environment_id: "5daabc50-8451-43f6-922d-96b403b4f28e".to_string(),
});
```
# update_mock
Updates a mock server.
```rust
let data_val = serde_json::json!({});
let data: PutMocksMockIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_mock(PutMocksMockIdRequest{
    data,
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
});
```
# update_server_response
Updates a server response.
```rust
let data_val = serde_json::json!({});
let data: PutMocksMockIdServerResponsesServerResponseIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_server_response(PutMocksMockIdServerResponsesServerResponseIdRequest{
    data,
    mock_id: "e3d951bf-873f-49ac-a658-b2dcb91d3289".to_string(),
    server_response_id: "965cdd16-fe22-4d96-a161-3d05490ac421".to_string(),
});
```
# update_monitor
Updates a monitor.
```rust
let data_val = serde_json::json!({});
let data: PutMonitorsMonitorIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_monitor(PutMonitorsMonitorIdRequest{
    data,
    monitor_id: "1e6b6cc1-c760-48e0-968f-4bfaeeae9af1".to_string(),
});
```
# respond_element_add_request
Responds to a request to add an element to the [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/). Only managers can approve or deny a request. Once approved, the element will appear in the team's Private API Network.
```rust
let data_val = serde_json::json!({
  "status": "denied"
});
let data: PutNetworkPrivateNetworkEntityRequestRequestIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.respond_element_add_request(PutNetworkPrivateNetworkEntityRequestRequestIdRequest{
    data,
    request_id: 232,
});
```
# put_element_or_folder
Updates an element or folder in your [Private API Network](https://learning.postman.com/docs/collaborating-in-postman/adding-private-network/).
```rust
let data = serde_json::json!({});

let response = client.put_element_or_folder(PutNetworkPrivateElementTypeElementIdRequest{
    data,
    element_type: "api".to_string(),
    element_id: "5360b75f-447e-467c-9299-12fd6c92450d".to_string(),
});
```
# update_user_information
Updates a user's first and last name in Postman.

**Note:**

You can only use the SCIM API to update a user's first and last name. You cannot update any other user attributes with the API.

```rust
let data_val = serde_json::json!({});
let data: PutScimV2UsersUserIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_user_information(PutScimV2UsersUserIdRequest{
    data,
    user_id: "405775fe15ed41872a8eea4c8aa2b38cda9749812cc55c99".to_string(),
});
```
# update_workspace
Updates a workspace.

### Important

We **deprecated** linking collections or environments between workspaces. We do **not** recommend that you do this.

If you have a linked collection or environment, note the following:

- The endpoint does **not** create a clone of a collection or environment.
- Any changes you make to a linked collection or environment changes them in **all** workspaces.
- If you delete a collection or environment linked between workspaces, the system deletes it in **all** the workspaces.

```rust
let data_val = serde_json::json!({});
let data: PutWorkspacesWorkspaceIdBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_workspace(PutWorkspacesWorkspaceIdRequest{
    data,
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# put_workspace_global_variables
Updates and replaces a workspace's global [variables](https://learning.postman.com/docs/sending-requests/variables/#variable-scopes). This endpoint replaces all existing global variables with the variables you pass in the request body.
```rust
let data_val = serde_json::json!({});
let data: PutWorkspacesWorkspaceIdGlobalVariablesBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.put_workspace_global_variables(PutWorkspacesWorkspaceIdGlobalVariablesRequest{
    data,
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```
# update_workspace_tags
Updates a workspace's associated tags. This endpoint replaces all existing tags with those you pass in the request body.
```rust
let data_val = serde_json::json!({
  "tags": [
    {
      "slug": "needs-review"
    }
  ]
});
let data: PutWorkspacesWorkspaceIdTagsBody = serde_json::from_value(data_val).expect("invalid json value");

let response = client.update_workspace_tags(PutWorkspacesWorkspaceIdTagsRequest{
    data,
    workspace_id: "1f0df51a-8658-4ee8-a2a1-d2567dfa09a9".to_string(),
});
```

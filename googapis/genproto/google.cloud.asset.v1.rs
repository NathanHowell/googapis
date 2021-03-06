/// An asset in Google Cloud and its temporal metadata, including the time window
/// when it was observed and its status during that window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemporalAsset {
    /// The time window when the asset data and state was observed.
    #[prost(message, optional, tag = "1")]
    pub window: ::std::option::Option<TimeWindow>,
    /// Whether the asset has been deleted or not.
    #[prost(bool, tag = "2")]
    pub deleted: bool,
    /// An asset in Google Cloud.
    #[prost(message, optional, tag = "3")]
    pub asset: ::std::option::Option<Asset>,
    /// State of prior_asset.
    #[prost(enumeration = "temporal_asset::PriorAssetState", tag = "4")]
    pub prior_asset_state: i32,
    /// Prior copy of the asset. Populated if prior_asset_state is PRESENT.
    /// Currently this is only set for responses in Real-Time Feed.
    #[prost(message, optional, tag = "5")]
    pub prior_asset: ::std::option::Option<Asset>,
}
pub mod temporal_asset {
    /// State of prior asset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriorAssetState {
        /// prior_asset is not applicable for the current asset.
        Unspecified = 0,
        /// prior_asset is populated correctly.
        Present = 1,
        /// Failed to set prior_asset.
        Invalid = 2,
        /// Current asset is the first known state.
        DoesNotExist = 3,
        /// prior_asset is a deletion.
        Deleted = 4,
    }
}
/// A time window specified by its `start_time` and `end_time`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// Start time of the time window (exclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End time of the time window (inclusive). If not specified, the current
    /// timestamp is used instead.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// An asset in Google Cloud. An asset can be any resource in the Google Cloud
/// [resource
/// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
/// a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy).
/// See [Supported asset
/// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
/// for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The last update timestamp of an asset. update_time is updated when
    /// create/update/delete operation is performed.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    /// See [Resource
    /// names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    /// See [Supported asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for more information.
    #[prost(string, tag = "2")]
    pub asset_type: std::string::String,
    /// A representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::std::option::Option<Resource>,
    /// A representation of the Cloud IAM policy set on a Google Cloud resource.
    /// There can be a maximum of one Cloud IAM policy set on any given resource.
    /// In addition, Cloud IAM policies inherit their granted access scope from any
    /// policies set on parent resources in the resource hierarchy. Therefore, the
    /// effectively policy is the union of both the policy set on this resource
    /// and each policy set on all of the resource's ancestry resource levels in
    /// the hierarchy. See
    /// [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for
    /// more information.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::std::option::Option<super::super::super::iam::v1::Policy>,
    /// A representation of an [organization
    /// policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy).
    /// There can be more than one organization policy with different constraints
    /// set on a given resource.
    #[prost(message, repeated, tag = "6")]
    pub org_policy: ::std::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset
    /// is a project, folder, or organization, the ancestry path starts from the
    /// asset itself.
    ///
    /// Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag = "10")]
    pub ancestors: ::std::vec::Vec<std::string::String>,
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[prost(oneof = "asset::AccessContextPolicy", tags = "7, 8, 9")]
    pub access_context_policy: ::std::option::Option<asset::AccessContextPolicy>,
}
pub mod asset {
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        /// Please also refer to the [access policy user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
        #[prost(message, tag = "7")]
        AccessPolicy(super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy),
        /// Please also refer to the [access level user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels).
        #[prost(message, tag = "8")]
        AccessLevel(super::super::super::super::identity::accesscontextmanager::v1::AccessLevel),
        /// Please also refer to the [service perimeter user
        /// guide](https://cloud.google.com/vpc-service-controls/docs/overview).
        #[prost(message, tag = "9")]
        ServicePerimeter(
            super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter,
        ),
    }
}
/// A representation of a Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: `v1`
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// Example:
    /// `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "2")]
    pub discovery_document_uri: std::string::String,
    /// The JSON schema name listed in the discovery document. Example:
    /// `Project`
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "3")]
    pub discovery_name: std::string::String,
    /// The REST URL for accessing the resource. An HTTP `GET` request using this
    /// URL returns the resource itself. Example:
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`
    ///
    /// This value is unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: std::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    ///
    /// For Google Cloud assets, this value is the parent resource defined in the
    /// [Cloud IAM policy
    /// hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
    /// Example:
    /// `//cloudresourcemanager.googleapis.com/projects/my_project_123`
    ///
    /// For third-party assets, this field may be set differently.
    #[prost(string, tag = "5")]
    pub parent: std::string::String,
    /// The content of the resource, in which some sensitive fields are removed
    /// and may not be present.
    #[prost(message, optional, tag = "6")]
    pub data: ::std::option::Option<::prost_types::Struct>,
    /// The location of the resource in Google Cloud, such as its zone and region.
    /// For more information, see https://cloud.google.com/about/locations/.
    #[prost(string, tag = "8")]
    pub location: std::string::String,
}
/// A result of Resource Search, containing information of a cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSearchResult {
    /// The full resource name of this resource. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Cloud Asset Inventory Resource Name
    /// Format](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    /// for more information.
    ///
    /// To search against the `name`:
    ///
    /// * use a field query. Example: `name:instance1`
    /// * use a free text query. Example: `instance1`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The type of this resource. Example: `compute.googleapis.com/Disk`.
    ///
    /// To search against the `asset_type`:
    ///
    /// * specify the `asset_type` field in your search request.
    #[prost(string, tag = "2")]
    pub asset_type: std::string::String,
    /// The project that this resource belongs to, in the form of
    /// projects/{PROJECT_NUMBER}.
    ///
    /// To search against the `project`:
    ///
    /// * specify the `scope` field as this project in your search request.
    #[prost(string, tag = "3")]
    pub project: std::string::String,
    /// The display name of this resource.
    ///
    /// To search against the `display_name`:
    ///
    /// * use a field query. Example: `displayName:"My Instance"`
    /// * use a free text query. Example: `"My Instance"`
    #[prost(string, tag = "4")]
    pub display_name: std::string::String,
    /// One or more paragraphs of text description of this resource. Maximum length
    /// could be up to 1M bytes.
    ///
    /// To search against the `description`:
    ///
    /// * use a field query. Example: `description:"*important instance*"`
    /// * use a free text query. Example: `"*important instance*"`
    #[prost(string, tag = "5")]
    pub description: std::string::String,
    /// Location can be `global`, regional like `us-east1`, or zonal like
    /// `us-west1-b`.
    ///
    /// To search against the `location`:
    ///
    /// * use a field query. Example: `location:us-west*`
    /// * use a free text query. Example: `us-west*`
    #[prost(string, tag = "6")]
    pub location: std::string::String,
    /// Labels associated with this resource. See [Labelling and grouping GCP
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    ///
    /// To search against the `labels`:
    ///
    /// * use a field query:
    ///     - query on any label's key or value. Example: `labels:prod`
    ///     - query by a given label. Example: `labels.env:prod`
    ///     - query by a given label's existence. Example: `labels.env:*`
    /// * use a free text query. Example: `prod`
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Network tags associated with this resource. Like labels, network tags are a
    /// type of annotations used to group GCP resources. See [Labelling GCP
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    ///
    /// To search against the `network_tags`:
    ///
    /// * use a field query. Example: `networkTags:internal`
    /// * use a free text query. Example: `internal`
    #[prost(string, repeated, tag = "8")]
    pub network_tags: ::std::vec::Vec<std::string::String>,
    /// The additional searchable attributes of this resource. The attributes may
    /// vary from one resource type to another. Examples: `projectId` for Project,
    /// `dnsName` for DNS ManagedZone. This field contains a subset of the resource
    /// metadata fields that are returned by the List or Get APIs provided by the
    /// corresponding GCP service (e.g., Compute Engine). see [API references and
    /// supported searchable
    /// attributes](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types)
    /// for more information.
    ///
    /// You can search values of these fields through free text search. However,
    /// you should not consume the field programically as the field names and
    /// values may change as the GCP service updates to a new incompatible API
    /// version.
    ///
    /// To search against the `additional_attributes`:
    ///
    /// * use a free text query to match the attributes values. Example: to search
    ///   `additional_attributes = { dnsName: "foobar" }`, you can issue a query
    ///   `foobar`.
    #[prost(message, optional, tag = "9")]
    pub additional_attributes: ::std::option::Option<::prost_types::Struct>,
}
/// A result of IAM Policy search, containing information of an IAM policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicySearchResult {
    /// The full resource name of the resource associated with this IAM policy.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Cloud Asset Inventory Resource Name
    /// Format](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    /// for more information.
    ///
    /// To search against the `resource`:
    ///
    /// * use a field query. Example: `resource:organizations/123`
    #[prost(string, tag = "1")]
    pub resource: std::string::String,
    /// The project that the associated GCP resource belongs to, in the form of
    /// projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM
    /// instance, Cloud Storage bucket), the project field will indicate the
    /// project that contains the resource. If an IAM policy is set on a folder or
    /// orgnization, the project field will be empty.
    ///
    /// To search against the `project`:
    ///
    /// * specify the `scope` field as this project in your search request.
    #[prost(string, tag = "2")]
    pub project: std::string::String,
    /// The IAM policy directly set on the given resource. Note that the original
    /// IAM policy can contain multiple bindings. This only contains the bindings
    /// that match the given query. For queries that don't contain a constrain on
    /// policies (e.g., an empty query), this contains all the bindings.
    ///
    /// To search against the `policy` bindings:
    ///
    /// * use a field query:
    ///     - query by the policy contained members. Example:
    ///       `policy:amy@gmail.com`
    ///     - query by the policy contained roles. Example:
    ///       `policy:roles/compute.admin`
    ///     - query by the policy contained roles' included permissions. Example:
    ///       `policy.role.permissions:compute.instances.create`
    #[prost(message, optional, tag = "3")]
    pub policy: ::std::option::Option<super::super::super::iam::v1::Policy>,
    /// Explanation about the IAM policy search result. It contains additional
    /// information to explain why the search result matches the query.
    #[prost(message, optional, tag = "4")]
    pub explanation: ::std::option::Option<iam_policy_search_result::Explanation>,
}
pub mod iam_policy_search_result {
    /// Explanation about the IAM policy search result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explanation {
        /// The map from roles to their included permissions that match the
        /// permission query (i.e., a query containing `policy.role.permissions:`).
        /// Example: if query `policy.role.permissions:compute.disk.get`
        /// matches a policy binding that contains owner role, the
        /// matched_permissions will be `{"roles/owner": ["compute.disk.get"]}`. The
        /// roles can also be found in the returned `policy` bindings. Note that the
        /// map is populated only for requests with permission queries.
        #[prost(map = "string, message", tag = "1")]
        pub matched_permissions:
            ::std::collections::HashMap<std::string::String, explanation::Permissions>,
    }
    pub mod explanation {
        /// IAM permissions
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Permissions {
            /// A list of permissions. A sample permission string: `compute.disk.get`.
            #[prost(string, repeated, tag = "1")]
            pub permissions: ::std::vec::Vec<std::string::String>,
        }
    }
}
/// Export asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsRequest {
    /// Required. The relative name of the root asset. This can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id"), or a project number (such as "projects/12345"),
    /// or a folder number (such as "folders/123").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp
    /// between the current time and the current time minus 35 days (inclusive).
    /// If not specified, the current time will be used. Due to delays in resource
    /// data collection and indexing, there is a volatile window during which
    /// running the same query may get different results.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// A list of asset types to take a snapshot for. For example:
    /// "compute.googleapis.com/Disk".
    ///
    /// Regular expressions are also supported. For example:
    ///
    /// * "compute.googleapis.com.*" snapshots resources whose asset type starts
    /// with "compute.googleapis.com".
    /// * ".*Instance" snapshots resources whose asset type ends with "Instance".
    /// * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    /// See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported
    /// regular expression syntax. If the regular expression does not match any
    /// supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    /// If specified, only matching assets will be returned, otherwise, it will
    /// snapshot all asset types. See [Introduction to Cloud Asset
    /// Inventory](https://cloud.google.com/asset-inventory/docs/overview)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::std::vec::Vec<std::string::String>,
    /// Asset content type. If not specified, no content but the asset name will be
    /// returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Output configuration indicating where the results will be output to.
    #[prost(message, optional, tag = "5")]
    pub output_config: ::std::option::Option<OutputConfig>,
}
/// The export asset response. This message is returned by the
/// [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation] method in the returned
/// [google.longrunning.Operation.response][google.longrunning.Operation.response] field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::std::option::Option<OutputConfig>,
    /// Output result indicating where the assets were exported to. For example, a
    /// set of actual Google Cloud Storage object uris where the assets are
    /// exported to. The uris can be different from what [output_config] has
    /// specified, as the service will split the output object into multiple ones
    /// once it exceeds a single Google Cloud Storage object limit.
    #[prost(message, optional, tag = "3")]
    pub output_result: ::std::option::Option<OutputResult>,
}
/// Batch get assets history request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryRequest {
    /// Required. The relative name of the root asset. It can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id")", or a project number (such as "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A list of the full names of the assets.
    /// See: https://cloud.google.com/asset-inventory/docs/resource-name-format
    /// Example:
    ///
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///
    /// The request becomes a no-op if the asset name list is empty, and the max
    /// size of the asset name list is 100 in one request.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::std::vec::Vec<std::string::String>,
    /// Optional. The content type.
    #[prost(enumeration = "ContentType", tag = "3")]
    pub content_type: i32,
    /// Optional. The time window for the asset history. Both start_time and
    /// end_time are optional and if set, it must be after the current time minus
    /// 35 days. If end_time is not set, it is default to current timestamp.
    /// If start_time is not set, the snapshot of the assets at end_time will be
    /// returned. The returned results contain all temporal assets whose time
    /// window overlap with read_time_window.
    #[prost(message, optional, tag = "4")]
    pub read_time_window: ::std::option::Option<TimeWindow>,
}
/// Batch get assets history response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryResponse {
    /// A list of assets with valid time windows.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::std::vec::Vec<TemporalAsset>,
}
/// Create asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// Required. The name of the project/folder/organization where this feed
    /// should be created in. It can only be an organization number (such as
    /// "organizations/123"), a folder number (such as "folders/123"), a project ID
    /// (such as "projects/my-project-id")", or a project number (such as
    /// "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. This is the client-assigned asset feed identifier and it needs to
    /// be unique under a specific parent project/folder/organization.
    #[prost(string, tag = "2")]
    pub feed_id: std::string::String,
    /// Required. The feed details. The field `name` must be empty and it will be generated
    /// in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(message, optional, tag = "3")]
    pub feed: ::std::option::Option<Feed>,
}
/// Get asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    /// Required. The name of the Feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// List asset feeds request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsRequest {
    /// Required. The parent project/folder/organization whose feeds are to be
    /// listed. It can only be using project/folder/organization number (such as
    /// "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsResponse {
    /// A list of feeds.
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::std::vec::Vec<Feed>,
}
/// Update asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    /// Required. The new values of feed details. It must match an existing feed and the
    /// field `name` must be in the format of:
    /// projects/project_number/feeds/feed_id or
    /// folders/folder_number/feeds/feed_id or
    /// organizations/organization_number/feeds/feed_id.
    #[prost(message, optional, tag = "1")]
    pub feed: ::std::option::Option<Feed>,
    /// Required. Only updates the `feed` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    /// Required. The name of the feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Output configuration for export assets destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Asset export destination.
    #[prost(oneof = "output_config::Destination", tags = "1, 2")]
    pub destination: ::std::option::Option<output_config::Destination>,
}
pub mod output_config {
    /// Asset export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
        /// Destination on BigQuery. The output table stores the fields in asset
        /// proto as columns in BigQuery.
        #[prost(message, tag = "2")]
        BigqueryDestination(super::BigQueryDestination),
    }
}
/// Output result of export assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputResult {
    /// Asset export result.
    #[prost(oneof = "output_result::Result", tags = "1")]
    pub result: ::std::option::Option<output_result::Result>,
}
pub mod output_result {
    /// Asset export result.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Export result on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsResult(super::GcsOutputResult),
    }
}
/// A Cloud Storage output result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsOutputResult {
    /// List of uris of the Cloud Storage objects. Example:
    /// "gs://bucket_name/object_name".
    #[prost(string, repeated, tag = "1")]
    pub uris: ::std::vec::Vec<std::string::String>,
}
/// A Cloud Storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required.
    #[prost(oneof = "gcs_destination::ObjectUri", tags = "1, 2")]
    pub object_uri: ::std::option::Option<gcs_destination::ObjectUri>,
}
pub mod gcs_destination {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        /// The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. Example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
        /// for more information.
        #[prost(string, tag = "1")]
        Uri(std::string::String),
        /// The uri prefix of all generated Cloud Storage objects. Example:
        /// "gs://bucket_name/object_name_prefix". Each object uri is in format:
        /// "gs://bucket_name/object_name_prefix/<asset type>/<shard number> and only
        /// contains assets for that type. <shard number> starts from 0. Example:
        /// "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is
        /// the first shard of output objects containing all
        /// compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be
        /// returned if file with the same name "gs://bucket_name/object_name_prefix"
        /// already exists.
        #[prost(string, tag = "2")]
        UriPrefix(std::string::String),
    }
}
/// A BigQuery destination for exporting assets to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDestination {
    /// Required. The BigQuery dataset in format
    /// "projects/projectId/datasets/datasetId", to which the snapshot result
    /// should be exported. If this dataset does not exist, the export call returns
    /// an INVALID_ARGUMENT error.
    #[prost(string, tag = "1")]
    pub dataset: std::string::String,
    /// Required. The BigQuery table to which the snapshot result should be
    /// written. If this table does not exist, a new table with the given name
    /// will be created.
    #[prost(string, tag = "2")]
    pub table: std::string::String,
    /// If the destination table already exists and this flag is `TRUE`, the
    /// table will be overwritten by the contents of assets snapshot. If the flag
    /// is `FALSE` or unset and the destination table already exists, the export
    /// call returns an INVALID_ARGUMEMT error.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// [partition_spec] determines whether to export to partitioned table(s) and
    /// how to partition the data.
    ///
    /// If [partition_spec] is unset or [partition_spec.partion_key] is unset or
    /// `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to
    /// non-partitioned table(s). [force] will decide whether to overwrite existing
    /// table(s).
    ///
    /// If [partition_spec] is specified. First, the snapshot results will be
    /// written to partitioned table(s) with two additional timestamp columns,
    /// readTime and requestTime, one of which will be the partition key. Secondly,
    /// in the case when any destination table already exists, it will first try to
    /// update existing table's schema as necessary by appending additional
    /// columns. Then, if [force] is `TRUE`, the corresponding partition will be
    /// overwritten by the snapshot results (data in different partitions will
    /// remain intact); if [force] is unset or `FALSE`, it will append the data. An
    /// error will be returned if the schema update or data appension fails.
    #[prost(message, optional, tag = "4")]
    pub partition_spec: ::std::option::Option<PartitionSpec>,
    /// If this flag is `TRUE`, the snapshot results will be written to one or
    /// multiple tables, each of which contains results of one asset type. The
    /// [force] and [partition_spec] fields will apply to each of them.
    ///
    /// Field [table] will be concatenated with "_" and the asset type names (see
    /// https://cloud.google.com/asset-inventory/docs/supported-asset-types for
    /// supported asset types) to construct per-asset-type table names, in which
    /// all non-alphanumeric characters like "." and "/" will be substituted by
    /// "_". Example: if field [table] is "mytable" and snapshot results
    /// contain "storage.googleapis.com/Bucket" assets, the corresponding table
    /// name will be "mytable_storage_googleapis_com_Bucket". If any of these
    /// tables does not exist, a new table with the concatenated name will be
    /// created.
    ///
    /// When [content_type] in the ExportAssetsRequest is `RESOURCE`, the schema of
    /// each table will include RECORD-type columns mapped to the nested fields in
    /// the Asset.resource.data field of that asset type (up to the 15 nested level
    /// BigQuery supports
    /// (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The
    /// fields in >15 nested levels will be stored in JSON format string as a child
    /// column of its parent RECORD column.
    ///
    /// If error occurs when exporting to any table, the whole export call will
    /// return an error but the export results that already succeed will persist.
    /// Example: if exporting to table_type_A succeeds when exporting to
    /// table_type_B fails during one export call, the results in table_type_A will
    /// persist and there will not be partial results persisting in a table.
    #[prost(bool, tag = "5")]
    pub separate_tables_per_asset_type: bool,
}
/// Specifications of BigQuery partitioned table as export destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionSpec {
    /// The partition key for BigQuery partitioned table.
    #[prost(enumeration = "partition_spec::PartitionKey", tag = "1")]
    pub partition_key: i32,
}
pub mod partition_spec {
    /// This enum is used to determine the partition key column when exporting
    /// assets to BigQuery partitioned table(s). Note that, if the partition key is
    /// a timestamp column, the actual partition is based on its date value
    /// (expressed in UTC. see details in
    /// https://cloud.google.com/bigquery/docs/partitioned-tables#date_timestamp_partitioned_tables).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionKey {
        /// Unspecified partition key. If used, it means using non-partitioned table.
        Unspecified = 0,
        /// The time when the snapshot is taken. If specified as partition key, the
        /// result table(s) is partitoned by the additional timestamp column,
        /// readTime. If [read_time] in ExportAssetsRequest is specified, the
        /// readTime column's value will be the same as it. Otherwise, its value will
        /// be the current time that is used to take the snapshot.
        ReadTime = 1,
        /// The time when the request is received and started to be processed. If
        /// specified as partition key, the result table(s) is partitoned by the
        /// requestTime column, an additional timestamp column representing when the
        /// request was received.
        RequestTime = 2,
    }
}
/// A Pub/Sub destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish to.
    /// Example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
}
/// Output configuration for asset feed destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOutputConfig {
    /// Asset feed destination.
    #[prost(oneof = "feed_output_config::Destination", tags = "1")]
    pub destination: ::std::option::Option<feed_output_config::Destination>,
}
pub mod feed_output_config {
    /// Asset feed destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Pub/Sub.
        #[prost(message, tag = "1")]
        PubsubDestination(super::PubsubDestination),
    }
}
/// An asset feed used to export asset updates to a destinations.
/// An asset feed filter controls what updates are exported.
/// The asset feed must be created within a project, organization, or
/// folder. Supported destinations are:
/// Pub/Sub topics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// Required. The format will be
    /// projects/{project_number}/feeds/{client-assigned_feed_identifier} or
    /// folders/{folder_number}/feeds/{client-assigned_feed_identifier} or
    /// organizations/{organization_number}/feeds/{client-assigned_feed_identifier}
    ///
    /// The client-assigned feed identifier must be unique within the parent
    /// project/folder/organization.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A list of the full names of the assets to receive updates. You must specify
    /// either or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more info.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::std::vec::Vec<std::string::String>,
    /// A list of types of the assets to receive updates. You must specify either
    /// or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed.
    /// Example: `"compute.googleapis.com/Disk"`
    ///
    /// See [this
    /// topic](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for a list of all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::std::vec::Vec<std::string::String>,
    /// Asset content type. If not specified, no content but the asset name and
    /// type will be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Feed output configuration defining where the asset updates are
    /// published to.
    #[prost(message, optional, tag = "5")]
    pub feed_output_config: ::std::option::Option<FeedOutputConfig>,
    /// A condition which determines whether an asset update should be published.
    /// If specified, an asset will be returned only when the expression evaluates
    /// to true.
    /// When set, `expression` field in the `Expr` must be a valid [CEL expression]
    /// (https://github.com/google/cel-spec) on a TemporalAsset with name
    /// `temporal_asset`. Example: a Feed with expression ("temporal_asset.deleted
    /// == true") will only publish Asset deletions. Other fields of `Expr` are
    /// optional.
    ///
    /// See our [user
    /// guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes#feed_with_condition)
    /// for detailed instructions.
    #[prost(message, optional, tag = "6")]
    pub condition: ::std::option::Option<super::super::super::r#type::Expr>,
}
/// Search all resources request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesRequest {
    /// Required. A scope can be a project, a folder, or an organization. The search is
    /// limited to the resources within the `scope`. The caller must be granted the
    /// [`cloudasset.assets.searchAllResources`](http://cloud.google.com/asset-inventory/docs/access-control#required_permissions)
    /// permission on the desired scope.
    ///
    /// The allowed values are:
    ///
    /// * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    /// * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    /// * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    /// * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag = "1")]
    pub scope: std::string::String,
    /// Optional. The query statement. See [how to construct a
    /// query](http://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query)
    /// for more information. If not specified or empty, it will search all the
    /// resources within the specified `scope`. Note that the query string is
    /// compared against each Cloud IAM policy binding, including its members,
    /// roles, and Cloud IAM conditions. The returned Cloud IAM policies will only
    /// contain the bindings that match your query. To learn more about the IAM
    /// policy structure, see [IAM policy
    /// doc](https://cloud.google.com/iam/docs/policies#structure).
    ///
    /// Examples:
    ///
    /// * `name:Important` to find Cloud resources whose name contains
    ///   "Important" as a word.
    /// * `displayName:Impor*` to find Cloud resources whose display name
    ///   contains "Impor" as a prefix.
    /// * `description:*por*` to find Cloud resources whose description
    ///   contains "por" as a substring.
    /// * `location:us-west*` to find Cloud resources whose location is
    ///   prefixed with "us-west".
    /// * `labels:prod` to find Cloud resources whose labels contain "prod" as
    ///   a key or value.
    /// * `labels.env:prod` to find Cloud resources that have a label "env"
    ///   and its value is "prod".
    /// * `labels.env:*` to find Cloud resources that have a label "env".
    /// * `Important` to find Cloud resources that contain "Important" as a word
    ///   in any of the searchable fields.
    /// * `Impor*` to find Cloud resources that contain "Impor" as a prefix
    ///   in any of the searchable fields.
    /// * `*por*` to find Cloud resources that contain "por" as a substring in
    ///   any of the searchable fields.
    /// * `Important location:(us-west1 OR global)` to find Cloud
    ///   resources that contain "Important" as a word in any of the searchable
    ///   fields and are also located in the "us-west1" region or the "global"
    ///   location.
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Optional. A list of asset types that this request searches for. If empty, it will
    /// search all the [searchable asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types).
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::std::vec::Vec<std::string::String>,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the preceding call
    /// to this method. `page_token` must be the value of `next_page_token` from
    /// the previous response. The values of all other method parameters, must be
    /// identical to those in the previous call.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
    /// Optional. A comma separated list of fields specifying the sorting order of the
    /// results. The default order is ascending. Add " DESC" after the field name
    /// to indicate descending order. Redundant space characters are ignored.
    /// Example: "location DESC, name". Only string fields in the response are
    /// sortable, including `name`, `displayName`, `description`, `location`. All
    /// the other fields such as repeated fields (e.g., `networkTags`), map
    /// fields (e.g., `labels`) and struct fields (e.g., `additionalAttributes`)
    /// are not supported.
    #[prost(string, tag = "6")]
    pub order_by: std::string::String,
}
/// Search all resources response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesResponse {
    /// A list of Resources that match the search query. It contains the resource
    /// standard metadata information.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<ResourceSearchResult>,
    /// If there are more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Search all IAM policies request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesRequest {
    /// Required. A scope can be a project, a folder, or an organization. The search is
    /// limited to the IAM policies within the `scope`. The caller must be granted
    /// the
    /// [`cloudasset.assets.searchAllIamPolicies`](http://cloud.google.com/asset-inventory/docs/access-control#required_permissions)
    /// permission on the desired scope.
    ///
    /// The allowed values are:
    ///
    /// * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    /// * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    /// * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    /// * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag = "1")]
    pub scope: std::string::String,
    /// Optional. The query statement. See [how to construct a
    /// query](https://cloud.google.com/asset-inventory/docs/searching-iam-policies#how_to_construct_a_query)
    /// for more information. If not specified or empty, it will search all the
    /// IAM policies within the specified `scope`.
    ///
    /// Examples:
    ///
    /// * `policy:amy@gmail.com` to find IAM policy bindings that specify user
    ///   "amy@gmail.com".
    /// * `policy:roles/compute.admin` to find IAM policy bindings that specify
    ///   the Compute Admin role.
    /// * `policy.role.permissions:storage.buckets.update` to find IAM policy
    ///   bindings that specify a role containing "storage.buckets.update"
    ///   permission. Note that if callers don't have `iam.roles.get` access to a
    ///   role's included permissions, policy bindings that specify this role will
    ///   be dropped from the search results.
    /// * `resource:organizations/123456` to find IAM policy bindings
    ///   that are set on "organizations/123456".
    /// * `Important` to find IAM policy bindings that contain "Important" as a
    ///   word in any of the searchable fields (except for the included
    ///   permissions).
    /// * `*por*` to find IAM policy bindings that contain "por" as a substring
    ///   in any of the searchable fields (except for the included permissions).
    /// * `resource:(instance1 OR instance2) policy:amy` to find
    ///   IAM policy bindings that are set on resources "instance1" or
    ///   "instance2" and also specify user "amy".
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. If present, retrieve the next batch of results from the preceding call to
    /// this method. `page_token` must be the value of `next_page_token` from the
    /// previous response. The values of all other method parameters must be
    /// identical to those in the previous call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Search all IAM policies response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesResponse {
    /// A list of IamPolicy that match the search query. Related information such
    /// as the associated resource is returned along with the policy.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<IamPolicySearchResult>,
    /// Set if there are more results than those appearing in this response; to get
    /// the next set of results, call this method again, using this value as the
    /// `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Asset content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    /// Unspecified content type.
    Unspecified = 0,
    /// Resource metadata.
    Resource = 1,
    /// The actual IAM policy set on a resource.
    IamPolicy = 2,
    /// The Cloud Organization Policy set on an asset.
    OrgPolicy = 4,
    /// The Cloud Access context manager Policy set on an asset.
    AccessPolicy = 5,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Exports assets with time and resource types to a given Cloud Storage"]
        #[doc = " location/BigQuery table. For Cloud Storage location destinations, the"]
        #[doc = " output format is newline-delimited JSON. Each line represents a"]
        #[doc = " [google.cloud.asset.v1.Asset][google.cloud.asset.v1.Asset] in the JSON format; for BigQuery table"]
        #[doc = " destinations, the output table stores the fields in asset proto as columns."]
        #[doc = " This API implements the [google.longrunning.Operation][google.longrunning.Operation] API"]
        #[doc = " , which allows you to keep track of the export. We recommend intervals of"]
        #[doc = " at least 2 seconds with exponential retry to poll the export operation"]
        #[doc = " result. For regular-size resource parent, the export operation usually"]
        #[doc = " finishes within 5 minutes."]
        pub async fn export_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAssetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/ExportAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch gets the update history of assets that overlap a time window."]
        #[doc = " For IAM_POLICY content, this API outputs history when the asset and its"]
        #[doc = " attached IAM POLICY both exist. This can create gaps in the output history."]
        #[doc = " Otherwise, this API outputs history with asset in both non-delete or"]
        #[doc = " deleted status."]
        #[doc = " If a specified asset does not exist, this API returns an INVALID_ARGUMENT"]
        #[doc = " error."]
        pub async fn batch_get_assets_history(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetAssetsHistoryRequest>,
        ) -> Result<tonic::Response<super::BatchGetAssetsHistoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/BatchGetAssetsHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a feed in a parent project/folder/organization to listen to its"]
        #[doc = " asset updates."]
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/CreateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details about an asset feed."]
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.asset.v1.AssetService/GetFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all asset feeds in a parent project/folder/organization."]
        pub async fn list_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeedsRequest>,
        ) -> Result<tonic::Response<super::ListFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/ListFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an asset feed configuration."]
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/UpdateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an asset feed."]
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/DeleteFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches all Cloud resources within the specified scope, such as a project,"]
        #[doc = " folder, or organization. The caller must be granted the"]
        #[doc = " `cloudasset.assets.searchAllResources` permission on the desired scope,"]
        #[doc = " otherwise the request will be rejected."]
        pub async fn search_all_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchAllResourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/SearchAllResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches all IAM policies within the specified scope, such as a project,"]
        #[doc = " folder, or organization. The caller must be granted the"]
        #[doc = " `cloudasset.assets.searchAllIamPolicies` permission on the desired scope,"]
        #[doc = " otherwise the request will be rejected."]
        pub async fn search_all_iam_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllIamPoliciesRequest>,
        ) -> Result<tonic::Response<super::SearchAllIamPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/SearchAllIamPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AssetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssetServiceClient {{ ... }}")
        }
    }
}

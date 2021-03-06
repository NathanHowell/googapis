/// A [Secret][google.cloud.secretmanager.v1.Secret] is a logical secret whose value and versions can
/// be accessed.
///
/// A [Secret][google.cloud.secretmanager.v1.Secret] is made up of zero or more [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] that
/// represent the secret data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Secret {
    /// Output only. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The replication policy of the secret data attached to the [Secret][google.cloud.secretmanager.v1.Secret].
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    #[prost(message, optional, tag = "2")]
    pub replication: ::std::option::Option<Replication>,
    /// Output only. The time at which the [Secret][google.cloud.secretmanager.v1.Secret] was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// A secret version resource in the Secret Manager API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVersion {
    /// Output only. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the
    /// format `projects/*/secrets/*/versions/*`.
    ///
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] IDs in a [Secret][google.cloud.secretmanager.v1.Secret] start at 1 and
    /// are incremented for each subsequent version of the secret.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The time at which the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] was destroyed.
    /// Only present if [state][google.cloud.secretmanager.v1.SecretVersion.state] is
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED].
    #[prost(message, optional, tag = "3")]
    pub destroy_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(enumeration = "secret_version::State", tag = "4")]
    pub state: i32,
    /// The replication status of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(message, optional, tag = "5")]
    pub replication_status: ::std::option::Option<ReplicationStatus>,
}
pub mod secret_version {
    /// The state of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion], indicating if it can be accessed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. This value is unused and invalid.
        Unspecified = 0,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may be accessed.
        Enabled = 1,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may not be accessed, but the secret data
        /// is still available and can be placed back into the [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]
        /// state.
        Disabled = 2,
        /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] is destroyed and the secret data is no longer
        /// stored. A version may not leave this state once entered.
        Destroyed = 3,
    }
}
/// A policy that defines the replication and encryption configuration of data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Replication {
    /// The replication policy for this secret.
    #[prost(oneof = "replication::Replication", tags = "1, 2")]
    pub replication: ::std::option::Option<replication::Replication>,
}
pub mod replication {
    /// A replication policy that replicates the [Secret][google.cloud.secretmanager.v1.Secret] payload without any
    /// restrictions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Automatic {
        /// Optional. The customer-managed encryption configuration of the [Secret][google.cloud.secretmanager.v1.Secret]. If no
        /// configuration is provided, Google-managed default encryption is used.
        ///
        /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption configuration only apply to
        /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added afterwards. They do not apply
        /// retroactively to existing [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
        #[prost(message, optional, tag = "1")]
        pub customer_managed_encryption: ::std::option::Option<super::CustomerManagedEncryption>,
    }
    /// A replication policy that replicates the [Secret][google.cloud.secretmanager.v1.Secret] payload into the
    /// locations specified in [Secret.replication.user_managed.replicas][]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManaged {
        /// Required. The list of Replicas for this [Secret][google.cloud.secretmanager.v1.Secret].
        ///
        /// Cannot be empty.
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::std::vec::Vec<user_managed::Replica>,
    }
    pub mod user_managed {
        /// Represents a Replica for this [Secret][google.cloud.secretmanager.v1.Secret].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Replica {
            /// The canonical IDs of the location to replicate data.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: std::string::String,
            /// Optional. The customer-managed encryption configuration of the [User-Managed
            /// Replica][Replication.UserManaged.Replica]. If no configuration is
            /// provided, Google-managed default encryption is used.
            ///
            /// Updates to the [Secret][google.cloud.secretmanager.v1.Secret] encryption configuration only apply to
            /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] added afterwards. They do not apply
            /// retroactively to existing [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
            #[prost(message, optional, tag = "2")]
            pub customer_managed_encryption:
                ::std::option::Option<super::super::CustomerManagedEncryption>,
        }
    }
    /// The replication policy for this secret.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Replication {
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will automatically be replicated without any restrictions.
        #[prost(message, tag = "1")]
        Automatic(Automatic),
        /// The [Secret][google.cloud.secretmanager.v1.Secret] will only be replicated into the locations specified.
        #[prost(message, tag = "2")]
        UserManaged(UserManaged),
    }
}
/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagedEncryption {
    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt secret
    /// payloads.
    ///
    /// For secrets using the [UserManaged][google.cloud.secretmanager.v1.Replication.UserManaged] replication
    /// policy type, Cloud KMS CryptoKeys must reside in the same location as the
    /// [replica location][Secret.UserManaged.Replica.location].
    ///
    /// For secrets using the [Automatic][google.cloud.secretmanager.v1.Replication.Automatic] replication policy
    /// type, Cloud KMS CryptoKeys must reside in `global`.
    ///
    /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub kms_key_name: std::string::String,
}
/// The replication status of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationStatus {
    /// The replication status of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(oneof = "replication_status::ReplicationStatus", tags = "1, 2")]
    pub replication_status: ::std::option::Option<replication_status::ReplicationStatus>,
}
pub mod replication_status {
    /// The replication status of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using automatic replication.
    ///
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret] has an automatic replication
    /// policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomaticStatus {
        /// Output only. The customer-managed encryption status of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
        /// populated if customer-managed encryption is used.
        #[prost(message, optional, tag = "1")]
        pub customer_managed_encryption:
            ::std::option::Option<super::CustomerManagedEncryptionStatus>,
    }
    /// The replication status of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using user-managed
    /// replication.
    ///
    /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret] has a user-managed replication
    /// policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UserManagedStatus {
        /// Output only. The list of replica statuses for the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        #[prost(message, repeated, tag = "1")]
        pub replicas: ::std::vec::Vec<user_managed_status::ReplicaStatus>,
    }
    pub mod user_managed_status {
        /// Describes the status of a user-managed replica for the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ReplicaStatus {
            /// Output only. The canonical ID of the replica location.
            /// For example: `"us-east1"`.
            #[prost(string, tag = "1")]
            pub location: std::string::String,
            /// Output only. The customer-managed encryption status of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. Only
            /// populated if customer-managed encryption is used.
            #[prost(message, optional, tag = "2")]
            pub customer_managed_encryption:
                ::std::option::Option<super::super::CustomerManagedEncryptionStatus>,
        }
    }
    /// The replication status of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ReplicationStatus {
        /// Describes the replication status of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// automatic replication.
        ///
        /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret] has an automatic replication
        /// policy.
        #[prost(message, tag = "1")]
        Automatic(AutomaticStatus),
        /// Describes the replication status of a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] with
        /// user-managed replication.
        ///
        /// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret] has a user-managed replication
        /// policy.
        #[prost(message, tag = "2")]
        UserManaged(UserManagedStatus),
    }
}
/// Describes the status of customer-managed encryption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagedEncryptionStatus {
    /// Required. The resource name of the Cloud KMS CryptoKeyVersion used to encrypt the
    /// secret payload, in the following format:
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*/versions/*`.
    #[prost(string, tag = "1")]
    pub kms_key_version_name: std::string::String,
}
/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
}
/// Request message for [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsRequest {
    /// Required. The resource name of the project associated with the
    /// [Secrets][google.cloud.secretmanager.v1.Secret], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// [ListSecretsResponse.next_page_token][google.cloud.secretmanager.v1.ListSecretsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretsResponse {
    /// The list of [Secrets][google.cloud.secretmanager.v1.Secret] sorted in reverse by create_time (newest
    /// first).
    #[prost(message, repeated, tag = "1")]
    pub secrets: ::std::vec::Vec<Secret>,
    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretsRequest.page_token][google.cloud.secretmanager.v1.ListSecretsRequest.page_token] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [Secrets][google.cloud.secretmanager.v1.Secret].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSecretRequest {
    /// Required. The resource name of the project to associate with the
    /// [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    #[prost(string, tag = "2")]
    pub secret_id: std::string::String,
    /// Required. A [Secret][google.cloud.secretmanager.v1.Secret] with initial field values.
    #[prost(message, optional, tag = "3")]
    pub secret: ::std::option::Option<Secret>,
}
/// Request message for [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddSecretVersionRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] to associate with the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The secret payload of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<SecretPayload>,
}
/// Request message for [SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret], in the format `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] associated with the
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] to list, in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSecretVersionsResponse {
    /// The list of [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] sorted in reverse by
    /// create_time (newest first).
    #[prost(message, repeated, tag = "1")]
    pub versions: ::std::vec::Vec<SecretVersion>,
    /// A token to retrieve the next page of results. Pass this value in
    /// [ListSecretVersionsRequest.page_token][google.cloud.secretmanager.v1.ListSecretVersionsRequest.page_token] to retrieve the next page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [SecretManagerService.GetSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.GetSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    /// `projects/*/secrets/*/versions/latest` is an alias to the `latest`
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.UpdateSecret][google.cloud.secretmanager.v1.SecretManagerService.UpdateSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSecretRequest {
    /// Required. [Secret][google.cloud.secretmanager.v1.Secret] with updated field values.
    #[prost(message, optional, tag = "1")]
    pub secret: ::std::option::Option<Secret>,
    /// Required. Specifies the fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSecretVersionResponse {
    /// The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Secret payload
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<SecretPayload>,
}
/// Request message for [SecretManagerService.DeleteSecret][google.cloud.secretmanager.v1.SecretManagerService.DeleteSecret].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSecretRequest {
    /// Required. The resource name of the [Secret][google.cloud.secretmanager.v1.Secret] to delete in the format
    /// `projects/*/secrets/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.DisableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DisableSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to disable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.EnableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.EnableSecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableSecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to enable in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for [SecretManagerService.DestroySecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DestroySecretVersion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestroySecretVersionRequest {
    /// Required. The resource name of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to destroy in the format
    /// `projects/*/secrets/*/versions/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod secret_manager_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Secret Manager Service"]
    #[doc = ""]
    #[doc = " Manages secrets and operations using those secrets. Implements a REST"]
    #[doc = " model with the following objects:"]
    #[doc = ""]
    #[doc = " * [Secret][google.cloud.secretmanager.v1.Secret]"]
    #[doc = " * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]"]
    pub struct SecretManagerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SecretManagerServiceClient<T>
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
        #[doc = " Lists [Secrets][google.cloud.secretmanager.v1.Secret]."]
        pub async fn list_secrets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretsRequest>,
        ) -> Result<tonic::Response<super::ListSecretsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecrets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]."]
        pub async fn create_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/CreateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] containing secret data and attaches"]
        #[doc = " it to an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
        pub async fn add_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AddSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/AddSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret]."]
        pub async fn get_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates metadata of an existing [Secret][google.cloud.secretmanager.v1.Secret]."]
        pub async fn update_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecretRequest>,
        ) -> Result<tonic::Response<super::Secret>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/UpdateSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Secret][google.cloud.secretmanager.v1.Secret]."]
        pub async fn delete_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSecretRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/DeleteSecret",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This call does not return secret"]
        #[doc = " data."]
        pub async fn list_secret_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSecretVersionsRequest>,
        ) -> Result<tonic::Response<super::ListSecretVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/ListSecretVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata for a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        pub async fn get_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/GetSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]. This call returns the secret data."]
        #[doc = ""]
        #[doc = " `projects/*/secrets/*/versions/latest` is an alias to the `latest`"]
        #[doc = " [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        pub async fn access_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::AccessSecretVersionRequest>,
        ) -> Result<tonic::Response<super::AccessSecretVersionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/AccessSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED]."]
        pub async fn disable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/DisableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]."]
        pub async fn enable_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableSecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/EnableSecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]."]
        #[doc = ""]
        #[doc = " Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to"]
        #[doc = " [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED] and irrevocably destroys the"]
        #[doc = " secret data."]
        pub async fn destroy_secret_version(
            &mut self,
            request: impl tonic::IntoRequest<super::DestroySecretVersionRequest>,
        ) -> Result<tonic::Response<super::SecretVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/DestroySecretVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified secret. Replaces any"]
        #[doc = " existing policy."]
        #[doc = ""]
        #[doc = " Permissions on [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced according"]
        #[doc = " to the policy set on the associated [Secret][google.cloud.secretmanager.v1.Secret]."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a secret."]
        #[doc = " Returns empty policy if the secret exists and does not have a policy set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.secretmanager.v1.SecretManagerService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has for the specified secret."]
        #[doc = " If the secret does not exist, this call returns an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        #[doc = ""]
        #[doc = " Note: This operation is designed to be used for building permission-aware"]
        #[doc = " UIs and command-line tools, not for authorization checking. This operation"]
        #[doc = " may \"fail open\" without warning."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.secretmanager.v1.SecretManagerService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SecretManagerServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SecretManagerServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SecretManagerServiceClient {{ ... }}")
        }
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>GetFederationToken</code> request, including temporary Amazon Web Services credentials that can be used to make Amazon Web Services requests. </p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetFederationTokenOutput {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    #[doc(hidden)]
    pub credentials: std::option::Option<crate::types::Credentials>,
    /// <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy. </p>
    #[doc(hidden)]
    pub federated_user: std::option::Option<crate::types::FederatedUser>,
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    #[doc(hidden)]
    pub packed_policy_size: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl GetFederationTokenOutput {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn credentials(&self) -> std::option::Option<&crate::types::Credentials> {
        self.credentials.as_ref()
    }
    /// <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy. </p>
    pub fn federated_user(&self) -> std::option::Option<&crate::types::FederatedUser> {
        self.federated_user.as_ref()
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn packed_policy_size(&self) -> std::option::Option<i32> {
        self.packed_policy_size
    }
}
impl aws_http::request_id::RequestId for GetFederationTokenOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetFederationTokenOutput {
    /// Creates a new builder-style object to manufacture [`GetFederationTokenOutput`](crate::operation::get_federation_token::GetFederationTokenOutput).
    pub fn builder(
    ) -> crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder {
        crate::operation::get_federation_token::builders::GetFederationTokenOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_federation_token::GetFederationTokenOutput;
/// A builder for [`GetFederationTokenOutput`](crate::operation::get_federation_token::GetFederationTokenOutput).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct GetFederationTokenOutputBuilder {
    pub(crate) credentials: std::option::Option<crate::types::Credentials>,
    pub(crate) federated_user: std::option::Option<crate::types::FederatedUser>,
    pub(crate) packed_policy_size: std::option::Option<i32>,
    _request_id: Option<String>,
}
impl GetFederationTokenOutputBuilder {
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn credentials(mut self, input: crate::types::Credentials) -> Self {
        self.credentials = Some(input);
        self
    }
    /// <p>The temporary security credentials, which include an access key ID, a secret access key, and a security (or session) token.</p> <note>
    /// <p>The size of the security token that STS API operations return is not fixed. We strongly recommend that you make no assumptions about the maximum size.</p>
    /// </note>
    pub fn set_credentials(
        mut self,
        input: std::option::Option<crate::types::Credentials>,
    ) -> Self {
        self.credentials = input;
        self
    }
    /// <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy. </p>
    pub fn federated_user(mut self, input: crate::types::FederatedUser) -> Self {
        self.federated_user = Some(input);
        self
    }
    /// <p>Identifiers for the federated user associated with the credentials (such as <code>arn:aws:sts::123456789012:federated-user/Bob</code> or <code>123456789012:Bob</code>). You can use the federated user's ARN in your resource-based policies, such as an Amazon S3 bucket policy. </p>
    pub fn set_federated_user(
        mut self,
        input: std::option::Option<crate::types::FederatedUser>,
    ) -> Self {
        self.federated_user = input;
        self
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn packed_policy_size(mut self, input: i32) -> Self {
        self.packed_policy_size = Some(input);
        self
    }
    /// <p>A percentage value that indicates the packed size of the session policies and session tags combined passed in the request. The request fails if the packed size is greater than 100 percent, which means the policies and tags exceeded the allowed space.</p>
    pub fn set_packed_policy_size(mut self, input: std::option::Option<i32>) -> Self {
        self.packed_policy_size = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetFederationTokenOutput`](crate::operation::get_federation_token::GetFederationTokenOutput).
    pub fn build(self) -> crate::operation::get_federation_token::GetFederationTokenOutput {
        crate::operation::get_federation_token::GetFederationTokenOutput {
            credentials: self.credentials,
            federated_user: self.federated_user,
            packed_policy_size: self.packed_policy_size,
            _request_id: self._request_id,
        }
    }
}

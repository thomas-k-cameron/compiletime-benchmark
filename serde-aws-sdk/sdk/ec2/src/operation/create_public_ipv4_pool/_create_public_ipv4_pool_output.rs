// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct CreatePublicIpv4PoolOutput {
    /// <p>The ID of the public IPv4 pool.</p>
    #[doc(hidden)]
    pub pool_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreatePublicIpv4PoolOutput {
    /// <p>The ID of the public IPv4 pool.</p>
    pub fn pool_id(&self) -> std::option::Option<&str> {
        self.pool_id.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreatePublicIpv4PoolOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreatePublicIpv4PoolOutput {
    /// Creates a new builder-style object to manufacture [`CreatePublicIpv4PoolOutput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput).
    pub fn builder(
    ) -> crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolOutputBuilder
    {
        crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput;
/// A builder for [`CreatePublicIpv4PoolOutput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput).
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
pub struct CreatePublicIpv4PoolOutputBuilder {
    pub(crate) pool_id: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreatePublicIpv4PoolOutputBuilder {
    /// <p>The ID of the public IPv4 pool.</p>
    pub fn pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the public IPv4 pool.</p>
    pub fn set_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.pool_id = input;
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
    /// Consumes the builder and constructs a [`CreatePublicIpv4PoolOutput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput).
    pub fn build(self) -> crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput {
        crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput {
            pool_id: self.pool_id,
            _request_id: self._request_id,
        }
    }
}

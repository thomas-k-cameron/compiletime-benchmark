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
pub struct DescribePublicIpv4PoolsOutput {
    /// <p>Information about the address pools.</p>
    #[doc(hidden)]
    pub public_ipv4_pools: std::option::Option<std::vec::Vec<crate::types::PublicIpv4Pool>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribePublicIpv4PoolsOutput {
    /// <p>Information about the address pools.</p>
    pub fn public_ipv4_pools(&self) -> std::option::Option<&[crate::types::PublicIpv4Pool]> {
        self.public_ipv4_pools.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribePublicIpv4PoolsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribePublicIpv4PoolsOutput {
    /// Creates a new builder-style object to manufacture [`DescribePublicIpv4PoolsOutput`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput).
    pub fn builder(
    ) -> crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsOutputBuilder
    {
        crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput;
/// A builder for [`DescribePublicIpv4PoolsOutput`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput).
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
pub struct DescribePublicIpv4PoolsOutputBuilder {
    pub(crate) public_ipv4_pools: std::option::Option<std::vec::Vec<crate::types::PublicIpv4Pool>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribePublicIpv4PoolsOutputBuilder {
    /// Appends an item to `public_ipv4_pools`.
    ///
    /// To override the contents of this collection use [`set_public_ipv4_pools`](Self::set_public_ipv4_pools).
    ///
    /// <p>Information about the address pools.</p>
    pub fn public_ipv4_pools(mut self, input: crate::types::PublicIpv4Pool) -> Self {
        let mut v = self.public_ipv4_pools.unwrap_or_default();
        v.push(input);
        self.public_ipv4_pools = Some(v);
        self
    }
    /// <p>Information about the address pools.</p>
    pub fn set_public_ipv4_pools(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PublicIpv4Pool>>,
    ) -> Self {
        self.public_ipv4_pools = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribePublicIpv4PoolsOutput`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput {
        crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput {
            public_ipv4_pools: self.public_ipv4_pools,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
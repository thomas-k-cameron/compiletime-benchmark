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
pub struct DescribeIpamsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Information about the IPAMs.</p>
    #[doc(hidden)]
    pub ipams: std::option::Option<std::vec::Vec<crate::types::Ipam>>,
    _request_id: Option<String>,
}
impl DescribeIpamsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Information about the IPAMs.</p>
    pub fn ipams(&self) -> std::option::Option<&[crate::types::Ipam]> {
        self.ipams.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeIpamsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeIpamsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeIpamsOutput`](crate::operation::describe_ipams::DescribeIpamsOutput).
    pub fn builder() -> crate::operation::describe_ipams::builders::DescribeIpamsOutputBuilder {
        crate::operation::describe_ipams::builders::DescribeIpamsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_ipams::DescribeIpamsOutput;
/// A builder for [`DescribeIpamsOutput`](crate::operation::describe_ipams::DescribeIpamsOutput).
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
pub struct DescribeIpamsOutputBuilder {
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) ipams: std::option::Option<std::vec::Vec<crate::types::Ipam>>,
    _request_id: Option<String>,
}
impl DescribeIpamsOutputBuilder {
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
    /// Appends an item to `ipams`.
    ///
    /// To override the contents of this collection use [`set_ipams`](Self::set_ipams).
    ///
    /// <p>Information about the IPAMs.</p>
    pub fn ipams(mut self, input: crate::types::Ipam) -> Self {
        let mut v = self.ipams.unwrap_or_default();
        v.push(input);
        self.ipams = Some(v);
        self
    }
    /// <p>Information about the IPAMs.</p>
    pub fn set_ipams(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipam>>,
    ) -> Self {
        self.ipams = input;
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
    /// Consumes the builder and constructs a [`DescribeIpamsOutput`](crate::operation::describe_ipams::DescribeIpamsOutput).
    pub fn build(self) -> crate::operation::describe_ipams::DescribeIpamsOutput {
        crate::operation::describe_ipams::DescribeIpamsOutput {
            next_token: self.next_token,
            ipams: self.ipams,
            _request_id: self._request_id,
        }
    }
}

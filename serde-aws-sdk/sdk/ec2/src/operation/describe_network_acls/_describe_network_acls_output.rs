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
pub struct DescribeNetworkAclsOutput {
    /// <p>Information about one or more network ACLs.</p>
    #[doc(hidden)]
    pub network_acls: std::option::Option<std::vec::Vec<crate::types::NetworkAcl>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkAclsOutput {
    /// <p>Information about one or more network ACLs.</p>
    pub fn network_acls(&self) -> std::option::Option<&[crate::types::NetworkAcl]> {
        self.network_acls.as_deref()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeNetworkAclsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeNetworkAclsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeNetworkAclsOutput`](crate::operation::describe_network_acls::DescribeNetworkAclsOutput).
    pub fn builder(
    ) -> crate::operation::describe_network_acls::builders::DescribeNetworkAclsOutputBuilder {
        crate::operation::describe_network_acls::builders::DescribeNetworkAclsOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_network_acls::DescribeNetworkAclsOutput;
/// A builder for [`DescribeNetworkAclsOutput`](crate::operation::describe_network_acls::DescribeNetworkAclsOutput).
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
pub struct DescribeNetworkAclsOutputBuilder {
    pub(crate) network_acls: std::option::Option<std::vec::Vec<crate::types::NetworkAcl>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkAclsOutputBuilder {
    /// Appends an item to `network_acls`.
    ///
    /// To override the contents of this collection use [`set_network_acls`](Self::set_network_acls).
    ///
    /// <p>Information about one or more network ACLs.</p>
    pub fn network_acls(mut self, input: crate::types::NetworkAcl) -> Self {
        let mut v = self.network_acls.unwrap_or_default();
        v.push(input);
        self.network_acls = Some(v);
        self
    }
    /// <p>Information about one or more network ACLs.</p>
    pub fn set_network_acls(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::NetworkAcl>>,
    ) -> Self {
        self.network_acls = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeNetworkAclsOutput`](crate::operation::describe_network_acls::DescribeNetworkAclsOutput).
    pub fn build(self) -> crate::operation::describe_network_acls::DescribeNetworkAclsOutput {
        crate::operation::describe_network_acls::DescribeNetworkAclsOutput {
            network_acls: self.network_acls,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
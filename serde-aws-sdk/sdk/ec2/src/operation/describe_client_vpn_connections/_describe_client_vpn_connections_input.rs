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
pub struct DescribeClientVpnConnectionsInput {
    /// <p>The ID of the Client VPN endpoint.</p>
    #[doc(hidden)]
    pub client_vpn_endpoint_id: std::option::Option<std::string::String>,
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>connection-id</code> - The ID of the connection.</p> </li>
    /// <li> <p> <code>username</code> - For Active Directory client authentication, the user name of the client who established the client connection.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The token to retrieve the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeClientVpnConnectionsInput {
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn client_vpn_endpoint_id(&self) -> std::option::Option<&str> {
        self.client_vpn_endpoint_id.as_deref()
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>connection-id</code> - The ID of the connection.</p> </li>
    /// <li> <p> <code>username</code> - For Active Directory client authentication, the user name of the client who established the client connection.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeClientVpnConnectionsInput {
    /// Creates a new builder-style object to manufacture [`DescribeClientVpnConnectionsInput`](crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput).
    pub fn builder() -> crate::operation::describe_client_vpn_connections::builders::DescribeClientVpnConnectionsInputBuilder{
        crate::operation::describe_client_vpn_connections::builders::DescribeClientVpnConnectionsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput;
/// A builder for [`DescribeClientVpnConnectionsInput`](crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput).
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
pub struct DescribeClientVpnConnectionsInputBuilder {
    pub(crate) client_vpn_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeClientVpnConnectionsInputBuilder {
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_vpn_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn set_client_vpn_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.client_vpn_endpoint_id = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>connection-id</code> - The ID of the connection.</p> </li>
    /// <li> <p> <code>username</code> - For Active Directory client authentication, the user name of the client who established the client connection.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>connection-id</code> - The ID of the connection.</p> </li>
    /// <li> <p> <code>username</code> - For Active Directory client authentication, the user name of the client who established the client connection.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeClientVpnConnectionsInput`](crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsInput {
                client_vpn_endpoint_id: self.client_vpn_endpoint_id,
                filters: self.filters,
                next_token: self.next_token,
                max_results: self.max_results,
                dry_run: self.dry_run,
            },
        )
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_client_vpn_endpoints::_describe_client_vpn_endpoints_output::DescribeClientVpnEndpointsOutputBuilder;

pub use crate::operation::describe_client_vpn_endpoints::_describe_client_vpn_endpoints_input::DescribeClientVpnEndpointsInputBuilder;

/// Fluent builder constructing a request to `DescribeClientVpnEndpoints`.
///
/// <p>Describes one or more Client VPN endpoints in the account.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeClientVpnEndpointsFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsInputBuilder
            }
impl DescribeClientVpnEndpointsFluentBuilder {
    /// Creates a new `DescribeClientVpnEndpoints`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpoints,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_client_vpn_endpoints::DescribeClientVpnEndpointsError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_client_vpn_endpoints().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_client_vpn_endpoints::builders::DescribeClientVpnEndpointsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_client_vpn_endpoints::paginator::DescribeClientVpnEndpointsPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::describe_client_vpn_endpoints::paginator::DescribeClientVpnEndpointsPaginator{
        crate::operation::describe_client_vpn_endpoints::paginator::DescribeClientVpnEndpointsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ClientVpnEndpointIds`.
    ///
    /// To override the contents of this collection use [`set_client_vpn_endpoint_ids`](Self::set_client_vpn_endpoint_ids).
    ///
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn client_vpn_endpoint_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_vpn_endpoint_ids(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn set_client_vpn_endpoint_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_client_vpn_endpoint_ids(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the nextToken value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>endpoint-id</code> - The ID of the Client VPN endpoint.</p> </li>
    /// <li> <p> <code>transport-protocol</code> - The transport protocol (<code>tcp</code> | <code>udp</code>).</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    /// <ul>
    /// <li> <p> <code>endpoint-id</code> - The ID of the Client VPN endpoint.</p> </li>
    /// <li> <p> <code>transport-protocol</code> - The transport protocol (<code>tcp</code> | <code>udp</code>).</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
}
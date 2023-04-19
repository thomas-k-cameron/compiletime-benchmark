// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_transit_gateways::_describe_transit_gateways_output::DescribeTransitGatewaysOutputBuilder;

pub use crate::operation::describe_transit_gateways::_describe_transit_gateways_input::DescribeTransitGatewaysInputBuilder;

/// Fluent builder constructing a request to `DescribeTransitGateways`.
///
/// <p>Describes one or more transit gateways. By default, all transit gateways are described. Alternatively, you can filter the results.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DescribeTransitGatewaysFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::describe_transit_gateways::builders::DescribeTransitGatewaysInputBuilder,
}
impl DescribeTransitGatewaysFluentBuilder {
    /// Creates a new `DescribeTransitGateways`.
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
            crate::operation::describe_transit_gateways::DescribeTransitGateways,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_transit_gateways::DescribeTransitGatewaysError,
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
        crate::operation::describe_transit_gateways::DescribeTransitGatewaysOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::describe_transit_gateways::DescribeTransitGatewaysError,
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
    ///     let deserialized_parameters: crate::operation::describe_transit_gateways::builders::DescribeTransitGatewaysInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.describe_transit_gateways().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::describe_transit_gateways::builders::DescribeTransitGatewaysInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_transit_gateways::paginator::DescribeTransitGatewaysPaginator::send) which returns a `Stream`.
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_transit_gateways::paginator::DescribeTransitGatewaysPaginator
    {
        crate::operation::describe_transit_gateways::paginator::DescribeTransitGatewaysPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `TransitGatewayIds`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_ids`](Self::set_transit_gateway_ids).
    ///
    /// <p>The IDs of the transit gateways.</p>
    pub fn transit_gateway_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_ids(input.into());
        self
    }
    /// <p>The IDs of the transit gateways.</p>
    pub fn set_transit_gateway_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_ids(input);
        self
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>options.propagation-default-route-table-id</code> - The ID of the default propagation route table.</p> </li>
    /// <li> <p> <code>options.amazon-side-asn</code> - The private ASN for the Amazon side of a BGP session.</p> </li>
    /// <li> <p> <code>options.association-default-route-table-id</code> - The ID of the default association route table.</p> </li>
    /// <li> <p> <code>options.auto-accept-shared-attachments</code> - Indicates whether there is automatic acceptance of attachment requests (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.default-route-table-association</code> - Indicates whether resource attachments are automatically associated with the default association route table (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.default-route-table-propagation</code> - Indicates whether resource attachments automatically propagate routes to the default propagation route table (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.dns-support</code> - Indicates whether DNS support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.vpn-ecmp-support</code> - Indicates whether Equal Cost Multipath Protocol support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the transit gateway.</p> </li>
    /// <li> <p> <code>state</code> - The state of the transit gateway (<code>available</code> | <code>deleted</code> | <code>deleting</code> | <code>modifying</code> | <code>pending</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-id</code> - The ID of the transit gateway.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>options.propagation-default-route-table-id</code> - The ID of the default propagation route table.</p> </li>
    /// <li> <p> <code>options.amazon-side-asn</code> - The private ASN for the Amazon side of a BGP session.</p> </li>
    /// <li> <p> <code>options.association-default-route-table-id</code> - The ID of the default association route table.</p> </li>
    /// <li> <p> <code>options.auto-accept-shared-attachments</code> - Indicates whether there is automatic acceptance of attachment requests (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.default-route-table-association</code> - Indicates whether resource attachments are automatically associated with the default association route table (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.default-route-table-propagation</code> - Indicates whether resource attachments automatically propagate routes to the default propagation route table (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.dns-support</code> - Indicates whether DNS support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>options.vpn-ecmp-support</code> - Indicates whether Equal Cost Multipath Protocol support is enabled (<code>enable</code> | <code>disable</code>).</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the transit gateway.</p> </li>
    /// <li> <p> <code>state</code> - The state of the transit gateway (<code>available</code> | <code>deleted</code> | <code>deleting</code> | <code>modifying</code> | <code>pending</code>).</p> </li>
    /// <li> <p> <code>transit-gateway-id</code> - The ID of the transit gateway.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
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

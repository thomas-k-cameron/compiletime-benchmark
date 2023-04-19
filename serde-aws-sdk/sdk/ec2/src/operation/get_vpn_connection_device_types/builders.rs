// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_vpn_connection_device_types::_get_vpn_connection_device_types_output::GetVpnConnectionDeviceTypesOutputBuilder;

pub use crate::operation::get_vpn_connection_device_types::_get_vpn_connection_device_types_input::GetVpnConnectionDeviceTypesInputBuilder;

/// Fluent builder constructing a request to `GetVpnConnectionDeviceTypes`.
///
/// <p>Obtain a list of customer gateway devices for which sample configuration files can be provided. The request has no additional parameters. You can also see the list of device types with sample configuration files available under <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/your-cgw.html">Your customer gateway device</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct GetVpnConnectionDeviceTypesFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::get_vpn_connection_device_types::builders::GetVpnConnectionDeviceTypesInputBuilder
            }
impl GetVpnConnectionDeviceTypesFluentBuilder {
    /// Creates a new `GetVpnConnectionDeviceTypes`.
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
            crate::operation::get_vpn_connection_device_types::GetVpnConnectionDeviceTypes,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::get_vpn_connection_device_types::GetVpnConnectionDeviceTypesError,
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
        crate::operation::get_vpn_connection_device_types::GetVpnConnectionDeviceTypesOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::get_vpn_connection_device_types::GetVpnConnectionDeviceTypesError,
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
    ///     let deserialized_parameters: crate::operation::get_vpn_connection_device_types::builders::GetVpnConnectionDeviceTypesInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.get_vpn_connection_device_types().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::get_vpn_connection_device_types::builders::GetVpnConnectionDeviceTypesInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::get_vpn_connection_device_types::paginator::GetVpnConnectionDeviceTypesPaginator::send) which returns a `Stream`.
    pub fn into_paginator(self) -> crate::operation::get_vpn_connection_device_types::paginator::GetVpnConnectionDeviceTypesPaginator{
        crate::operation::get_vpn_connection_device_types::paginator::GetVpnConnectionDeviceTypesPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of results returned by <code>GetVpnConnectionDeviceTypes</code> in paginated output. When this parameter is used, <code>GetVpnConnectionDeviceTypes</code> only returns <code>MaxResults</code> results in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>GetVpnConnectionDeviceTypes</code> request with the returned <code>NextToken</code> value. This value can be between 200 and 1000. If this parameter is not used, then <code>GetVpnConnectionDeviceTypes</code> returns all results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>GetVpnConnectionDeviceTypes</code> in paginated output. When this parameter is used, <code>GetVpnConnectionDeviceTypes</code> only returns <code>MaxResults</code> results in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>GetVpnConnectionDeviceTypes</code> request with the returned <code>NextToken</code> value. This value can be between 200 and 1000. If this parameter is not used, then <code>GetVpnConnectionDeviceTypes</code> returns all results.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>GetVpnConnectionDeviceTypes</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is null when there are no more results to return. </p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>GetVpnConnectionDeviceTypes</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is null when there are no more results to return. </p>
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
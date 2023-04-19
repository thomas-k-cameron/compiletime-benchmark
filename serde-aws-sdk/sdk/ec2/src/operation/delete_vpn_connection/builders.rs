// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_vpn_connection::_delete_vpn_connection_output::DeleteVpnConnectionOutputBuilder;

pub use crate::operation::delete_vpn_connection::_delete_vpn_connection_input::DeleteVpnConnectionInputBuilder;

/// Fluent builder constructing a request to `DeleteVpnConnection`.
///
/// <p>Deletes the specified VPN connection.</p>
/// <p>If you're deleting the VPC and its associated components, we recommend that you detach the virtual private gateway from the VPC and delete the VPC before deleting the VPN connection. If you believe that the tunnel credentials for your VPN connection have been compromised, you can delete the VPN connection and create a new one that has new keys, without needing to delete the VPC or virtual private gateway. If you create a new VPN connection, you must reconfigure the customer gateway device using the new configuration information returned with the new VPN connection ID.</p>
/// <p>For certificate-based authentication, delete all Certificate Manager (ACM) private certificates used for the Amazon Web Services-side tunnel endpoints for the VPN connection before deleting the VPN connection.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteVpnConnectionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionInputBuilder,
}
impl DeleteVpnConnectionFluentBuilder {
    /// Creates a new `DeleteVpnConnection`.
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
            crate::operation::delete_vpn_connection::DeleteVpnConnection,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_vpn_connection::DeleteVpnConnectionError,
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
        crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::delete_vpn_connection::DeleteVpnConnectionError,
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
    ///     let deserialized_parameters: crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_vpn_connection().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn set_vpn_connection_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
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

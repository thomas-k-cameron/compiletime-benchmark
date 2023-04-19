// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpn_tunnel_options::_modify_vpn_tunnel_options_output::ModifyVpnTunnelOptionsOutputBuilder;

pub use crate::operation::modify_vpn_tunnel_options::_modify_vpn_tunnel_options_input::ModifyVpnTunnelOptionsInputBuilder;

/// Fluent builder constructing a request to `ModifyVpnTunnelOptions`.
///
/// <p>Modifies the options for a VPN tunnel in an Amazon Web Services Site-to-Site VPN connection. You can modify multiple options for a tunnel in a single request, but you can only modify one tunnel at a time. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/VPNTunnels.html">Site-to-Site VPN tunnel options for your Site-to-Site VPN connection</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct ModifyVpnTunnelOptionsFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsInputBuilder,
}
impl ModifyVpnTunnelOptionsFluentBuilder {
    /// Creates a new `ModifyVpnTunnelOptions`.
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
            crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptions,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsError,
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
        crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsError,
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
    ///     let deserialized_parameters: crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.modify_vpn_tunnel_options().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn set_vpn_connection_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.vpn_tunnel_outside_ip_address(input.into());
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn set_vpn_tunnel_outside_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_vpn_tunnel_outside_ip_address(input);
        self
    }
    /// <p>The tunnel options to modify.</p>
    pub fn tunnel_options(
        mut self,
        input: crate::types::ModifyVpnTunnelOptionsSpecification,
    ) -> Self {
        self.inner = self.inner.tunnel_options(input);
        self
    }
    /// <p>The tunnel options to modify.</p>
    pub fn set_tunnel_options(
        mut self,
        input: std::option::Option<crate::types::ModifyVpnTunnelOptionsSpecification>,
    ) -> Self {
        self.inner = self.inner.set_tunnel_options(input);
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

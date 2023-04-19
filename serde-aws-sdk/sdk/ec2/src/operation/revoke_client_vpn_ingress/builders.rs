// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_client_vpn_ingress::_revoke_client_vpn_ingress_output::RevokeClientVpnIngressOutputBuilder;

pub use crate::operation::revoke_client_vpn_ingress::_revoke_client_vpn_ingress_input::RevokeClientVpnIngressInputBuilder;

/// Fluent builder constructing a request to `RevokeClientVpnIngress`.
///
/// <p>Removes an ingress authorization rule from a Client VPN endpoint. </p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RevokeClientVpnIngressFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner:
        crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressInputBuilder,
}
impl RevokeClientVpnIngressFluentBuilder {
    /// Creates a new `RevokeClientVpnIngress`.
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
            crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngress,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError,
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
        crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError,
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
    ///     let deserialized_parameters: crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.revoke_client_vpn_ingress().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.client_vpn_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint with which the authorization rule is associated.</p>
    pub fn set_client_vpn_endpoint_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_client_vpn_endpoint_id(input);
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the network for which access is being removed.</p>
    pub fn target_network_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.target_network_cidr(input.into());
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the network for which access is being removed.</p>
    pub fn set_target_network_cidr(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_target_network_cidr(input);
        self
    }
    /// <p>The ID of the Active Directory group for which to revoke access. </p>
    pub fn access_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.access_group_id(input.into());
        self
    }
    /// <p>The ID of the Active Directory group for which to revoke access. </p>
    pub fn set_access_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_access_group_id(input);
        self
    }
    /// <p>Indicates whether access should be revoked for all clients.</p>
    pub fn revoke_all_groups(mut self, input: bool) -> Self {
        self.inner = self.inner.revoke_all_groups(input);
        self
    }
    /// <p>Indicates whether access should be revoked for all clients.</p>
    pub fn set_revoke_all_groups(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_revoke_all_groups(input);
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

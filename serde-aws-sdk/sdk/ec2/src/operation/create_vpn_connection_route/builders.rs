// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpn_connection_route::_create_vpn_connection_route_output::CreateVpnConnectionRouteOutputBuilder;

pub use crate::operation::create_vpn_connection_route::_create_vpn_connection_route_input::CreateVpnConnectionRouteInputBuilder;

/// Fluent builder constructing a request to `CreateVpnConnectionRoute`.
///
/// <p>Creates a static route associated with a VPN connection between an existing virtual private gateway and a VPN customer gateway. The static route allows traffic to be routed from the virtual private gateway to the VPN customer gateway.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/VPC_VPN.html">Amazon Web Services Site-to-Site VPN</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateVpnConnectionRouteFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_vpn_connection_route::builders::CreateVpnConnectionRouteInputBuilder
            }
impl CreateVpnConnectionRouteFluentBuilder {
    /// Creates a new `CreateVpnConnectionRoute`.
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
            crate::operation::create_vpn_connection_route::CreateVpnConnectionRoute,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteError,
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
        crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::create_vpn_connection_route::CreateVpnConnectionRouteError,
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
    ///     let deserialized_parameters: crate::operation::create_vpn_connection_route::builders::CreateVpnConnectionRouteInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_vpn_connection_route().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_vpn_connection_route::builders::CreateVpnConnectionRouteInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.destination_cidr_block(input.into());
        self
    }
    /// <p>The CIDR block associated with the local subnet of the customer network.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_destination_cidr_block(input);
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
}

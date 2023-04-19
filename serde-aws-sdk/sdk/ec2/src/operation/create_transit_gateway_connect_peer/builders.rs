// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_transit_gateway_connect_peer::_create_transit_gateway_connect_peer_output::CreateTransitGatewayConnectPeerOutputBuilder;

pub use crate::operation::create_transit_gateway_connect_peer::_create_transit_gateway_connect_peer_input::CreateTransitGatewayConnectPeerInputBuilder;

/// Fluent builder constructing a request to `CreateTransitGatewayConnectPeer`.
///
/// <p>Creates a Connect peer for a specified transit gateway Connect attachment between a transit gateway and an appliance.</p>
/// <p>The peer address and transit gateway address must be the same IP address family (IPv4 or IPv6).</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-connect.html#tgw-connect-peer">Connect peers</a> in the <i>Transit Gateways Guide</i>.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct CreateTransitGatewayConnectPeerFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::create_transit_gateway_connect_peer::builders::CreateTransitGatewayConnectPeerInputBuilder
            }
impl CreateTransitGatewayConnectPeerFluentBuilder {
    /// Creates a new `CreateTransitGatewayConnectPeer`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
                    pub async fn customize(self) -> std::result::Result<
                        crate::client::customize::CustomizableOperation<crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeer, aws_http::retry::AwsResponseRetryClassifier,>,
                        aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerError>
    >{
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
                    pub async fn send(self) -> std::result::Result<crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerOutput, aws_smithy_http::result::SdkError<crate::operation::create_transit_gateway_connect_peer::CreateTransitGatewayConnectPeerError>>
                     {
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
    ///     let deserialized_parameters: crate::operation::create_transit_gateway_connect_peer::builders::CreateTransitGatewayConnectPeerInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.create_transit_gateway_connect_peer().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::create_transit_gateway_connect_peer::builders::CreateTransitGatewayConnectPeerInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The ID of the Connect attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the Connect attachment.</p>
    pub fn set_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    pub fn transit_gateway_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_address(input.into());
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the transit gateway side of the Connect peer, which must be specified from a transit gateway CIDR block. If not specified, Amazon automatically assigns the first available IP address from the transit gateway CIDR block.</p>
    pub fn set_transit_gateway_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_transit_gateway_address(input);
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    pub fn peer_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.peer_address(input.into());
        self
    }
    /// <p>The peer IP address (GRE outer IP address) on the appliance side of the Connect peer.</p>
    pub fn set_peer_address(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_peer_address(input);
        self
    }
    /// <p>The BGP options for the Connect peer.</p>
    pub fn bgp_options(
        mut self,
        input: crate::types::TransitGatewayConnectRequestBgpOptions,
    ) -> Self {
        self.inner = self.inner.bgp_options(input);
        self
    }
    /// <p>The BGP options for the Connect peer.</p>
    pub fn set_bgp_options(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayConnectRequestBgpOptions>,
    ) -> Self {
        self.inner = self.inner.set_bgp_options(input);
        self
    }
    /// Appends an item to `InsideCidrBlocks`.
    ///
    /// To override the contents of this collection use [`set_inside_cidr_blocks`](Self::set_inside_cidr_blocks).
    ///
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    pub fn inside_cidr_blocks(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.inside_cidr_blocks(input.into());
        self
    }
    /// <p>The range of inside IP addresses that are used for BGP peering. You must specify a size /29 IPv4 CIDR block from the <code>169.254.0.0/16</code> range. The first address from the range must be configured on the appliance as the BGP IP address. You can also optionally specify a size /125 IPv6 CIDR block from the <code>fd00::/8</code> range.</p>
    pub fn set_inside_cidr_blocks(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_inside_cidr_blocks(input);
        self
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Connect peer.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Connect peer.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
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

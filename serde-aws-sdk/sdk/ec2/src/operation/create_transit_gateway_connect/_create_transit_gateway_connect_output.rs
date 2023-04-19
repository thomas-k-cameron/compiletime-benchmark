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
pub struct CreateTransitGatewayConnectOutput {
    /// <p>Information about the Connect attachment.</p>
    #[doc(hidden)]
    pub transit_gateway_connect: std::option::Option<crate::types::TransitGatewayConnect>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayConnectOutput {
    /// <p>Information about the Connect attachment.</p>
    pub fn transit_gateway_connect(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayConnect> {
        self.transit_gateway_connect.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateTransitGatewayConnectOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayConnectOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayConnectOutput`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput).
    pub fn builder() -> crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectOutputBuilder{
        crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput;
/// A builder for [`CreateTransitGatewayConnectOutput`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput).
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
pub struct CreateTransitGatewayConnectOutputBuilder {
    pub(crate) transit_gateway_connect: std::option::Option<crate::types::TransitGatewayConnect>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayConnectOutputBuilder {
    /// <p>Information about the Connect attachment.</p>
    pub fn transit_gateway_connect(mut self, input: crate::types::TransitGatewayConnect) -> Self {
        self.transit_gateway_connect = Some(input);
        self
    }
    /// <p>Information about the Connect attachment.</p>
    pub fn set_transit_gateway_connect(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayConnect>,
    ) -> Self {
        self.transit_gateway_connect = input;
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
    /// Consumes the builder and constructs a [`CreateTransitGatewayConnectOutput`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput).
    pub fn build(
        self,
    ) -> crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput {
        crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput {
            transit_gateway_connect: self.transit_gateway_connect,
            _request_id: self._request_id,
        }
    }
}

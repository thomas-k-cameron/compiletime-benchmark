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
pub struct CreateTransitGatewayOutput {
    /// <p>Information about the transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway: std::option::Option<crate::types::TransitGateway>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayOutput {
    /// <p>Information about the transit gateway.</p>
    pub fn transit_gateway(&self) -> std::option::Option<&crate::types::TransitGateway> {
        self.transit_gateway.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateTransitGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayOutput`](crate::operation::create_transit_gateway::CreateTransitGatewayOutput).
    pub fn builder(
    ) -> crate::operation::create_transit_gateway::builders::CreateTransitGatewayOutputBuilder {
        crate::operation::create_transit_gateway::builders::CreateTransitGatewayOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_transit_gateway::CreateTransitGatewayOutput;
/// A builder for [`CreateTransitGatewayOutput`](crate::operation::create_transit_gateway::CreateTransitGatewayOutput).
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
pub struct CreateTransitGatewayOutputBuilder {
    pub(crate) transit_gateway: std::option::Option<crate::types::TransitGateway>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayOutputBuilder {
    /// <p>Information about the transit gateway.</p>
    pub fn transit_gateway(mut self, input: crate::types::TransitGateway) -> Self {
        self.transit_gateway = Some(input);
        self
    }
    /// <p>Information about the transit gateway.</p>
    pub fn set_transit_gateway(
        mut self,
        input: std::option::Option<crate::types::TransitGateway>,
    ) -> Self {
        self.transit_gateway = input;
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
    /// Consumes the builder and constructs a [`CreateTransitGatewayOutput`](crate::operation::create_transit_gateway::CreateTransitGatewayOutput).
    pub fn build(self) -> crate::operation::create_transit_gateway::CreateTransitGatewayOutput {
        crate::operation::create_transit_gateway::CreateTransitGatewayOutput {
            transit_gateway: self.transit_gateway,
            _request_id: self._request_id,
        }
    }
}
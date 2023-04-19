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
pub struct DeleteTransitGatewayRouteTableOutput {
    /// <p>Information about the deleted transit gateway route table.</p>
    #[doc(hidden)]
    pub transit_gateway_route_table: std::option::Option<crate::types::TransitGatewayRouteTable>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayRouteTableOutput {
    /// <p>Information about the deleted transit gateway route table.</p>
    pub fn transit_gateway_route_table(
        &self,
    ) -> std::option::Option<&crate::types::TransitGatewayRouteTable> {
        self.transit_gateway_route_table.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteTransitGatewayRouteTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTransitGatewayRouteTableOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayRouteTableOutput`](crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput).
    pub fn builder() -> crate::operation::delete_transit_gateway_route_table::builders::DeleteTransitGatewayRouteTableOutputBuilder{
        crate::operation::delete_transit_gateway_route_table::builders::DeleteTransitGatewayRouteTableOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput;
/// A builder for [`DeleteTransitGatewayRouteTableOutput`](crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput).
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
pub struct DeleteTransitGatewayRouteTableOutputBuilder {
    pub(crate) transit_gateway_route_table:
        std::option::Option<crate::types::TransitGatewayRouteTable>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayRouteTableOutputBuilder {
    /// <p>Information about the deleted transit gateway route table.</p>
    pub fn transit_gateway_route_table(
        mut self,
        input: crate::types::TransitGatewayRouteTable,
    ) -> Self {
        self.transit_gateway_route_table = Some(input);
        self
    }
    /// <p>Information about the deleted transit gateway route table.</p>
    pub fn set_transit_gateway_route_table(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayRouteTable>,
    ) -> Self {
        self.transit_gateway_route_table = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayRouteTableOutput`](crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput
    {
        crate::operation::delete_transit_gateway_route_table::DeleteTransitGatewayRouteTableOutput {
            transit_gateway_route_table: self.transit_gateway_route_table,
            _request_id: self._request_id,
        }
    }
}

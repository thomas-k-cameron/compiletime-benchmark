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
pub struct DisassociateTransitGatewayRouteTableOutput {
    /// <p>Information about the association.</p>
    #[doc(hidden)]
    pub association: std::option::Option<crate::types::TransitGatewayAssociation>,
    _request_id: Option<String>,
}
impl DisassociateTransitGatewayRouteTableOutput {
    /// <p>Information about the association.</p>
    pub fn association(&self) -> std::option::Option<&crate::types::TransitGatewayAssociation> {
        self.association.as_ref()
    }
}
impl aws_http::request_id::RequestId for DisassociateTransitGatewayRouteTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateTransitGatewayRouteTableOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateTransitGatewayRouteTableOutput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput).
    pub fn builder() -> crate::operation::disassociate_transit_gateway_route_table::builders::DisassociateTransitGatewayRouteTableOutputBuilder{
        crate::operation::disassociate_transit_gateway_route_table::builders::DisassociateTransitGatewayRouteTableOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput;
/// A builder for [`DisassociateTransitGatewayRouteTableOutput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput).
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
pub struct DisassociateTransitGatewayRouteTableOutputBuilder {
    pub(crate) association: std::option::Option<crate::types::TransitGatewayAssociation>,
    _request_id: Option<String>,
}
impl DisassociateTransitGatewayRouteTableOutputBuilder {
    /// <p>Information about the association.</p>
    pub fn association(mut self, input: crate::types::TransitGatewayAssociation) -> Self {
        self.association = Some(input);
        self
    }
    /// <p>Information about the association.</p>
    pub fn set_association(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayAssociation>,
    ) -> Self {
        self.association = input;
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
    /// Consumes the builder and constructs a [`DisassociateTransitGatewayRouteTableOutput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput).
    pub fn build(self) -> crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput{
        crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableOutput {
            association: self.association
            ,
            _request_id: self._request_id,
        }
    }
}

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
pub struct DeleteTransitGatewayRouteOutput {
    /// <p>Information about the route.</p>
    #[doc(hidden)]
    pub route: std::option::Option<crate::types::TransitGatewayRoute>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayRouteOutput {
    /// <p>Information about the route.</p>
    pub fn route(&self) -> std::option::Option<&crate::types::TransitGatewayRoute> {
        self.route.as_ref()
    }
}
impl aws_http::request_id::RequestId for DeleteTransitGatewayRouteOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTransitGatewayRouteOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTransitGatewayRouteOutput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput).
    pub fn builder() -> crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteOutputBuilder{
        crate::operation::delete_transit_gateway_route::builders::DeleteTransitGatewayRouteOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput;
/// A builder for [`DeleteTransitGatewayRouteOutput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput).
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
pub struct DeleteTransitGatewayRouteOutputBuilder {
    pub(crate) route: std::option::Option<crate::types::TransitGatewayRoute>,
    _request_id: Option<String>,
}
impl DeleteTransitGatewayRouteOutputBuilder {
    /// <p>Information about the route.</p>
    pub fn route(mut self, input: crate::types::TransitGatewayRoute) -> Self {
        self.route = Some(input);
        self
    }
    /// <p>Information about the route.</p>
    pub fn set_route(
        mut self,
        input: std::option::Option<crate::types::TransitGatewayRoute>,
    ) -> Self {
        self.route = input;
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
    /// Consumes the builder and constructs a [`DeleteTransitGatewayRouteOutput`](crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput).
    pub fn build(
        self,
    ) -> crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput {
        crate::operation::delete_transit_gateway_route::DeleteTransitGatewayRouteOutput {
            route: self.route,
            _request_id: self._request_id,
        }
    }
}

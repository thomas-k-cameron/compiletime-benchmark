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
pub struct CreateRouteTableOutput {
    /// <p>Information about the route table.</p>
    #[doc(hidden)]
    pub route_table: std::option::Option<crate::types::RouteTable>,
    _request_id: Option<String>,
}
impl CreateRouteTableOutput {
    /// <p>Information about the route table.</p>
    pub fn route_table(&self) -> std::option::Option<&crate::types::RouteTable> {
        self.route_table.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateRouteTableOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateRouteTableOutput {
    /// Creates a new builder-style object to manufacture [`CreateRouteTableOutput`](crate::operation::create_route_table::CreateRouteTableOutput).
    pub fn builder() -> crate::operation::create_route_table::builders::CreateRouteTableOutputBuilder
    {
        crate::operation::create_route_table::builders::CreateRouteTableOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_route_table::CreateRouteTableOutput;
/// A builder for [`CreateRouteTableOutput`](crate::operation::create_route_table::CreateRouteTableOutput).
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
pub struct CreateRouteTableOutputBuilder {
    pub(crate) route_table: std::option::Option<crate::types::RouteTable>,
    _request_id: Option<String>,
}
impl CreateRouteTableOutputBuilder {
    /// <p>Information about the route table.</p>
    pub fn route_table(mut self, input: crate::types::RouteTable) -> Self {
        self.route_table = Some(input);
        self
    }
    /// <p>Information about the route table.</p>
    pub fn set_route_table(mut self, input: std::option::Option<crate::types::RouteTable>) -> Self {
        self.route_table = input;
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
    /// Consumes the builder and constructs a [`CreateRouteTableOutput`](crate::operation::create_route_table::CreateRouteTableOutput).
    pub fn build(self) -> crate::operation::create_route_table::CreateRouteTableOutput {
        crate::operation::create_route_table::CreateRouteTableOutput {
            route_table: self.route_table,
            _request_id: self._request_id,
        }
    }
}

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
pub struct DescribeLocalGatewayRouteTableVpcAssociationsOutput {
    /// <p>Information about the associations.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_vpc_associations:
        std::option::Option<std::vec::Vec<crate::types::LocalGatewayRouteTableVpcAssociation>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewayRouteTableVpcAssociationsOutput {
    /// <p>Information about the associations.</p>
    pub fn local_gateway_route_table_vpc_associations(
        &self,
    ) -> std::option::Option<&[crate::types::LocalGatewayRouteTableVpcAssociation]> {
        self.local_gateway_route_table_vpc_associations.as_deref()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for DescribeLocalGatewayRouteTableVpcAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLocalGatewayRouteTableVpcAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLocalGatewayRouteTableVpcAssociationsOutput`](crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput).
    pub fn builder() -> crate::operation::describe_local_gateway_route_table_vpc_associations::builders::DescribeLocalGatewayRouteTableVpcAssociationsOutputBuilder{
        crate::operation::describe_local_gateway_route_table_vpc_associations::builders::DescribeLocalGatewayRouteTableVpcAssociationsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput;
/// A builder for [`DescribeLocalGatewayRouteTableVpcAssociationsOutput`](crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput).
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
pub struct DescribeLocalGatewayRouteTableVpcAssociationsOutputBuilder {
    pub(crate) local_gateway_route_table_vpc_associations:
        std::option::Option<std::vec::Vec<crate::types::LocalGatewayRouteTableVpcAssociation>>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewayRouteTableVpcAssociationsOutputBuilder {
    /// Appends an item to `local_gateway_route_table_vpc_associations`.
    ///
    /// To override the contents of this collection use [`set_local_gateway_route_table_vpc_associations`](Self::set_local_gateway_route_table_vpc_associations).
    ///
    /// <p>Information about the associations.</p>
    pub fn local_gateway_route_table_vpc_associations(
        mut self,
        input: crate::types::LocalGatewayRouteTableVpcAssociation,
    ) -> Self {
        let mut v = self
            .local_gateway_route_table_vpc_associations
            .unwrap_or_default();
        v.push(input);
        self.local_gateway_route_table_vpc_associations = Some(v);
        self
    }
    /// <p>Information about the associations.</p>
    pub fn set_local_gateway_route_table_vpc_associations(
        mut self,
        input: std::option::Option<
            std::vec::Vec<crate::types::LocalGatewayRouteTableVpcAssociation>,
        >,
    ) -> Self {
        self.local_gateway_route_table_vpc_associations = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
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
    /// Consumes the builder and constructs a [`DescribeLocalGatewayRouteTableVpcAssociationsOutput`](crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput).
    pub fn build(self) -> crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput{
        crate::operation::describe_local_gateway_route_table_vpc_associations::DescribeLocalGatewayRouteTableVpcAssociationsOutput {
            local_gateway_route_table_vpc_associations: self.local_gateway_route_table_vpc_associations
            ,
            next_token: self.next_token
            ,
            _request_id: self._request_id,
        }
    }
}
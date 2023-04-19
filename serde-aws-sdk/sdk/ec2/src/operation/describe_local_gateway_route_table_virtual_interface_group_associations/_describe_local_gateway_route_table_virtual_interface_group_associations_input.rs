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
pub struct DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput {
    /// <p>The IDs of the associations.</p>
    #[doc(hidden)]
    pub local_gateway_route_table_virtual_interface_group_association_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-association-id</code> - The ID of the association.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-id</code> - The ID of the virtual interface group.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p> </li>
    /// <li> <p> <code>state</code> - The state of the association.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    #[doc(hidden)]
    pub max_results: std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
}
impl DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput {
    /// <p>The IDs of the associations.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association_ids(
        &self,
    ) -> std::option::Option<&[std::string::String]> {
        self.local_gateway_route_table_virtual_interface_group_association_ids
            .as_deref()
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-association-id</code> - The ID of the association.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-id</code> - The ID of the virtual interface group.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p> </li>
    /// <li> <p> <code>state</code> - The state of the association.</p> </li>
    /// </ul>
    pub fn filters(&self) -> std::option::Option<&[crate::types::Filter]> {
        self.filters.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput {
    /// Creates a new builder-style object to manufacture [`DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput`](crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput).
    pub fn builder() -> crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::builders::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder{
        crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::builders::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput;
/// A builder for [`DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput`](crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput).
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
pub struct DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder {
    pub(crate) local_gateway_route_table_virtual_interface_group_association_ids:
        std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) filters: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: std::option::Option<i32>,
    pub(crate) next_token: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
}
impl DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInputBuilder {
    /// Appends an item to `local_gateway_route_table_virtual_interface_group_association_ids`.
    ///
    /// To override the contents of this collection use [`set_local_gateway_route_table_virtual_interface_group_association_ids`](Self::set_local_gateway_route_table_virtual_interface_group_association_ids).
    ///
    /// <p>The IDs of the associations.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association_ids(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        let mut v = self
            .local_gateway_route_table_virtual_interface_group_association_ids
            .unwrap_or_default();
        v.push(input.into());
        self.local_gateway_route_table_virtual_interface_group_association_ids = Some(v);
        self
    }
    /// <p>The IDs of the associations.</p>
    pub fn set_local_gateway_route_table_virtual_interface_group_association_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.local_gateway_route_table_virtual_interface_group_association_ids = input;
        self
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-association-id</code> - The ID of the association.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-id</code> - The ID of the virtual interface group.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p> </li>
    /// <li> <p> <code>state</code> - The state of the association.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = Some(v);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table for the virtual interface group.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-id</code> - The ID of the local gateway route table.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-association-id</code> - The ID of the association.</p> </li>
    /// <li> <p> <code>local-gateway-route-table-virtual-interface-group-id</code> - The ID of the virtual interface group.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway virtual interface group association.</p> </li>
    /// <li> <p> <code>state</code> - The state of the association.</p> </li>
    /// </ul>
    pub fn set_filters(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Filter>>,
    ) -> Self {
        self.filters = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.next_token = Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// Consumes the builder and constructs a [`DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput`](crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput).
    pub fn build(self) -> Result<crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::describe_local_gateway_route_table_virtual_interface_group_associations::DescribeLocalGatewayRouteTableVirtualInterfaceGroupAssociationsInput {
                local_gateway_route_table_virtual_interface_group_association_ids: self.local_gateway_route_table_virtual_interface_group_association_ids
                ,
                filters: self.filters
                ,
                max_results: self.max_results
                ,
                next_token: self.next_token
                ,
                dry_run: self.dry_run
                ,
            }
        )
    }
}

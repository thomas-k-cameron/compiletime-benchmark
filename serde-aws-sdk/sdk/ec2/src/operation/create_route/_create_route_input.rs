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
pub struct CreateRouteInput {
    /// <p>The IPv4 CIDR address block used for the destination match. Routing decisions are based on the most specific match. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    #[doc(hidden)]
    pub destination_cidr_block: std::option::Option<std::string::String>,
    /// <p>The IPv6 CIDR block used for the destination match. Routing decisions are based on the most specific match.</p>
    #[doc(hidden)]
    pub destination_ipv6_cidr_block: std::option::Option<std::string::String>,
    /// <p>The ID of a prefix list used for the destination match.</p>
    #[doc(hidden)]
    pub destination_prefix_list_id: std::option::Option<std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    #[doc(hidden)]
    pub dry_run: std::option::Option<bool>,
    /// <p>The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.</p>
    #[doc(hidden)]
    pub vpc_endpoint_id: std::option::Option<std::string::String>,
    /// <p>[IPv6 traffic only] The ID of an egress-only internet gateway.</p>
    #[doc(hidden)]
    pub egress_only_internet_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of an internet gateway or virtual private gateway attached to your VPC.</p>
    #[doc(hidden)]
    pub gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of a NAT instance in your VPC. The operation fails if you specify an instance ID unless exactly one network interface is attached.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>[IPv4 traffic only] The ID of a NAT gateway.</p>
    #[doc(hidden)]
    pub nat_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of a transit gateway.</p>
    #[doc(hidden)]
    pub transit_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of the local gateway.</p>
    #[doc(hidden)]
    pub local_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of the carrier gateway.</p>
    /// <p>You can only use this option when the VPC contains a subnet which is associated with a Wavelength Zone.</p>
    #[doc(hidden)]
    pub carrier_gateway_id: std::option::Option<std::string::String>,
    /// <p>The ID of a network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The ID of the route table for the route.</p>
    #[doc(hidden)]
    pub route_table_id: std::option::Option<std::string::String>,
    /// <p>The ID of a VPC peering connection.</p>
    #[doc(hidden)]
    pub vpc_peering_connection_id: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the core network.</p>
    #[doc(hidden)]
    pub core_network_arn: std::option::Option<std::string::String>,
}
impl CreateRouteInput {
    /// <p>The IPv4 CIDR address block used for the destination match. Routing decisions are based on the most specific match. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    pub fn destination_cidr_block(&self) -> std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The IPv6 CIDR block used for the destination match. Routing decisions are based on the most specific match.</p>
    pub fn destination_ipv6_cidr_block(&self) -> std::option::Option<&str> {
        self.destination_ipv6_cidr_block.as_deref()
    }
    /// <p>The ID of a prefix list used for the destination match.</p>
    pub fn destination_prefix_list_id(&self) -> std::option::Option<&str> {
        self.destination_prefix_list_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.</p>
    pub fn vpc_endpoint_id(&self) -> std::option::Option<&str> {
        self.vpc_endpoint_id.as_deref()
    }
    /// <p>[IPv6 traffic only] The ID of an egress-only internet gateway.</p>
    pub fn egress_only_internet_gateway_id(&self) -> std::option::Option<&str> {
        self.egress_only_internet_gateway_id.as_deref()
    }
    /// <p>The ID of an internet gateway or virtual private gateway attached to your VPC.</p>
    pub fn gateway_id(&self) -> std::option::Option<&str> {
        self.gateway_id.as_deref()
    }
    /// <p>The ID of a NAT instance in your VPC. The operation fails if you specify an instance ID unless exactly one network interface is attached.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>[IPv4 traffic only] The ID of a NAT gateway.</p>
    pub fn nat_gateway_id(&self) -> std::option::Option<&str> {
        self.nat_gateway_id.as_deref()
    }
    /// <p>The ID of a transit gateway.</p>
    pub fn transit_gateway_id(&self) -> std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(&self) -> std::option::Option<&str> {
        self.local_gateway_id.as_deref()
    }
    /// <p>The ID of the carrier gateway.</p>
    /// <p>You can only use this option when the VPC contains a subnet which is associated with a Wavelength Zone.</p>
    pub fn carrier_gateway_id(&self) -> std::option::Option<&str> {
        self.carrier_gateway_id.as_deref()
    }
    /// <p>The ID of a network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The ID of the route table for the route.</p>
    pub fn route_table_id(&self) -> std::option::Option<&str> {
        self.route_table_id.as_deref()
    }
    /// <p>The ID of a VPC peering connection.</p>
    pub fn vpc_peering_connection_id(&self) -> std::option::Option<&str> {
        self.vpc_peering_connection_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the core network.</p>
    pub fn core_network_arn(&self) -> std::option::Option<&str> {
        self.core_network_arn.as_deref()
    }
}
impl CreateRouteInput {
    /// Creates a new builder-style object to manufacture [`CreateRouteInput`](crate::operation::create_route::CreateRouteInput).
    pub fn builder() -> crate::operation::create_route::builders::CreateRouteInputBuilder {
        crate::operation::create_route::builders::CreateRouteInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_route::CreateRouteInput;
/// A builder for [`CreateRouteInput`](crate::operation::create_route::CreateRouteInput).
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
pub struct CreateRouteInputBuilder {
    pub(crate) destination_cidr_block: std::option::Option<std::string::String>,
    pub(crate) destination_ipv6_cidr_block: std::option::Option<std::string::String>,
    pub(crate) destination_prefix_list_id: std::option::Option<std::string::String>,
    pub(crate) dry_run: std::option::Option<bool>,
    pub(crate) vpc_endpoint_id: std::option::Option<std::string::String>,
    pub(crate) egress_only_internet_gateway_id: std::option::Option<std::string::String>,
    pub(crate) gateway_id: std::option::Option<std::string::String>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) nat_gateway_id: std::option::Option<std::string::String>,
    pub(crate) transit_gateway_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_id: std::option::Option<std::string::String>,
    pub(crate) carrier_gateway_id: std::option::Option<std::string::String>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) route_table_id: std::option::Option<std::string::String>,
    pub(crate) vpc_peering_connection_id: std::option::Option<std::string::String>,
    pub(crate) core_network_arn: std::option::Option<std::string::String>,
}
impl CreateRouteInputBuilder {
    /// <p>The IPv4 CIDR address block used for the destination match. Routing decisions are based on the most specific match. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    pub fn destination_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_cidr_block = Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR address block used for the destination match. Routing decisions are based on the most specific match. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    pub fn set_destination_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The IPv6 CIDR block used for the destination match. Routing decisions are based on the most specific match.</p>
    pub fn destination_ipv6_cidr_block(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_ipv6_cidr_block = Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR block used for the destination match. Routing decisions are based on the most specific match.</p>
    pub fn set_destination_ipv6_cidr_block(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_ipv6_cidr_block = input;
        self
    }
    /// <p>The ID of a prefix list used for the destination match.</p>
    pub fn destination_prefix_list_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.destination_prefix_list_id = Some(input.into());
        self
    }
    /// <p>The ID of a prefix list used for the destination match.</p>
    pub fn set_destination_prefix_list_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.destination_prefix_list_id = input;
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
    /// <p>The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.</p>
    pub fn vpc_endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_endpoint_id = Some(input.into());
        self
    }
    /// <p>The ID of a VPC endpoint. Supported for Gateway Load Balancer endpoints only.</p>
    pub fn set_vpc_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.vpc_endpoint_id = input;
        self
    }
    /// <p>[IPv6 traffic only] The ID of an egress-only internet gateway.</p>
    pub fn egress_only_internet_gateway_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.egress_only_internet_gateway_id = Some(input.into());
        self
    }
    /// <p>[IPv6 traffic only] The ID of an egress-only internet gateway.</p>
    pub fn set_egress_only_internet_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.egress_only_internet_gateway_id = input;
        self
    }
    /// <p>The ID of an internet gateway or virtual private gateway attached to your VPC.</p>
    pub fn gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of an internet gateway or virtual private gateway attached to your VPC.</p>
    pub fn set_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.gateway_id = input;
        self
    }
    /// <p>The ID of a NAT instance in your VPC. The operation fails if you specify an instance ID unless exactly one network interface is attached.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of a NAT instance in your VPC. The operation fails if you specify an instance ID unless exactly one network interface is attached.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>[IPv4 traffic only] The ID of a NAT gateway.</p>
    pub fn nat_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.nat_gateway_id = Some(input.into());
        self
    }
    /// <p>[IPv4 traffic only] The ID of a NAT gateway.</p>
    pub fn set_nat_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.nat_gateway_id = input;
        self
    }
    /// <p>The ID of a transit gateway.</p>
    pub fn transit_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.transit_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of a transit gateway.</p>
    pub fn set_transit_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn set_local_gateway_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.local_gateway_id = input;
        self
    }
    /// <p>The ID of the carrier gateway.</p>
    /// <p>You can only use this option when the VPC contains a subnet which is associated with a Wavelength Zone.</p>
    pub fn carrier_gateway_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.carrier_gateway_id = Some(input.into());
        self
    }
    /// <p>The ID of the carrier gateway.</p>
    /// <p>You can only use this option when the VPC contains a subnet which is associated with a Wavelength Zone.</p>
    pub fn set_carrier_gateway_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.carrier_gateway_id = input;
        self
    }
    /// <p>The ID of a network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of a network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The ID of the route table for the route.</p>
    pub fn route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.route_table_id = Some(input.into());
        self
    }
    /// <p>The ID of the route table for the route.</p>
    pub fn set_route_table_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.route_table_id = input;
        self
    }
    /// <p>The ID of a VPC peering connection.</p>
    pub fn vpc_peering_connection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.vpc_peering_connection_id = Some(input.into());
        self
    }
    /// <p>The ID of a VPC peering connection.</p>
    pub fn set_vpc_peering_connection_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.vpc_peering_connection_id = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the core network.</p>
    pub fn core_network_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.core_network_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the core network.</p>
    pub fn set_core_network_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.core_network_arn = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateRouteInput`](crate::operation::create_route::CreateRouteInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_route::CreateRouteInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_route::CreateRouteInput {
            destination_cidr_block: self.destination_cidr_block,
            destination_ipv6_cidr_block: self.destination_ipv6_cidr_block,
            destination_prefix_list_id: self.destination_prefix_list_id,
            dry_run: self.dry_run,
            vpc_endpoint_id: self.vpc_endpoint_id,
            egress_only_internet_gateway_id: self.egress_only_internet_gateway_id,
            gateway_id: self.gateway_id,
            instance_id: self.instance_id,
            nat_gateway_id: self.nat_gateway_id,
            transit_gateway_id: self.transit_gateway_id,
            local_gateway_id: self.local_gateway_id,
            carrier_gateway_id: self.carrier_gateway_id,
            network_interface_id: self.network_interface_id,
            route_table_id: self.route_table_id,
            vpc_peering_connection_id: self.vpc_peering_connection_id,
            core_network_arn: self.core_network_arn,
        })
    }
}

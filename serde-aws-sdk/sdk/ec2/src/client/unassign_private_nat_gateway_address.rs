// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UnassignPrivateNatGatewayAddress`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`nat_gateway_id(impl Into<String>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::nat_gateway_id) / [`set_nat_gateway_id(Option<String>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::set_nat_gateway_id): <p>The NAT gateway ID.</p>
    ///   - [`private_ip_addresses(Vec<String>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::private_ip_addresses) / [`set_private_ip_addresses(Option<Vec<String>>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::set_private_ip_addresses): <p>The private IPv4 addresses you want to unassign.</p>
    ///   - [`max_drain_duration_seconds(i32)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::max_drain_duration_seconds) / [`set_max_drain_duration_seconds(Option<i32>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::set_max_drain_duration_seconds): <p>The maximum amount of time to wait (in seconds) before forcibly releasing the IP addresses if connections are still in progress. Default value is 350 seconds.</p>
    ///   - [`dry_run(bool)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`UnassignPrivateNatGatewayAddressOutput`](crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressOutput) with field(s):
    ///   - [`nat_gateway_id(Option<String>)`](crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressOutput::nat_gateway_id): <p>The NAT gateway ID.</p>
    ///   - [`nat_gateway_addresses(Option<Vec<NatGatewayAddress>>)`](crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressOutput::nat_gateway_addresses): <p>Information about the NAT gateway IP addresses.</p>
    /// - On failure, responds with [`SdkError<UnassignPrivateNatGatewayAddressError>`](crate::operation::unassign_private_nat_gateway_address::UnassignPrivateNatGatewayAddressError)
    pub fn unassign_private_nat_gateway_address(&self) -> crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder{
        crate::operation::unassign_private_nat_gateway_address::builders::UnassignPrivateNatGatewayAddressFluentBuilder::new(self.handle.clone())
    }
}
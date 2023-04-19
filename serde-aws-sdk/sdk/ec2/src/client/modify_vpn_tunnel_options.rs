// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVpnTunnelOptions`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpn_connection_id(impl Into<String>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::vpn_connection_id) / [`set_vpn_connection_id(Option<String>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::set_vpn_connection_id): <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    ///   - [`vpn_tunnel_outside_ip_address(impl Into<String>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::vpn_tunnel_outside_ip_address) / [`set_vpn_tunnel_outside_ip_address(Option<String>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::set_vpn_tunnel_outside_ip_address): <p>The external IP address of the VPN tunnel.</p>
    ///   - [`tunnel_options(ModifyVpnTunnelOptionsSpecification)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::tunnel_options) / [`set_tunnel_options(Option<ModifyVpnTunnelOptionsSpecification>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::set_tunnel_options): <p>The tunnel options to modify.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyVpnTunnelOptionsOutput`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsOutput) with field(s):
    ///   - [`vpn_connection(Option<VpnConnection>)`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsOutput::vpn_connection): <p>Information about the VPN connection.</p>
    /// - On failure, responds with [`SdkError<ModifyVpnTunnelOptionsError>`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsError)
    pub fn modify_vpn_tunnel_options(
        &self,
    ) -> crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder
    {
        crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsFluentBuilder::new(self.handle.clone())
    }
}

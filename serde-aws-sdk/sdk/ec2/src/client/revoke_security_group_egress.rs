// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RevokeSecurityGroupEgress`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`group_id(impl Into<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_group_id): <p>The ID of the security group.</p>
    ///   - [`ip_permissions(Vec<IpPermission>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::ip_permissions) / [`set_ip_permissions(Option<Vec<IpPermission>>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_ip_permissions): <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    ///   - [`security_group_rule_ids(Vec<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::security_group_rule_ids) / [`set_security_group_rule_ids(Option<Vec<String>>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_security_group_rule_ids): <p>The IDs of the security group rules.</p>
    ///   - [`cidr_ip(impl Into<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::cidr_ip) / [`set_cidr_ip(Option<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_cidr_ip): <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    ///   - [`from_port(i32)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::from_port) / [`set_from_port(Option<i32>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_from_port): <p>Not supported. Use a set of IP permissions to specify the port.</p>
    ///   - [`ip_protocol(impl Into<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::ip_protocol) / [`set_ip_protocol(Option<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_ip_protocol): <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    ///   - [`to_port(i32)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::to_port) / [`set_to_port(Option<i32>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_to_port): <p>Not supported. Use a set of IP permissions to specify the port.</p>
    ///   - [`source_security_group_name(impl Into<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::source_security_group_name) / [`set_source_security_group_name(Option<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_source_security_group_name): <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    ///   - [`source_security_group_owner_id(impl Into<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::source_security_group_owner_id) / [`set_source_security_group_owner_id(Option<String>)`](crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::set_source_security_group_owner_id): <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    /// - On success, responds with [`RevokeSecurityGroupEgressOutput`](crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, returns an error.</p>
    ///   - [`unknown_ip_permissions(Option<Vec<IpPermission>>)`](crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput::unknown_ip_permissions): <p>The outbound rules that were unknown to the service. In some cases, <code>unknownIpPermissionSet</code> might be in a different format from the request parameter. </p>
    /// - On failure, responds with [`SdkError<RevokeSecurityGroupEgressError>`](crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError)
    pub fn revoke_security_group_egress(&self) -> crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder{
        crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressFluentBuilder::new(self.handle.clone())
    }
}

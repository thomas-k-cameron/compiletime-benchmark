// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_security_group_ingress::_revoke_security_group_ingress_output::RevokeSecurityGroupIngressOutputBuilder;

pub use crate::operation::revoke_security_group_ingress::_revoke_security_group_ingress_input::RevokeSecurityGroupIngressInputBuilder;

/// Fluent builder constructing a request to `RevokeSecurityGroupIngress`.
///
/// <p>Removes the specified inbound (ingress) rules from a security group.</p>
/// <p>You can specify rules using either rule IDs or security group rule properties. If you use rule properties, the values that you specify (for example, ports) must match the existing rule's values exactly. Each rule has a protocol, from and to ports, and source (CIDR range, security group, or prefix list). For the TCP and UDP protocols, you must also specify the destination port or range of ports. For the ICMP protocol, you must also specify the ICMP type and code. If the security group rule has a description, you do not need to specify the description to revoke the rule.</p>
/// <p>[EC2-Classic, default VPC] If the values you specify do not match the existing rule's values, no error is returned, and the output describes the security group rules that were not revoked.</p>
/// <p>Amazon Web Services recommends that you describe the security group to verify that the rules were removed.</p>
/// <p>Rule changes are propagated to instances within the security group as quickly as possible. However, a small delay might occur.</p> <note>
/// <p>We are retiring EC2-Classic. We recommend that you migrate from EC2-Classic to a VPC. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/vpc-migrate.html">Migrate from EC2-Classic to a VPC</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// </note>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RevokeSecurityGroupIngressFluentBuilder {
                handle: std::sync::Arc<crate::client::Handle>,
                inner: crate::operation::revoke_security_group_ingress::builders::RevokeSecurityGroupIngressInputBuilder
            }
impl RevokeSecurityGroupIngressFluentBuilder {
    /// Creates a new `RevokeSecurityGroupIngress`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngress,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressError,
        >,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressError,
        >,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::revoke_security_group_ingress::builders::RevokeSecurityGroupIngressInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.revoke_security_group_ingress().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::revoke_security_group_ingress::builders::RevokeSecurityGroupIngressInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The CIDR IP address range. You can't specify this parameter when specifying a source security group.</p>
    pub fn cidr_ip(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.cidr_ip(input.into());
        self
    }
    /// <p>The CIDR IP address range. You can't specify this parameter when specifying a source security group.</p>
    pub fn set_cidr_ip(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_cidr_ip(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.inner = self.inner.from_port(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types.</p>
    pub fn set_from_port(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_from_port(input);
        self
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.group_id(input.into());
        self
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn set_group_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_group_id(input);
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn set_group_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// Appends an item to `IpPermissions`.
    ///
    /// To override the contents of this collection use [`set_ip_permissions`](Self::set_ip_permissions).
    ///
    /// <p>The sets of IP permissions. You can't specify a source security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        self.inner = self.inner.ip_permissions(input);
        self
    }
    /// <p>The sets of IP permissions. You can't specify a source security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn set_ip_permissions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::IpPermission>>,
    ) -> Self {
        self.inner = self.inner.set_ip_permissions(input);
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). Use <code>-1</code> to specify all.</p>
    pub fn ip_protocol(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.ip_protocol(input.into());
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). Use <code>-1</code> to specify all.</p>
    pub fn set_ip_protocol(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_ip_protocol(input);
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. For EC2-VPC, the source security group must be in the same VPC. To revoke a specific rule for an IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn source_security_group_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_security_group_name(input.into());
        self
    }
    /// <p>[EC2-Classic, default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. For EC2-VPC, the source security group must be in the same VPC. To revoke a specific rule for an IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn set_source_security_group_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_security_group_name(input);
        self
    }
    /// <p>[EC2-Classic] The Amazon Web Services account ID of the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. To revoke a specific rule for an IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn source_security_group_owner_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.source_security_group_owner_id(input.into());
        self
    }
    /// <p>[EC2-Classic] The Amazon Web Services account ID of the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. To revoke a specific rule for an IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn set_source_security_group_owner_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_source_security_group_owner_id(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.inner = self.inner.to_port(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes.</p>
    pub fn set_to_port(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_to_port(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// Appends an item to `SecurityGroupRuleIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_rule_ids`](Self::set_security_group_rule_ids).
    ///
    /// <p>The IDs of the security group rules.</p>
    pub fn security_group_rule_ids(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.security_group_rule_ids(input.into());
        self
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn set_security_group_rule_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_security_group_rule_ids(input);
        self
    }
}
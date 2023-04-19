// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a security group rule.</p>
/// <p>You must specify exactly one of the following parameters, based on the rule type:</p>
/// <ul>
/// <li> <p>CidrIpv4</p> </li>
/// <li> <p>CidrIpv6</p> </li>
/// <li> <p>PrefixListId</p> </li>
/// <li> <p>ReferencedGroupId</p> </li>
/// </ul>
/// <p>When you modify a rule, you cannot change the rule type. For example, if the rule uses an IPv4 address range, you must use <code>CidrIpv4</code> to specify a new IPv4 address range.</p>
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
pub struct SecurityGroupRuleRequest {
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    #[doc(hidden)]
    pub ip_protocol: std::option::Option<std::string::String>,
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    #[doc(hidden)]
    pub from_port: std::option::Option<i32>,
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    #[doc(hidden)]
    pub to_port: std::option::Option<i32>,
    /// <p>The IPv4 CIDR range. To specify a single IPv4 address, use the /32 prefix length. </p>
    #[doc(hidden)]
    pub cidr_ipv4: std::option::Option<std::string::String>,
    /// <p>The IPv6 CIDR range. To specify a single IPv6 address, use the /128 prefix length.</p>
    #[doc(hidden)]
    pub cidr_ipv6: std::option::Option<std::string::String>,
    /// <p>The ID of the prefix list.</p>
    #[doc(hidden)]
    pub prefix_list_id: std::option::Option<std::string::String>,
    /// <p>The ID of the security group that is referenced in the security group rule.</p>
    #[doc(hidden)]
    pub referenced_group_id: std::option::Option<std::string::String>,
    /// <p>The description of the security group rule.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
}
impl SecurityGroupRuleRequest {
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn ip_protocol(&self) -> std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(&self) -> std::option::Option<i32> {
        self.from_port
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(&self) -> std::option::Option<i32> {
        self.to_port
    }
    /// <p>The IPv4 CIDR range. To specify a single IPv4 address, use the /32 prefix length. </p>
    pub fn cidr_ipv4(&self) -> std::option::Option<&str> {
        self.cidr_ipv4.as_deref()
    }
    /// <p>The IPv6 CIDR range. To specify a single IPv6 address, use the /128 prefix length.</p>
    pub fn cidr_ipv6(&self) -> std::option::Option<&str> {
        self.cidr_ipv6.as_deref()
    }
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(&self) -> std::option::Option<&str> {
        self.prefix_list_id.as_deref()
    }
    /// <p>The ID of the security group that is referenced in the security group rule.</p>
    pub fn referenced_group_id(&self) -> std::option::Option<&str> {
        self.referenced_group_id.as_deref()
    }
    /// <p>The description of the security group rule.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl SecurityGroupRuleRequest {
    /// Creates a new builder-style object to manufacture [`SecurityGroupRuleRequest`](crate::types::SecurityGroupRuleRequest).
    pub fn builder() -> crate::types::builders::SecurityGroupRuleRequestBuilder {
        crate::types::builders::SecurityGroupRuleRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SecurityGroupRuleRequest;
/// A builder for [`SecurityGroupRuleRequest`](crate::types::SecurityGroupRuleRequest).
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
pub struct SecurityGroupRuleRequestBuilder {
    pub(crate) ip_protocol: std::option::Option<std::string::String>,
    pub(crate) from_port: std::option::Option<i32>,
    pub(crate) to_port: std::option::Option<i32>,
    pub(crate) cidr_ipv4: std::option::Option<std::string::String>,
    pub(crate) cidr_ipv6: std::option::Option<std::string::String>,
    pub(crate) prefix_list_id: std::option::Option<std::string::String>,
    pub(crate) referenced_group_id: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
}
impl SecurityGroupRuleRequestBuilder {
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn ip_protocol(mut self, input: impl Into<std::string::String>) -> Self {
        self.ip_protocol = Some(input.into());
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn set_ip_protocol(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_from_port(mut self, input: std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the code. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_to_port(mut self, input: std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>The IPv4 CIDR range. To specify a single IPv4 address, use the /32 prefix length. </p>
    pub fn cidr_ipv4(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr_ipv4 = Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR range. To specify a single IPv4 address, use the /32 prefix length. </p>
    pub fn set_cidr_ipv4(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr_ipv4 = input;
        self
    }
    /// <p>The IPv6 CIDR range. To specify a single IPv6 address, use the /128 prefix length.</p>
    pub fn cidr_ipv6(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr_ipv6 = Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR range. To specify a single IPv6 address, use the /128 prefix length.</p>
    pub fn set_cidr_ipv6(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr_ipv6 = input;
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.prefix_list_id = Some(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.prefix_list_id = input;
        self
    }
    /// <p>The ID of the security group that is referenced in the security group rule.</p>
    pub fn referenced_group_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.referenced_group_id = Some(input.into());
        self
    }
    /// <p>The ID of the security group that is referenced in the security group rule.</p>
    pub fn set_referenced_group_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.referenced_group_id = input;
        self
    }
    /// <p>The description of the security group rule.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of the security group rule.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Consumes the builder and constructs a [`SecurityGroupRuleRequest`](crate::types::SecurityGroupRuleRequest).
    pub fn build(self) -> crate::types::SecurityGroupRuleRequest {
        crate::types::SecurityGroupRuleRequest {
            ip_protocol: self.ip_protocol,
            from_port: self.from_port,
            to_port: self.to_port,
            cidr_ipv4: self.cidr_ipv4,
            cidr_ipv6: self.cidr_ipv6,
            prefix_list_id: self.prefix_list_id,
            referenced_group_id: self.referenced_group_id,
            description: self.description,
        }
    }
}
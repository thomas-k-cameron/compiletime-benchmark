// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes VPN connection options.</p>
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
pub struct VpnConnectionOptions {
    /// <p>Indicates whether acceleration is enabled for the VPN connection.</p>
    #[doc(hidden)]
    pub enable_acceleration: std::option::Option<bool>,
    /// <p>Indicates whether the VPN connection uses static routes only. Static routes must be used for devices that don't support BGP.</p>
    #[doc(hidden)]
    pub static_routes_only: std::option::Option<bool>,
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    #[doc(hidden)]
    pub local_ipv4_network_cidr: std::option::Option<std::string::String>,
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    #[doc(hidden)]
    pub remote_ipv4_network_cidr: std::option::Option<std::string::String>,
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    #[doc(hidden)]
    pub local_ipv6_network_cidr: std::option::Option<std::string::String>,
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    #[doc(hidden)]
    pub remote_ipv6_network_cidr: std::option::Option<std::string::String>,
    /// <p>The type of IPv4 address assigned to the outside interface of the customer gateway.</p>
    /// <p>Valid values: <code>PrivateIpv4</code> | <code>PublicIpv4</code> </p>
    /// <p>Default: <code>PublicIpv4</code> </p>
    #[doc(hidden)]
    pub outside_ip_address_type: std::option::Option<std::string::String>,
    /// <p>The transit gateway attachment ID in use for the VPN tunnel.</p>
    #[doc(hidden)]
    pub transport_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether the VPN tunnels process IPv4 or IPv6 traffic.</p>
    #[doc(hidden)]
    pub tunnel_inside_ip_version: std::option::Option<crate::types::TunnelInsideIpVersion>,
    /// <p>Indicates the VPN tunnel options.</p>
    #[doc(hidden)]
    pub tunnel_options: std::option::Option<std::vec::Vec<crate::types::TunnelOption>>,
}
impl VpnConnectionOptions {
    /// <p>Indicates whether acceleration is enabled for the VPN connection.</p>
    pub fn enable_acceleration(&self) -> std::option::Option<bool> {
        self.enable_acceleration
    }
    /// <p>Indicates whether the VPN connection uses static routes only. Static routes must be used for devices that don't support BGP.</p>
    pub fn static_routes_only(&self) -> std::option::Option<bool> {
        self.static_routes_only
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn local_ipv4_network_cidr(&self) -> std::option::Option<&str> {
        self.local_ipv4_network_cidr.as_deref()
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn remote_ipv4_network_cidr(&self) -> std::option::Option<&str> {
        self.remote_ipv4_network_cidr.as_deref()
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn local_ipv6_network_cidr(&self) -> std::option::Option<&str> {
        self.local_ipv6_network_cidr.as_deref()
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn remote_ipv6_network_cidr(&self) -> std::option::Option<&str> {
        self.remote_ipv6_network_cidr.as_deref()
    }
    /// <p>The type of IPv4 address assigned to the outside interface of the customer gateway.</p>
    /// <p>Valid values: <code>PrivateIpv4</code> | <code>PublicIpv4</code> </p>
    /// <p>Default: <code>PublicIpv4</code> </p>
    pub fn outside_ip_address_type(&self) -> std::option::Option<&str> {
        self.outside_ip_address_type.as_deref()
    }
    /// <p>The transit gateway attachment ID in use for the VPN tunnel.</p>
    pub fn transport_transit_gateway_attachment_id(&self) -> std::option::Option<&str> {
        self.transport_transit_gateway_attachment_id.as_deref()
    }
    /// <p>Indicates whether the VPN tunnels process IPv4 or IPv6 traffic.</p>
    pub fn tunnel_inside_ip_version(
        &self,
    ) -> std::option::Option<&crate::types::TunnelInsideIpVersion> {
        self.tunnel_inside_ip_version.as_ref()
    }
    /// <p>Indicates the VPN tunnel options.</p>
    pub fn tunnel_options(&self) -> std::option::Option<&[crate::types::TunnelOption]> {
        self.tunnel_options.as_deref()
    }
}
impl VpnConnectionOptions {
    /// Creates a new builder-style object to manufacture [`VpnConnectionOptions`](crate::types::VpnConnectionOptions).
    pub fn builder() -> crate::types::builders::VpnConnectionOptionsBuilder {
        crate::types::builders::VpnConnectionOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VpnConnectionOptions;
/// A builder for [`VpnConnectionOptions`](crate::types::VpnConnectionOptions).
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
pub struct VpnConnectionOptionsBuilder {
    pub(crate) enable_acceleration: std::option::Option<bool>,
    pub(crate) static_routes_only: std::option::Option<bool>,
    pub(crate) local_ipv4_network_cidr: std::option::Option<std::string::String>,
    pub(crate) remote_ipv4_network_cidr: std::option::Option<std::string::String>,
    pub(crate) local_ipv6_network_cidr: std::option::Option<std::string::String>,
    pub(crate) remote_ipv6_network_cidr: std::option::Option<std::string::String>,
    pub(crate) outside_ip_address_type: std::option::Option<std::string::String>,
    pub(crate) transport_transit_gateway_attachment_id: std::option::Option<std::string::String>,
    pub(crate) tunnel_inside_ip_version: std::option::Option<crate::types::TunnelInsideIpVersion>,
    pub(crate) tunnel_options: std::option::Option<std::vec::Vec<crate::types::TunnelOption>>,
}
impl VpnConnectionOptionsBuilder {
    /// <p>Indicates whether acceleration is enabled for the VPN connection.</p>
    pub fn enable_acceleration(mut self, input: bool) -> Self {
        self.enable_acceleration = Some(input);
        self
    }
    /// <p>Indicates whether acceleration is enabled for the VPN connection.</p>
    pub fn set_enable_acceleration(mut self, input: std::option::Option<bool>) -> Self {
        self.enable_acceleration = input;
        self
    }
    /// <p>Indicates whether the VPN connection uses static routes only. Static routes must be used for devices that don't support BGP.</p>
    pub fn static_routes_only(mut self, input: bool) -> Self {
        self.static_routes_only = Some(input);
        self
    }
    /// <p>Indicates whether the VPN connection uses static routes only. Static routes must be used for devices that don't support BGP.</p>
    pub fn set_static_routes_only(mut self, input: std::option::Option<bool>) -> Self {
        self.static_routes_only = input;
        self
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn local_ipv4_network_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_ipv4_network_cidr = Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn set_local_ipv4_network_cidr(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_ipv4_network_cidr = input;
        self
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn remote_ipv4_network_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.remote_ipv4_network_cidr = Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn set_remote_ipv4_network_cidr(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.remote_ipv4_network_cidr = input;
        self
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn local_ipv6_network_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_ipv6_network_cidr = Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    pub fn set_local_ipv6_network_cidr(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_ipv6_network_cidr = input;
        self
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn remote_ipv6_network_cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.remote_ipv6_network_cidr = Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    pub fn set_remote_ipv6_network_cidr(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.remote_ipv6_network_cidr = input;
        self
    }
    /// <p>The type of IPv4 address assigned to the outside interface of the customer gateway.</p>
    /// <p>Valid values: <code>PrivateIpv4</code> | <code>PublicIpv4</code> </p>
    /// <p>Default: <code>PublicIpv4</code> </p>
    pub fn outside_ip_address_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.outside_ip_address_type = Some(input.into());
        self
    }
    /// <p>The type of IPv4 address assigned to the outside interface of the customer gateway.</p>
    /// <p>Valid values: <code>PrivateIpv4</code> | <code>PublicIpv4</code> </p>
    /// <p>Default: <code>PublicIpv4</code> </p>
    pub fn set_outside_ip_address_type(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.outside_ip_address_type = input;
        self
    }
    /// <p>The transit gateway attachment ID in use for the VPN tunnel.</p>
    pub fn transport_transit_gateway_attachment_id(
        mut self,
        input: impl Into<std::string::String>,
    ) -> Self {
        self.transport_transit_gateway_attachment_id = Some(input.into());
        self
    }
    /// <p>The transit gateway attachment ID in use for the VPN tunnel.</p>
    pub fn set_transport_transit_gateway_attachment_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.transport_transit_gateway_attachment_id = input;
        self
    }
    /// <p>Indicates whether the VPN tunnels process IPv4 or IPv6 traffic.</p>
    pub fn tunnel_inside_ip_version(mut self, input: crate::types::TunnelInsideIpVersion) -> Self {
        self.tunnel_inside_ip_version = Some(input);
        self
    }
    /// <p>Indicates whether the VPN tunnels process IPv4 or IPv6 traffic.</p>
    pub fn set_tunnel_inside_ip_version(
        mut self,
        input: std::option::Option<crate::types::TunnelInsideIpVersion>,
    ) -> Self {
        self.tunnel_inside_ip_version = input;
        self
    }
    /// Appends an item to `tunnel_options`.
    ///
    /// To override the contents of this collection use [`set_tunnel_options`](Self::set_tunnel_options).
    ///
    /// <p>Indicates the VPN tunnel options.</p>
    pub fn tunnel_options(mut self, input: crate::types::TunnelOption) -> Self {
        let mut v = self.tunnel_options.unwrap_or_default();
        v.push(input);
        self.tunnel_options = Some(v);
        self
    }
    /// <p>Indicates the VPN tunnel options.</p>
    pub fn set_tunnel_options(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TunnelOption>>,
    ) -> Self {
        self.tunnel_options = input;
        self
    }
    /// Consumes the builder and constructs a [`VpnConnectionOptions`](crate::types::VpnConnectionOptions).
    pub fn build(self) -> crate::types::VpnConnectionOptions {
        crate::types::VpnConnectionOptions {
            enable_acceleration: self.enable_acceleration,
            static_routes_only: self.static_routes_only,
            local_ipv4_network_cidr: self.local_ipv4_network_cidr,
            remote_ipv4_network_cidr: self.remote_ipv4_network_cidr,
            local_ipv6_network_cidr: self.local_ipv6_network_cidr,
            remote_ipv6_network_cidr: self.remote_ipv6_network_cidr,
            outside_ip_address_type: self.outside_ip_address_type,
            transport_transit_gateway_attachment_id: self.transport_transit_gateway_attachment_id,
            tunnel_inside_ip_version: self.tunnel_inside_ip_version,
            tunnel_options: self.tunnel_options,
        }
    }
}

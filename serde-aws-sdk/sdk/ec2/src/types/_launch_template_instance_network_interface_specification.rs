// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a network interface.</p>
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
pub struct LaunchTemplateInstanceNetworkInterfaceSpecification {
    /// <p>Indicates whether to associate a Carrier IP address with eth0 for a new network interface.</p>
    /// <p>Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information about Carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP addresses</a> in the <i>Wavelength Developer Guide</i>.</p>
    #[doc(hidden)]
    pub associate_carrier_ip_address: std::option::Option<bool>,
    /// <p>Indicates whether to associate a public IPv4 address with eth0 for a new network interface.</p>
    #[doc(hidden)]
    pub associate_public_ip_address: std::option::Option<bool>,
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    #[doc(hidden)]
    pub delete_on_termination: std::option::Option<bool>,
    /// <p>A description for the network interface.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The device index for the network interface attachment.</p>
    #[doc(hidden)]
    pub device_index: std::option::Option<i32>,
    /// <p>The IDs of one or more security groups.</p>
    #[doc(hidden)]
    pub groups: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>The type of network interface.</p>
    #[doc(hidden)]
    pub interface_type: std::option::Option<std::string::String>,
    /// <p>The number of IPv6 addresses for the network interface.</p>
    #[doc(hidden)]
    pub ipv6_address_count: std::option::Option<i32>,
    /// <p>The IPv6 addresses for the network interface.</p>
    #[doc(hidden)]
    pub ipv6_addresses: std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    /// <p>The ID of the network interface.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The primary private IPv4 address of the network interface.</p>
    #[doc(hidden)]
    pub private_ip_address: std::option::Option<std::string::String>,
    /// <p>One or more private IPv4 addresses.</p>
    #[doc(hidden)]
    pub private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    /// <p>The number of secondary private IPv4 addresses for the network interface.</p>
    #[doc(hidden)]
    pub secondary_private_ip_address_count: std::option::Option<i32>,
    /// <p>The ID of the subnet for the network interface.</p>
    #[doc(hidden)]
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>The index of the network card.</p>
    #[doc(hidden)]
    pub network_card_index: std::option::Option<i32>,
    /// <p>One or more IPv4 prefixes assigned to the network interface.</p>
    #[doc(hidden)]
    pub ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationResponse>>,
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    #[doc(hidden)]
    pub ipv4_prefix_count: std::option::Option<i32>,
    /// <p>One or more IPv6 prefixes assigned to the network interface.</p>
    #[doc(hidden)]
    pub ipv6_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationResponse>>,
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    #[doc(hidden)]
    pub ipv6_prefix_count: std::option::Option<i32>,
}
impl LaunchTemplateInstanceNetworkInterfaceSpecification {
    /// <p>Indicates whether to associate a Carrier IP address with eth0 for a new network interface.</p>
    /// <p>Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information about Carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP addresses</a> in the <i>Wavelength Developer Guide</i>.</p>
    pub fn associate_carrier_ip_address(&self) -> std::option::Option<bool> {
        self.associate_carrier_ip_address
    }
    /// <p>Indicates whether to associate a public IPv4 address with eth0 for a new network interface.</p>
    pub fn associate_public_ip_address(&self) -> std::option::Option<bool> {
        self.associate_public_ip_address
    }
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    pub fn delete_on_termination(&self) -> std::option::Option<bool> {
        self.delete_on_termination
    }
    /// <p>A description for the network interface.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The device index for the network interface attachment.</p>
    pub fn device_index(&self) -> std::option::Option<i32> {
        self.device_index
    }
    /// <p>The IDs of one or more security groups.</p>
    pub fn groups(&self) -> std::option::Option<&[std::string::String]> {
        self.groups.as_deref()
    }
    /// <p>The type of network interface.</p>
    pub fn interface_type(&self) -> std::option::Option<&str> {
        self.interface_type.as_deref()
    }
    /// <p>The number of IPv6 addresses for the network interface.</p>
    pub fn ipv6_address_count(&self) -> std::option::Option<i32> {
        self.ipv6_address_count
    }
    /// <p>The IPv6 addresses for the network interface.</p>
    pub fn ipv6_addresses(&self) -> std::option::Option<&[crate::types::InstanceIpv6Address]> {
        self.ipv6_addresses.as_deref()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The primary private IPv4 address of the network interface.</p>
    pub fn private_ip_address(&self) -> std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p>One or more private IPv4 addresses.</p>
    pub fn private_ip_addresses(
        &self,
    ) -> std::option::Option<&[crate::types::PrivateIpAddressSpecification]> {
        self.private_ip_addresses.as_deref()
    }
    /// <p>The number of secondary private IPv4 addresses for the network interface.</p>
    pub fn secondary_private_ip_address_count(&self) -> std::option::Option<i32> {
        self.secondary_private_ip_address_count
    }
    /// <p>The ID of the subnet for the network interface.</p>
    pub fn subnet_id(&self) -> std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The index of the network card.</p>
    pub fn network_card_index(&self) -> std::option::Option<i32> {
        self.network_card_index
    }
    /// <p>One or more IPv4 prefixes assigned to the network interface.</p>
    pub fn ipv4_prefixes(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv4PrefixSpecificationResponse]> {
        self.ipv4_prefixes.as_deref()
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn ipv4_prefix_count(&self) -> std::option::Option<i32> {
        self.ipv4_prefix_count
    }
    /// <p>One or more IPv6 prefixes assigned to the network interface.</p>
    pub fn ipv6_prefixes(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv6PrefixSpecificationResponse]> {
        self.ipv6_prefixes.as_deref()
    }
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn ipv6_prefix_count(&self) -> std::option::Option<i32> {
        self.ipv6_prefix_count
    }
}
impl LaunchTemplateInstanceNetworkInterfaceSpecification {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateInstanceNetworkInterfaceSpecification`](crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification).
    pub fn builder(
    ) -> crate::types::builders::LaunchTemplateInstanceNetworkInterfaceSpecificationBuilder {
        crate::types::builders::LaunchTemplateInstanceNetworkInterfaceSpecificationBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification;
/// A builder for [`LaunchTemplateInstanceNetworkInterfaceSpecification`](crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification).
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
pub struct LaunchTemplateInstanceNetworkInterfaceSpecificationBuilder {
    pub(crate) associate_carrier_ip_address: std::option::Option<bool>,
    pub(crate) associate_public_ip_address: std::option::Option<bool>,
    pub(crate) delete_on_termination: std::option::Option<bool>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) device_index: std::option::Option<i32>,
    pub(crate) groups: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) interface_type: std::option::Option<std::string::String>,
    pub(crate) ipv6_address_count: std::option::Option<i32>,
    pub(crate) ipv6_addresses:
        std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) private_ip_address: std::option::Option<std::string::String>,
    pub(crate) private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    pub(crate) secondary_private_ip_address_count: std::option::Option<i32>,
    pub(crate) subnet_id: std::option::Option<std::string::String>,
    pub(crate) network_card_index: std::option::Option<i32>,
    pub(crate) ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationResponse>>,
    pub(crate) ipv4_prefix_count: std::option::Option<i32>,
    pub(crate) ipv6_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationResponse>>,
    pub(crate) ipv6_prefix_count: std::option::Option<i32>,
}
impl LaunchTemplateInstanceNetworkInterfaceSpecificationBuilder {
    /// <p>Indicates whether to associate a Carrier IP address with eth0 for a new network interface.</p>
    /// <p>Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information about Carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP addresses</a> in the <i>Wavelength Developer Guide</i>.</p>
    pub fn associate_carrier_ip_address(mut self, input: bool) -> Self {
        self.associate_carrier_ip_address = Some(input);
        self
    }
    /// <p>Indicates whether to associate a Carrier IP address with eth0 for a new network interface.</p>
    /// <p>Use this option when you launch an instance in a Wavelength Zone and want to associate a Carrier IP address with the network interface. For more information about Carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP addresses</a> in the <i>Wavelength Developer Guide</i>.</p>
    pub fn set_associate_carrier_ip_address(mut self, input: std::option::Option<bool>) -> Self {
        self.associate_carrier_ip_address = input;
        self
    }
    /// <p>Indicates whether to associate a public IPv4 address with eth0 for a new network interface.</p>
    pub fn associate_public_ip_address(mut self, input: bool) -> Self {
        self.associate_public_ip_address = Some(input);
        self
    }
    /// <p>Indicates whether to associate a public IPv4 address with eth0 for a new network interface.</p>
    pub fn set_associate_public_ip_address(mut self, input: std::option::Option<bool>) -> Self {
        self.associate_public_ip_address = input;
        self
    }
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    pub fn delete_on_termination(mut self, input: bool) -> Self {
        self.delete_on_termination = Some(input);
        self
    }
    /// <p>Indicates whether the network interface is deleted when the instance is terminated.</p>
    pub fn set_delete_on_termination(mut self, input: std::option::Option<bool>) -> Self {
        self.delete_on_termination = input;
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The device index for the network interface attachment.</p>
    pub fn device_index(mut self, input: i32) -> Self {
        self.device_index = Some(input);
        self
    }
    /// <p>The device index for the network interface attachment.</p>
    pub fn set_device_index(mut self, input: std::option::Option<i32>) -> Self {
        self.device_index = input;
        self
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The IDs of one or more security groups.</p>
    pub fn groups(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = Some(v);
        self
    }
    /// <p>The IDs of one or more security groups.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p>The type of network interface.</p>
    pub fn interface_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.interface_type = Some(input.into());
        self
    }
    /// <p>The type of network interface.</p>
    pub fn set_interface_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.interface_type = input;
        self
    }
    /// <p>The number of IPv6 addresses for the network interface.</p>
    pub fn ipv6_address_count(mut self, input: i32) -> Self {
        self.ipv6_address_count = Some(input);
        self
    }
    /// <p>The number of IPv6 addresses for the network interface.</p>
    pub fn set_ipv6_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv6_address_count = input;
        self
    }
    /// Appends an item to `ipv6_addresses`.
    ///
    /// To override the contents of this collection use [`set_ipv6_addresses`](Self::set_ipv6_addresses).
    ///
    /// <p>The IPv6 addresses for the network interface.</p>
    pub fn ipv6_addresses(mut self, input: crate::types::InstanceIpv6Address) -> Self {
        let mut v = self.ipv6_addresses.unwrap_or_default();
        v.push(input);
        self.ipv6_addresses = Some(v);
        self
    }
    /// <p>The IPv6 addresses for the network interface.</p>
    pub fn set_ipv6_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    ) -> Self {
        self.ipv6_addresses = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The primary private IPv4 address of the network interface.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip_address = Some(input.into());
        self
    }
    /// <p>The primary private IPv4 address of the network interface.</p>
    pub fn set_private_ip_address(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.private_ip_address = input;
        self
    }
    /// Appends an item to `private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>One or more private IPv4 addresses.</p>
    pub fn private_ip_addresses(
        mut self,
        input: crate::types::PrivateIpAddressSpecification,
    ) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input);
        self.private_ip_addresses = Some(v);
        self
    }
    /// <p>One or more private IPv4 addresses.</p>
    pub fn set_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    ) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p>The number of secondary private IPv4 addresses for the network interface.</p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.secondary_private_ip_address_count = Some(input);
        self
    }
    /// <p>The number of secondary private IPv4 addresses for the network interface.</p>
    pub fn set_secondary_private_ip_address_count(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.secondary_private_ip_address_count = input;
        self
    }
    /// <p>The ID of the subnet for the network interface.</p>
    pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.subnet_id = Some(input.into());
        self
    }
    /// <p>The ID of the subnet for the network interface.</p>
    pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The index of the network card.</p>
    pub fn network_card_index(mut self, input: i32) -> Self {
        self.network_card_index = Some(input);
        self
    }
    /// <p>The index of the network card.</p>
    pub fn set_network_card_index(mut self, input: std::option::Option<i32>) -> Self {
        self.network_card_index = input;
        self
    }
    /// Appends an item to `ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p>One or more IPv4 prefixes assigned to the network interface.</p>
    pub fn ipv4_prefixes(mut self, input: crate::types::Ipv4PrefixSpecificationResponse) -> Self {
        let mut v = self.ipv4_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv4_prefixes = Some(v);
        self
    }
    /// <p>One or more IPv4 prefixes assigned to the network interface.</p>
    pub fn set_ipv4_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationResponse>>,
    ) -> Self {
        self.ipv4_prefixes = input;
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn ipv4_prefix_count(mut self, input: i32) -> Self {
        self.ipv4_prefix_count = Some(input);
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn set_ipv4_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv4_prefix_count = input;
        self
    }
    /// Appends an item to `ipv6_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv6_prefixes`](Self::set_ipv6_prefixes).
    ///
    /// <p>One or more IPv6 prefixes assigned to the network interface.</p>
    pub fn ipv6_prefixes(mut self, input: crate::types::Ipv6PrefixSpecificationResponse) -> Self {
        let mut v = self.ipv6_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv6_prefixes = Some(v);
        self
    }
    /// <p>One or more IPv6 prefixes assigned to the network interface.</p>
    pub fn set_ipv6_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationResponse>>,
    ) -> Self {
        self.ipv6_prefixes = input;
        self
    }
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn ipv6_prefix_count(mut self, input: i32) -> Self {
        self.ipv6_prefix_count = Some(input);
        self
    }
    /// <p>The number of IPv6 prefixes that Amazon Web Services automatically assigned to the network interface.</p>
    pub fn set_ipv6_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv6_prefix_count = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateInstanceNetworkInterfaceSpecification`](crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification).
    pub fn build(self) -> crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification {
        crate::types::LaunchTemplateInstanceNetworkInterfaceSpecification {
            associate_carrier_ip_address: self.associate_carrier_ip_address,
            associate_public_ip_address: self.associate_public_ip_address,
            delete_on_termination: self.delete_on_termination,
            description: self.description,
            device_index: self.device_index,
            groups: self.groups,
            interface_type: self.interface_type,
            ipv6_address_count: self.ipv6_address_count,
            ipv6_addresses: self.ipv6_addresses,
            network_interface_id: self.network_interface_id,
            private_ip_address: self.private_ip_address,
            private_ip_addresses: self.private_ip_addresses,
            secondary_private_ip_address_count: self.secondary_private_ip_address_count,
            subnet_id: self.subnet_id,
            network_card_index: self.network_card_index,
            ipv4_prefixes: self.ipv4_prefixes,
            ipv4_prefix_count: self.ipv4_prefix_count,
            ipv6_prefixes: self.ipv6_prefixes,
            ipv6_prefix_count: self.ipv6_prefix_count,
        }
    }
}

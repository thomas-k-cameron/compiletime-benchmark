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
pub struct InstanceNetworkInterfaceSpecification {
    /// <p>Indicates whether to assign a public IPv4 address to an instance you launch in a VPC. The public IP address can only be assigned to a network interface for eth0, and can only be assigned to a new network interface, not an existing one. You cannot specify more than one network interface in the request. If launching into a default subnet, the default value is <code>true</code>.</p>
    #[doc(hidden)]
    pub associate_public_ip_address: std::option::Option<bool>,
    /// <p>If set to <code>true</code>, the interface is deleted when the instance is terminated. You can specify <code>true</code> only if creating a new network interface when launching an instance.</p>
    #[doc(hidden)]
    pub delete_on_termination: std::option::Option<bool>,
    /// <p>The description of the network interface. Applies only if creating a network interface when launching an instance.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The position of the network interface in the attachment order. A primary network interface has a device index of 0.</p>
    /// <p>If you specify a network interface when launching an instance, you must specify the device index.</p>
    #[doc(hidden)]
    pub device_index: std::option::Option<i32>,
    /// <p>The IDs of the security groups for the network interface. Applies only if creating a network interface when launching an instance.</p>
    #[doc(hidden)]
    pub groups: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>A number of IPv6 addresses to assign to the network interface. Amazon EC2 chooses the IPv6 addresses from the range of the subnet. You cannot specify this option and the option to assign specific IPv6 addresses in the same request. You can specify this option if you've specified a minimum number of instances to launch.</p>
    #[doc(hidden)]
    pub ipv6_address_count: std::option::Option<i32>,
    /// <p>The IPv6 addresses to assign to the network interface. You cannot specify this option and the option to assign a number of IPv6 addresses in the same request. You cannot specify this option if you've specified a minimum number of instances to launch.</p>
    #[doc(hidden)]
    pub ipv6_addresses: std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    /// <p>The ID of the network interface.</p>
    /// <p>If you are creating a Spot Fleet, omit this parameter because you can’t specify a network interface ID in a launch specification.</p>
    #[doc(hidden)]
    pub network_interface_id: std::option::Option<std::string::String>,
    /// <p>The private IPv4 address of the network interface. Applies only if creating a network interface when launching an instance. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    #[doc(hidden)]
    pub private_ip_address: std::option::Option<std::string::String>,
    /// <p>The private IPv4 addresses to assign to the network interface. Only one private IPv4 address can be designated as primary. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    #[doc(hidden)]
    pub private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    /// <p>The number of secondary private IPv4 addresses. You can't specify this option and specify more than one private IP address using the private IP addresses option. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    #[doc(hidden)]
    pub secondary_private_ip_address_count: std::option::Option<i32>,
    /// <p>The ID of the subnet associated with the network interface. Applies only if creating a network interface when launching an instance.</p>
    #[doc(hidden)]
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether to assign a carrier IP address to the network interface.</p>
    /// <p>You can only assign a carrier IP address to a network interface that is in a subnet in a Wavelength Zone. For more information about carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Amazon Web Services Wavelength Developer Guide</i>.</p>
    #[doc(hidden)]
    pub associate_carrier_ip_address: std::option::Option<bool>,
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> </p>
    #[doc(hidden)]
    pub interface_type: std::option::Option<std::string::String>,
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    /// <p>If you are using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a> to create Spot Instances, omit this parameter because you can’t specify the network card index when using this API. To specify the network card index, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>.</p>
    #[doc(hidden)]
    pub network_card_index: std::option::Option<i32>,
    /// <p>The IPv4 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    #[doc(hidden)]
    pub ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationRequest>>,
    /// <p>The number of IPv4 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefix</code> option.</p>
    #[doc(hidden)]
    pub ipv4_prefix_count: std::option::Option<i32>,
    /// <p>The IPv6 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option.</p>
    #[doc(hidden)]
    pub ipv6_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationRequest>>,
    /// <p>The number of IPv6 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option.</p>
    #[doc(hidden)]
    pub ipv6_prefix_count: std::option::Option<i32>,
}
impl InstanceNetworkInterfaceSpecification {
    /// <p>Indicates whether to assign a public IPv4 address to an instance you launch in a VPC. The public IP address can only be assigned to a network interface for eth0, and can only be assigned to a new network interface, not an existing one. You cannot specify more than one network interface in the request. If launching into a default subnet, the default value is <code>true</code>.</p>
    pub fn associate_public_ip_address(&self) -> std::option::Option<bool> {
        self.associate_public_ip_address
    }
    /// <p>If set to <code>true</code>, the interface is deleted when the instance is terminated. You can specify <code>true</code> only if creating a new network interface when launching an instance.</p>
    pub fn delete_on_termination(&self) -> std::option::Option<bool> {
        self.delete_on_termination
    }
    /// <p>The description of the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The position of the network interface in the attachment order. A primary network interface has a device index of 0.</p>
    /// <p>If you specify a network interface when launching an instance, you must specify the device index.</p>
    pub fn device_index(&self) -> std::option::Option<i32> {
        self.device_index
    }
    /// <p>The IDs of the security groups for the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn groups(&self) -> std::option::Option<&[std::string::String]> {
        self.groups.as_deref()
    }
    /// <p>A number of IPv6 addresses to assign to the network interface. Amazon EC2 chooses the IPv6 addresses from the range of the subnet. You cannot specify this option and the option to assign specific IPv6 addresses in the same request. You can specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn ipv6_address_count(&self) -> std::option::Option<i32> {
        self.ipv6_address_count
    }
    /// <p>The IPv6 addresses to assign to the network interface. You cannot specify this option and the option to assign a number of IPv6 addresses in the same request. You cannot specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn ipv6_addresses(&self) -> std::option::Option<&[crate::types::InstanceIpv6Address]> {
        self.ipv6_addresses.as_deref()
    }
    /// <p>The ID of the network interface.</p>
    /// <p>If you are creating a Spot Fleet, omit this parameter because you can’t specify a network interface ID in a launch specification.</p>
    pub fn network_interface_id(&self) -> std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The private IPv4 address of the network interface. Applies only if creating a network interface when launching an instance. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn private_ip_address(&self) -> std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p>The private IPv4 addresses to assign to the network interface. Only one private IPv4 address can be designated as primary. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn private_ip_addresses(
        &self,
    ) -> std::option::Option<&[crate::types::PrivateIpAddressSpecification]> {
        self.private_ip_addresses.as_deref()
    }
    /// <p>The number of secondary private IPv4 addresses. You can't specify this option and specify more than one private IP address using the private IP addresses option. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn secondary_private_ip_address_count(&self) -> std::option::Option<i32> {
        self.secondary_private_ip_address_count
    }
    /// <p>The ID of the subnet associated with the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn subnet_id(&self) -> std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>Indicates whether to assign a carrier IP address to the network interface.</p>
    /// <p>You can only assign a carrier IP address to a network interface that is in a subnet in a Wavelength Zone. For more information about carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Amazon Web Services Wavelength Developer Guide</i>.</p>
    pub fn associate_carrier_ip_address(&self) -> std::option::Option<bool> {
        self.associate_carrier_ip_address
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> </p>
    pub fn interface_type(&self) -> std::option::Option<&str> {
        self.interface_type.as_deref()
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    /// <p>If you are using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a> to create Spot Instances, omit this parameter because you can’t specify the network card index when using this API. To specify the network card index, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>.</p>
    pub fn network_card_index(&self) -> std::option::Option<i32> {
        self.network_card_index
    }
    /// <p>The IPv4 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn ipv4_prefixes(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv4PrefixSpecificationRequest]> {
        self.ipv4_prefixes.as_deref()
    }
    /// <p>The number of IPv4 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefix</code> option.</p>
    pub fn ipv4_prefix_count(&self) -> std::option::Option<i32> {
        self.ipv4_prefix_count
    }
    /// <p>The IPv6 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option.</p>
    pub fn ipv6_prefixes(
        &self,
    ) -> std::option::Option<&[crate::types::Ipv6PrefixSpecificationRequest]> {
        self.ipv6_prefixes.as_deref()
    }
    /// <p>The number of IPv6 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option.</p>
    pub fn ipv6_prefix_count(&self) -> std::option::Option<i32> {
        self.ipv6_prefix_count
    }
}
impl InstanceNetworkInterfaceSpecification {
    /// Creates a new builder-style object to manufacture [`InstanceNetworkInterfaceSpecification`](crate::types::InstanceNetworkInterfaceSpecification).
    pub fn builder() -> crate::types::builders::InstanceNetworkInterfaceSpecificationBuilder {
        crate::types::builders::InstanceNetworkInterfaceSpecificationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::InstanceNetworkInterfaceSpecification;
/// A builder for [`InstanceNetworkInterfaceSpecification`](crate::types::InstanceNetworkInterfaceSpecification).
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
pub struct InstanceNetworkInterfaceSpecificationBuilder {
    pub(crate) associate_public_ip_address: std::option::Option<bool>,
    pub(crate) delete_on_termination: std::option::Option<bool>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) device_index: std::option::Option<i32>,
    pub(crate) groups: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) ipv6_address_count: std::option::Option<i32>,
    pub(crate) ipv6_addresses:
        std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    pub(crate) network_interface_id: std::option::Option<std::string::String>,
    pub(crate) private_ip_address: std::option::Option<std::string::String>,
    pub(crate) private_ip_addresses:
        std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    pub(crate) secondary_private_ip_address_count: std::option::Option<i32>,
    pub(crate) subnet_id: std::option::Option<std::string::String>,
    pub(crate) associate_carrier_ip_address: std::option::Option<bool>,
    pub(crate) interface_type: std::option::Option<std::string::String>,
    pub(crate) network_card_index: std::option::Option<i32>,
    pub(crate) ipv4_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationRequest>>,
    pub(crate) ipv4_prefix_count: std::option::Option<i32>,
    pub(crate) ipv6_prefixes:
        std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationRequest>>,
    pub(crate) ipv6_prefix_count: std::option::Option<i32>,
}
impl InstanceNetworkInterfaceSpecificationBuilder {
    /// <p>Indicates whether to assign a public IPv4 address to an instance you launch in a VPC. The public IP address can only be assigned to a network interface for eth0, and can only be assigned to a new network interface, not an existing one. You cannot specify more than one network interface in the request. If launching into a default subnet, the default value is <code>true</code>.</p>
    pub fn associate_public_ip_address(mut self, input: bool) -> Self {
        self.associate_public_ip_address = Some(input);
        self
    }
    /// <p>Indicates whether to assign a public IPv4 address to an instance you launch in a VPC. The public IP address can only be assigned to a network interface for eth0, and can only be assigned to a new network interface, not an existing one. You cannot specify more than one network interface in the request. If launching into a default subnet, the default value is <code>true</code>.</p>
    pub fn set_associate_public_ip_address(mut self, input: std::option::Option<bool>) -> Self {
        self.associate_public_ip_address = input;
        self
    }
    /// <p>If set to <code>true</code>, the interface is deleted when the instance is terminated. You can specify <code>true</code> only if creating a new network interface when launching an instance.</p>
    pub fn delete_on_termination(mut self, input: bool) -> Self {
        self.delete_on_termination = Some(input);
        self
    }
    /// <p>If set to <code>true</code>, the interface is deleted when the instance is terminated. You can specify <code>true</code> only if creating a new network interface when launching an instance.</p>
    pub fn set_delete_on_termination(mut self, input: std::option::Option<bool>) -> Self {
        self.delete_on_termination = input;
        self
    }
    /// <p>The description of the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>The description of the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The position of the network interface in the attachment order. A primary network interface has a device index of 0.</p>
    /// <p>If you specify a network interface when launching an instance, you must specify the device index.</p>
    pub fn device_index(mut self, input: i32) -> Self {
        self.device_index = Some(input);
        self
    }
    /// <p>The position of the network interface in the attachment order. A primary network interface has a device index of 0.</p>
    /// <p>If you specify a network interface when launching an instance, you must specify the device index.</p>
    pub fn set_device_index(mut self, input: std::option::Option<i32>) -> Self {
        self.device_index = input;
        self
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The IDs of the security groups for the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn groups(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = Some(v);
        self
    }
    /// <p>The IDs of the security groups for the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// <p>A number of IPv6 addresses to assign to the network interface. Amazon EC2 chooses the IPv6 addresses from the range of the subnet. You cannot specify this option and the option to assign specific IPv6 addresses in the same request. You can specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn ipv6_address_count(mut self, input: i32) -> Self {
        self.ipv6_address_count = Some(input);
        self
    }
    /// <p>A number of IPv6 addresses to assign to the network interface. Amazon EC2 chooses the IPv6 addresses from the range of the subnet. You cannot specify this option and the option to assign specific IPv6 addresses in the same request. You can specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn set_ipv6_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv6_address_count = input;
        self
    }
    /// Appends an item to `ipv6_addresses`.
    ///
    /// To override the contents of this collection use [`set_ipv6_addresses`](Self::set_ipv6_addresses).
    ///
    /// <p>The IPv6 addresses to assign to the network interface. You cannot specify this option and the option to assign a number of IPv6 addresses in the same request. You cannot specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn ipv6_addresses(mut self, input: crate::types::InstanceIpv6Address) -> Self {
        let mut v = self.ipv6_addresses.unwrap_or_default();
        v.push(input);
        self.ipv6_addresses = Some(v);
        self
    }
    /// <p>The IPv6 addresses to assign to the network interface. You cannot specify this option and the option to assign a number of IPv6 addresses in the same request. You cannot specify this option if you've specified a minimum number of instances to launch.</p>
    pub fn set_ipv6_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceIpv6Address>>,
    ) -> Self {
        self.ipv6_addresses = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    /// <p>If you are creating a Spot Fleet, omit this parameter because you can’t specify a network interface ID in a launch specification.</p>
    pub fn network_interface_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_interface_id = Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    /// <p>If you are creating a Spot Fleet, omit this parameter because you can’t specify a network interface ID in a launch specification.</p>
    pub fn set_network_interface_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The private IPv4 address of the network interface. Applies only if creating a network interface when launching an instance. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn private_ip_address(mut self, input: impl Into<std::string::String>) -> Self {
        self.private_ip_address = Some(input.into());
        self
    }
    /// <p>The private IPv4 address of the network interface. Applies only if creating a network interface when launching an instance. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
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
    /// <p>The private IPv4 addresses to assign to the network interface. Only one private IPv4 address can be designated as primary. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn private_ip_addresses(
        mut self,
        input: crate::types::PrivateIpAddressSpecification,
    ) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input);
        self.private_ip_addresses = Some(v);
        self
    }
    /// <p>The private IPv4 addresses to assign to the network interface. Only one private IPv4 address can be designated as primary. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn set_private_ip_addresses(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PrivateIpAddressSpecification>>,
    ) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p>The number of secondary private IPv4 addresses. You can't specify this option and specify more than one private IP address using the private IP addresses option. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.secondary_private_ip_address_count = Some(input);
        self
    }
    /// <p>The number of secondary private IPv4 addresses. You can't specify this option and specify more than one private IP address using the private IP addresses option. You cannot specify this option if you're launching more than one instance in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a> request.</p>
    pub fn set_secondary_private_ip_address_count(
        mut self,
        input: std::option::Option<i32>,
    ) -> Self {
        self.secondary_private_ip_address_count = input;
        self
    }
    /// <p>The ID of the subnet associated with the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.subnet_id = Some(input.into());
        self
    }
    /// <p>The ID of the subnet associated with the network interface. Applies only if creating a network interface when launching an instance.</p>
    pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>Indicates whether to assign a carrier IP address to the network interface.</p>
    /// <p>You can only assign a carrier IP address to a network interface that is in a subnet in a Wavelength Zone. For more information about carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Amazon Web Services Wavelength Developer Guide</i>.</p>
    pub fn associate_carrier_ip_address(mut self, input: bool) -> Self {
        self.associate_carrier_ip_address = Some(input);
        self
    }
    /// <p>Indicates whether to assign a carrier IP address to the network interface.</p>
    /// <p>You can only assign a carrier IP address to a network interface that is in a subnet in a Wavelength Zone. For more information about carrier IP addresses, see <a href="https://docs.aws.amazon.com/wavelength/latest/developerguide/how-wavelengths-work.html#provider-owned-ip">Carrier IP address</a> in the <i>Amazon Web Services Wavelength Developer Guide</i>.</p>
    pub fn set_associate_carrier_ip_address(mut self, input: std::option::Option<bool>) -> Self {
        self.associate_carrier_ip_address = input;
        self
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> </p>
    pub fn interface_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.interface_type = Some(input.into());
        self
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> </p>
    pub fn set_interface_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.interface_type = input;
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    /// <p>If you are using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a> to create Spot Instances, omit this parameter because you can’t specify the network card index when using this API. To specify the network card index, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>.</p>
    pub fn network_card_index(mut self, input: i32) -> Self {
        self.network_card_index = Some(input);
        self
    }
    /// <p>The index of the network card. Some instance types support multiple network cards. The primary network interface must be assigned to network card index 0. The default is network card index 0.</p>
    /// <p>If you are using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a> to create Spot Instances, omit this parameter because you can’t specify the network card index when using this API. To specify the network card index, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>.</p>
    pub fn set_network_card_index(mut self, input: std::option::Option<i32>) -> Self {
        self.network_card_index = input;
        self
    }
    /// Appends an item to `ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p>The IPv4 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn ipv4_prefixes(mut self, input: crate::types::Ipv4PrefixSpecificationRequest) -> Self {
        let mut v = self.ipv4_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv4_prefixes = Some(v);
        self
    }
    /// <p>The IPv4 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn set_ipv4_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv4PrefixSpecificationRequest>>,
    ) -> Self {
        self.ipv4_prefixes = input;
        self
    }
    /// <p>The number of IPv4 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefix</code> option.</p>
    pub fn ipv4_prefix_count(mut self, input: i32) -> Self {
        self.ipv4_prefix_count = Some(input);
        self
    }
    /// <p>The number of IPv4 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv4Prefix</code> option.</p>
    pub fn set_ipv4_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv4_prefix_count = input;
        self
    }
    /// Appends an item to `ipv6_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv6_prefixes`](Self::set_ipv6_prefixes).
    ///
    /// <p>The IPv6 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option.</p>
    pub fn ipv6_prefixes(mut self, input: crate::types::Ipv6PrefixSpecificationRequest) -> Self {
        let mut v = self.ipv6_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv6_prefixes = Some(v);
        self
    }
    /// <p>The IPv6 delegated prefixes to be assigned to the network interface. You cannot use this option if you use the <code>Ipv6PrefixCount</code> option.</p>
    pub fn set_ipv6_prefixes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Ipv6PrefixSpecificationRequest>>,
    ) -> Self {
        self.ipv6_prefixes = input;
        self
    }
    /// <p>The number of IPv6 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option.</p>
    pub fn ipv6_prefix_count(mut self, input: i32) -> Self {
        self.ipv6_prefix_count = Some(input);
        self
    }
    /// <p>The number of IPv6 delegated prefixes to be automatically assigned to the network interface. You cannot use this option if you use the <code>Ipv6Prefix</code> option.</p>
    pub fn set_ipv6_prefix_count(mut self, input: std::option::Option<i32>) -> Self {
        self.ipv6_prefix_count = input;
        self
    }
    /// Consumes the builder and constructs a [`InstanceNetworkInterfaceSpecification`](crate::types::InstanceNetworkInterfaceSpecification).
    pub fn build(self) -> crate::types::InstanceNetworkInterfaceSpecification {
        crate::types::InstanceNetworkInterfaceSpecification {
            associate_public_ip_address: self.associate_public_ip_address,
            delete_on_termination: self.delete_on_termination,
            description: self.description,
            device_index: self.device_index,
            groups: self.groups,
            ipv6_address_count: self.ipv6_address_count,
            ipv6_addresses: self.ipv6_addresses,
            network_interface_id: self.network_interface_id,
            private_ip_address: self.private_ip_address,
            private_ip_addresses: self.private_ip_addresses,
            secondary_private_ip_address_count: self.secondary_private_ip_address_count,
            subnet_id: self.subnet_id,
            associate_carrier_ip_address: self.associate_carrier_ip_address,
            interface_type: self.interface_type,
            network_card_index: self.network_card_index,
            ipv4_prefixes: self.ipv4_prefixes,
            ipv4_prefix_count: self.ipv4_prefix_count,
            ipv6_prefixes: self.ipv6_prefixes,
            ipv6_prefix_count: self.ipv6_prefix_count,
        }
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IPv4 address pool.</p>
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
pub struct PublicIpv4Pool {
    /// <p>The ID of the address pool.</p>
    #[doc(hidden)]
    pub pool_id: std::option::Option<std::string::String>,
    /// <p>A description of the address pool.</p>
    #[doc(hidden)]
    pub description: std::option::Option<std::string::String>,
    /// <p>The address ranges.</p>
    #[doc(hidden)]
    pub pool_address_ranges: std::option::Option<std::vec::Vec<crate::types::PublicIpv4PoolRange>>,
    /// <p>The total number of addresses.</p>
    #[doc(hidden)]
    pub total_address_count: std::option::Option<i32>,
    /// <p>The total number of available addresses.</p>
    #[doc(hidden)]
    pub total_available_address_count: std::option::Option<i32>,
    /// <p>The name of the location from which the address pool is advertised. A network border group is a unique set of Availability Zones or Local Zones from where Amazon Web Services advertises public IP addresses.</p>
    #[doc(hidden)]
    pub network_border_group: std::option::Option<std::string::String>,
    /// <p>Any tags for the address pool.</p>
    #[doc(hidden)]
    pub tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl PublicIpv4Pool {
    /// <p>The ID of the address pool.</p>
    pub fn pool_id(&self) -> std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>A description of the address pool.</p>
    pub fn description(&self) -> std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The address ranges.</p>
    pub fn pool_address_ranges(&self) -> std::option::Option<&[crate::types::PublicIpv4PoolRange]> {
        self.pool_address_ranges.as_deref()
    }
    /// <p>The total number of addresses.</p>
    pub fn total_address_count(&self) -> std::option::Option<i32> {
        self.total_address_count
    }
    /// <p>The total number of available addresses.</p>
    pub fn total_available_address_count(&self) -> std::option::Option<i32> {
        self.total_available_address_count
    }
    /// <p>The name of the location from which the address pool is advertised. A network border group is a unique set of Availability Zones or Local Zones from where Amazon Web Services advertises public IP addresses.</p>
    pub fn network_border_group(&self) -> std::option::Option<&str> {
        self.network_border_group.as_deref()
    }
    /// <p>Any tags for the address pool.</p>
    pub fn tags(&self) -> std::option::Option<&[crate::types::Tag]> {
        self.tags.as_deref()
    }
}
impl PublicIpv4Pool {
    /// Creates a new builder-style object to manufacture [`PublicIpv4Pool`](crate::types::PublicIpv4Pool).
    pub fn builder() -> crate::types::builders::PublicIpv4PoolBuilder {
        crate::types::builders::PublicIpv4PoolBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::PublicIpv4Pool;
/// A builder for [`PublicIpv4Pool`](crate::types::PublicIpv4Pool).
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
pub struct PublicIpv4PoolBuilder {
    pub(crate) pool_id: std::option::Option<std::string::String>,
    pub(crate) description: std::option::Option<std::string::String>,
    pub(crate) pool_address_ranges:
        std::option::Option<std::vec::Vec<crate::types::PublicIpv4PoolRange>>,
    pub(crate) total_address_count: std::option::Option<i32>,
    pub(crate) total_available_address_count: std::option::Option<i32>,
    pub(crate) network_border_group: std::option::Option<std::string::String>,
    pub(crate) tags: std::option::Option<std::vec::Vec<crate::types::Tag>>,
}
impl PublicIpv4PoolBuilder {
    /// <p>The ID of the address pool.</p>
    pub fn pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.pool_id = Some(input.into());
        self
    }
    /// <p>The ID of the address pool.</p>
    pub fn set_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>A description of the address pool.</p>
    pub fn description(mut self, input: impl Into<std::string::String>) -> Self {
        self.description = Some(input.into());
        self
    }
    /// <p>A description of the address pool.</p>
    pub fn set_description(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// Appends an item to `pool_address_ranges`.
    ///
    /// To override the contents of this collection use [`set_pool_address_ranges`](Self::set_pool_address_ranges).
    ///
    /// <p>The address ranges.</p>
    pub fn pool_address_ranges(mut self, input: crate::types::PublicIpv4PoolRange) -> Self {
        let mut v = self.pool_address_ranges.unwrap_or_default();
        v.push(input);
        self.pool_address_ranges = Some(v);
        self
    }
    /// <p>The address ranges.</p>
    pub fn set_pool_address_ranges(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PublicIpv4PoolRange>>,
    ) -> Self {
        self.pool_address_ranges = input;
        self
    }
    /// <p>The total number of addresses.</p>
    pub fn total_address_count(mut self, input: i32) -> Self {
        self.total_address_count = Some(input);
        self
    }
    /// <p>The total number of addresses.</p>
    pub fn set_total_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.total_address_count = input;
        self
    }
    /// <p>The total number of available addresses.</p>
    pub fn total_available_address_count(mut self, input: i32) -> Self {
        self.total_available_address_count = Some(input);
        self
    }
    /// <p>The total number of available addresses.</p>
    pub fn set_total_available_address_count(mut self, input: std::option::Option<i32>) -> Self {
        self.total_available_address_count = input;
        self
    }
    /// <p>The name of the location from which the address pool is advertised. A network border group is a unique set of Availability Zones or Local Zones from where Amazon Web Services advertises public IP addresses.</p>
    pub fn network_border_group(mut self, input: impl Into<std::string::String>) -> Self {
        self.network_border_group = Some(input.into());
        self
    }
    /// <p>The name of the location from which the address pool is advertised. A network border group is a unique set of Availability Zones or Local Zones from where Amazon Web Services advertises public IP addresses.</p>
    pub fn set_network_border_group(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.network_border_group = input;
        self
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags for the address pool.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = Some(v);
        self
    }
    /// <p>Any tags for the address pool.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::Tag>>,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`PublicIpv4Pool`](crate::types::PublicIpv4Pool).
    pub fn build(self) -> crate::types::PublicIpv4Pool {
        crate::types::PublicIpv4Pool {
            pool_id: self.pool_id,
            description: self.description,
            pool_address_ranges: self.pool_address_ranges,
            total_address_count: self.total_address_count,
            total_available_address_count: self.total_available_address_count,
            network_border_group: self.network_border_group,
            tags: self.tags,
        }
    }
}
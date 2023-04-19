// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p> Information about a customer-owned IP address range. </p>
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
pub struct CoipCidr {
    /// <p> An address range in a customer-owned IP address space. </p>
    #[doc(hidden)]
    pub cidr: std::option::Option<std::string::String>,
    /// <p> The ID of the address pool. </p>
    #[doc(hidden)]
    pub coip_pool_id: std::option::Option<std::string::String>,
    /// <p> The ID of the local gateway route table. </p>
    #[doc(hidden)]
    pub local_gateway_route_table_id: std::option::Option<std::string::String>,
}
impl CoipCidr {
    /// <p> An address range in a customer-owned IP address space. </p>
    pub fn cidr(&self) -> std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p> The ID of the address pool. </p>
    pub fn coip_pool_id(&self) -> std::option::Option<&str> {
        self.coip_pool_id.as_deref()
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn local_gateway_route_table_id(&self) -> std::option::Option<&str> {
        self.local_gateway_route_table_id.as_deref()
    }
}
impl CoipCidr {
    /// Creates a new builder-style object to manufacture [`CoipCidr`](crate::types::CoipCidr).
    pub fn builder() -> crate::types::builders::CoipCidrBuilder {
        crate::types::builders::CoipCidrBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CoipCidr;
/// A builder for [`CoipCidr`](crate::types::CoipCidr).
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
pub struct CoipCidrBuilder {
    pub(crate) cidr: std::option::Option<std::string::String>,
    pub(crate) coip_pool_id: std::option::Option<std::string::String>,
    pub(crate) local_gateway_route_table_id: std::option::Option<std::string::String>,
}
impl CoipCidrBuilder {
    /// <p> An address range in a customer-owned IP address space. </p>
    pub fn cidr(mut self, input: impl Into<std::string::String>) -> Self {
        self.cidr = Some(input.into());
        self
    }
    /// <p> An address range in a customer-owned IP address space. </p>
    pub fn set_cidr(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// <p> The ID of the address pool. </p>
    pub fn coip_pool_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.coip_pool_id = Some(input.into());
        self
    }
    /// <p> The ID of the address pool. </p>
    pub fn set_coip_pool_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.coip_pool_id = input;
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn local_gateway_route_table_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.local_gateway_route_table_id = Some(input.into());
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn set_local_gateway_route_table_id(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.local_gateway_route_table_id = input;
        self
    }
    /// Consumes the builder and constructs a [`CoipCidr`](crate::types::CoipCidr).
    pub fn build(self) -> crate::types::CoipCidr {
        crate::types::CoipCidr {
            cidr: self.cidr,
            coip_pool_id: self.coip_pool_id,
            local_gateway_route_table_id: self.local_gateway_route_table_id,
        }
    }
}

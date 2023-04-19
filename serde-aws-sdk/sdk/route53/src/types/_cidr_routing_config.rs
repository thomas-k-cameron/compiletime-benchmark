// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The object that is specified in resource record set object when you are linking a resource record set to a CIDR location.</p>
/// <p>A <code>LocationName</code> with an asterisk “*” can be used to create a default CIDR record. <code>CollectionId</code> is still required for default record.</p>
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
pub struct CidrRoutingConfig {
    /// <p>The CIDR collection ID.</p>
    #[doc(hidden)]
    pub collection_id: std::option::Option<std::string::String>,
    /// <p>The CIDR collection location name.</p>
    #[doc(hidden)]
    pub location_name: std::option::Option<std::string::String>,
}
impl CidrRoutingConfig {
    /// <p>The CIDR collection ID.</p>
    pub fn collection_id(&self) -> std::option::Option<&str> {
        self.collection_id.as_deref()
    }
    /// <p>The CIDR collection location name.</p>
    pub fn location_name(&self) -> std::option::Option<&str> {
        self.location_name.as_deref()
    }
}
impl CidrRoutingConfig {
    /// Creates a new builder-style object to manufacture [`CidrRoutingConfig`](crate::types::CidrRoutingConfig).
    pub fn builder() -> crate::types::builders::CidrRoutingConfigBuilder {
        crate::types::builders::CidrRoutingConfigBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CidrRoutingConfig;
/// A builder for [`CidrRoutingConfig`](crate::types::CidrRoutingConfig).
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
pub struct CidrRoutingConfigBuilder {
    pub(crate) collection_id: std::option::Option<std::string::String>,
    pub(crate) location_name: std::option::Option<std::string::String>,
}
impl CidrRoutingConfigBuilder {
    /// <p>The CIDR collection ID.</p>
    pub fn collection_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.collection_id = Some(input.into());
        self
    }
    /// <p>The CIDR collection ID.</p>
    pub fn set_collection_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.collection_id = input;
        self
    }
    /// <p>The CIDR collection location name.</p>
    pub fn location_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.location_name = Some(input.into());
        self
    }
    /// <p>The CIDR collection location name.</p>
    pub fn set_location_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.location_name = input;
        self
    }
    /// Consumes the builder and constructs a [`CidrRoutingConfig`](crate::types::CidrRoutingConfig).
    pub fn build(self) -> crate::types::CidrRoutingConfig {
        crate::types::CidrRoutingConfig {
            collection_id: self.collection_id,
            location_name: self.location_name,
        }
    }
}

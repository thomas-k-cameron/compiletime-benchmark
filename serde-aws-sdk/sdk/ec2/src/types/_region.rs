// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Region.</p>
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
pub struct Region {
    /// <p>The Region service endpoint.</p>
    #[doc(hidden)]
    pub endpoint: std::option::Option<std::string::String>,
    /// <p>The name of the Region.</p>
    #[doc(hidden)]
    pub region_name: std::option::Option<std::string::String>,
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    #[doc(hidden)]
    pub opt_in_status: std::option::Option<std::string::String>,
}
impl Region {
    /// <p>The Region service endpoint.</p>
    pub fn endpoint(&self) -> std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The name of the Region.</p>
    pub fn region_name(&self) -> std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn opt_in_status(&self) -> std::option::Option<&str> {
        self.opt_in_status.as_deref()
    }
}
impl Region {
    /// Creates a new builder-style object to manufacture [`Region`](crate::types::Region).
    pub fn builder() -> crate::types::builders::RegionBuilder {
        crate::types::builders::RegionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Region;
/// A builder for [`Region`](crate::types::Region).
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
pub struct RegionBuilder {
    pub(crate) endpoint: std::option::Option<std::string::String>,
    pub(crate) region_name: std::option::Option<std::string::String>,
    pub(crate) opt_in_status: std::option::Option<std::string::String>,
}
impl RegionBuilder {
    /// <p>The Region service endpoint.</p>
    pub fn endpoint(mut self, input: impl Into<std::string::String>) -> Self {
        self.endpoint = Some(input.into());
        self
    }
    /// <p>The Region service endpoint.</p>
    pub fn set_endpoint(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The name of the Region.</p>
    pub fn region_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.region_name = Some(input.into());
        self
    }
    /// <p>The name of the Region.</p>
    pub fn set_region_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn opt_in_status(mut self, input: impl Into<std::string::String>) -> Self {
        self.opt_in_status = Some(input.into());
        self
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn set_opt_in_status(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.opt_in_status = input;
        self
    }
    /// Consumes the builder and constructs a [`Region`](crate::types::Region).
    pub fn build(self) -> crate::types::Region {
        crate::types::Region {
            endpoint: self.endpoint,
            region_name: self.region_name,
            opt_in_status: self.opt_in_status,
        }
    }
}

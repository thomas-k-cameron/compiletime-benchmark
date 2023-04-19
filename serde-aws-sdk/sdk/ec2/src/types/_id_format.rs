// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the ID format for a resource.</p>
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
pub struct IdFormat {
    /// <p>The date in UTC at which you are permanently switched over to using longer IDs. If a deadline is not yet available for this resource type, this field is not returned.</p>
    #[doc(hidden)]
    pub deadline: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The type of resource.</p>
    #[doc(hidden)]
    pub resource: std::option::Option<std::string::String>,
    /// <p>Indicates whether longer IDs (17-character IDs) are enabled for the resource.</p>
    #[doc(hidden)]
    pub use_long_ids: std::option::Option<bool>,
}
impl IdFormat {
    /// <p>The date in UTC at which you are permanently switched over to using longer IDs. If a deadline is not yet available for this resource type, this field is not returned.</p>
    pub fn deadline(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.deadline.as_ref()
    }
    /// <p>The type of resource.</p>
    pub fn resource(&self) -> std::option::Option<&str> {
        self.resource.as_deref()
    }
    /// <p>Indicates whether longer IDs (17-character IDs) are enabled for the resource.</p>
    pub fn use_long_ids(&self) -> std::option::Option<bool> {
        self.use_long_ids
    }
}
impl IdFormat {
    /// Creates a new builder-style object to manufacture [`IdFormat`](crate::types::IdFormat).
    pub fn builder() -> crate::types::builders::IdFormatBuilder {
        crate::types::builders::IdFormatBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::IdFormat;
/// A builder for [`IdFormat`](crate::types::IdFormat).
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
pub struct IdFormatBuilder {
    pub(crate) deadline: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) resource: std::option::Option<std::string::String>,
    pub(crate) use_long_ids: std::option::Option<bool>,
}
impl IdFormatBuilder {
    /// <p>The date in UTC at which you are permanently switched over to using longer IDs. If a deadline is not yet available for this resource type, this field is not returned.</p>
    pub fn deadline(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.deadline = Some(input);
        self
    }
    /// <p>The date in UTC at which you are permanently switched over to using longer IDs. If a deadline is not yet available for this resource type, this field is not returned.</p>
    pub fn set_deadline(mut self, input: std::option::Option<aws_smithy_types::DateTime>) -> Self {
        self.deadline = input;
        self
    }
    /// <p>The type of resource.</p>
    pub fn resource(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource = Some(input.into());
        self
    }
    /// <p>The type of resource.</p>
    pub fn set_resource(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// <p>Indicates whether longer IDs (17-character IDs) are enabled for the resource.</p>
    pub fn use_long_ids(mut self, input: bool) -> Self {
        self.use_long_ids = Some(input);
        self
    }
    /// <p>Indicates whether longer IDs (17-character IDs) are enabled for the resource.</p>
    pub fn set_use_long_ids(mut self, input: std::option::Option<bool>) -> Self {
        self.use_long_ids = input;
        self
    }
    /// Consumes the builder and constructs a [`IdFormat`](crate::types::IdFormat).
    pub fn build(self) -> crate::types::IdFormat {
        crate::types::IdFormat {
            deadline: self.deadline,
            resource: self.resource,
            use_long_ids: self.use_long_ids,
        }
    }
}
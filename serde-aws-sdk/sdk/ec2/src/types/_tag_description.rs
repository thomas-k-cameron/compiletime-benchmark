// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a tag.</p>
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
pub struct TagDescription {
    /// <p>The tag key.</p>
    #[doc(hidden)]
    pub key: std::option::Option<std::string::String>,
    /// <p>The ID of the resource.</p>
    #[doc(hidden)]
    pub resource_id: std::option::Option<std::string::String>,
    /// <p>The resource type.</p>
    #[doc(hidden)]
    pub resource_type: std::option::Option<crate::types::ResourceType>,
    /// <p>The tag value.</p>
    #[doc(hidden)]
    pub value: std::option::Option<std::string::String>,
}
impl TagDescription {
    /// <p>The tag key.</p>
    pub fn key(&self) -> std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(&self) -> std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The resource type.</p>
    pub fn resource_type(&self) -> std::option::Option<&crate::types::ResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The tag value.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl TagDescription {
    /// Creates a new builder-style object to manufacture [`TagDescription`](crate::types::TagDescription).
    pub fn builder() -> crate::types::builders::TagDescriptionBuilder {
        crate::types::builders::TagDescriptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::TagDescription;
/// A builder for [`TagDescription`](crate::types::TagDescription).
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
pub struct TagDescriptionBuilder {
    pub(crate) key: std::option::Option<std::string::String>,
    pub(crate) resource_id: std::option::Option<std::string::String>,
    pub(crate) resource_type: std::option::Option<crate::types::ResourceType>,
    pub(crate) value: std::option::Option<std::string::String>,
}
impl TagDescriptionBuilder {
    /// <p>The tag key.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.key = Some(input.into());
        self
    }
    /// <p>The tag key.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.resource_id = Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The resource type.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.resource_type = Some(input);
        self
    }
    /// <p>The resource type.</p>
    pub fn set_resource_type(
        mut self,
        input: std::option::Option<crate::types::ResourceType>,
    ) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The tag value.</p>
    pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
        self.value = Some(input.into());
        self
    }
    /// <p>The tag value.</p>
    pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`TagDescription`](crate::types::TagDescription).
    pub fn build(self) -> crate::types::TagDescription {
        crate::types::TagDescription {
            key: self.key,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            value: self.value,
        }
    }
}

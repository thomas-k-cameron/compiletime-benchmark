// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a value of an account attribute.</p>
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
pub struct AccountAttributeValue {
    /// <p>The value of the attribute.</p>
    #[doc(hidden)]
    pub attribute_value: std::option::Option<std::string::String>,
}
impl AccountAttributeValue {
    /// <p>The value of the attribute.</p>
    pub fn attribute_value(&self) -> std::option::Option<&str> {
        self.attribute_value.as_deref()
    }
}
impl AccountAttributeValue {
    /// Creates a new builder-style object to manufacture [`AccountAttributeValue`](crate::types::AccountAttributeValue).
    pub fn builder() -> crate::types::builders::AccountAttributeValueBuilder {
        crate::types::builders::AccountAttributeValueBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::AccountAttributeValue;
/// A builder for [`AccountAttributeValue`](crate::types::AccountAttributeValue).
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
pub struct AccountAttributeValueBuilder {
    pub(crate) attribute_value: std::option::Option<std::string::String>,
}
impl AccountAttributeValueBuilder {
    /// <p>The value of the attribute.</p>
    pub fn attribute_value(mut self, input: impl Into<std::string::String>) -> Self {
        self.attribute_value = Some(input.into());
        self
    }
    /// <p>The value of the attribute.</p>
    pub fn set_attribute_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.attribute_value = input;
        self
    }
    /// Consumes the builder and constructs a [`AccountAttributeValue`](crate::types::AccountAttributeValue).
    pub fn build(self) -> crate::types::AccountAttributeValue {
        crate::types::AccountAttributeValue {
            attribute_value: self.attribute_value,
        }
    }
}

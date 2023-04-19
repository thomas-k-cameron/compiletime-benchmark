// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The encryption algorithm for phase 2 IKE negotiations.</p>
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
pub struct Phase2EncryptionAlgorithmsListValue {
    /// <p>The encryption algorithm.</p>
    #[doc(hidden)]
    pub value: std::option::Option<std::string::String>,
}
impl Phase2EncryptionAlgorithmsListValue {
    /// <p>The encryption algorithm.</p>
    pub fn value(&self) -> std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Phase2EncryptionAlgorithmsListValue {
    /// Creates a new builder-style object to manufacture [`Phase2EncryptionAlgorithmsListValue`](crate::types::Phase2EncryptionAlgorithmsListValue).
    pub fn builder() -> crate::types::builders::Phase2EncryptionAlgorithmsListValueBuilder {
        crate::types::builders::Phase2EncryptionAlgorithmsListValueBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::Phase2EncryptionAlgorithmsListValue;
/// A builder for [`Phase2EncryptionAlgorithmsListValue`](crate::types::Phase2EncryptionAlgorithmsListValue).
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
pub struct Phase2EncryptionAlgorithmsListValueBuilder {
    pub(crate) value: std::option::Option<std::string::String>,
}
impl Phase2EncryptionAlgorithmsListValueBuilder {
    /// <p>The encryption algorithm.</p>
    pub fn value(mut self, input: impl Into<std::string::String>) -> Self {
        self.value = Some(input.into());
        self
    }
    /// <p>The encryption algorithm.</p>
    pub fn set_value(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// Consumes the builder and constructs a [`Phase2EncryptionAlgorithmsListValue`](crate::types::Phase2EncryptionAlgorithmsListValue).
    pub fn build(self) -> crate::types::Phase2EncryptionAlgorithmsListValue {
        crate::types::Phase2EncryptionAlgorithmsListValue { value: self.value }
    }
}
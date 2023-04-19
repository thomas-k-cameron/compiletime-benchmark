// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A structure that can contain a value in multiple encoding formats.</p>
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
pub struct ValueHolder {
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure.</p>
    #[doc(hidden)]
    pub ion_binary: std::option::Option<aws_smithy_types::Blob>,
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    #[doc(hidden)]
    pub ion_text: std::option::Option<std::string::String>,
}
impl ValueHolder {
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure.</p>
    pub fn ion_binary(&self) -> std::option::Option<&aws_smithy_types::Blob> {
        self.ion_binary.as_ref()
    }
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    pub fn ion_text(&self) -> std::option::Option<&str> {
        self.ion_text.as_deref()
    }
}
impl ValueHolder {
    /// Creates a new builder-style object to manufacture [`ValueHolder`](crate::types::ValueHolder).
    pub fn builder() -> crate::types::builders::ValueHolderBuilder {
        crate::types::builders::ValueHolderBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ValueHolder;
/// A builder for [`ValueHolder`](crate::types::ValueHolder).
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
pub struct ValueHolderBuilder {
    pub(crate) ion_binary: std::option::Option<aws_smithy_types::Blob>,
    pub(crate) ion_text: std::option::Option<std::string::String>,
}
impl ValueHolderBuilder {
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure.</p>
    pub fn ion_binary(mut self, input: aws_smithy_types::Blob) -> Self {
        self.ion_binary = Some(input);
        self
    }
    /// <p>An Amazon Ion binary value contained in a <code>ValueHolder</code> structure.</p>
    pub fn set_ion_binary(mut self, input: std::option::Option<aws_smithy_types::Blob>) -> Self {
        self.ion_binary = input;
        self
    }
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    pub fn ion_text(mut self, input: impl Into<std::string::String>) -> Self {
        self.ion_text = Some(input.into());
        self
    }
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    pub fn set_ion_text(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.ion_text = input;
        self
    }
    /// Consumes the builder and constructs a [`ValueHolder`](crate::types::ValueHolder).
    pub fn build(self) -> crate::types::ValueHolder {
        crate::types::ValueHolder {
            ion_binary: self.ion_binary,
            ion_text: self.ion_text,
        }
    }
}

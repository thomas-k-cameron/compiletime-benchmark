// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether the instance is enabled for Amazon Web Services Nitro Enclaves.</p>
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
pub struct EnclaveOptions {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    #[doc(hidden)]
    pub enabled: std::option::Option<bool>,
}
impl EnclaveOptions {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn enabled(&self) -> std::option::Option<bool> {
        self.enabled
    }
}
impl EnclaveOptions {
    /// Creates a new builder-style object to manufacture [`EnclaveOptions`](crate::types::EnclaveOptions).
    pub fn builder() -> crate::types::builders::EnclaveOptionsBuilder {
        crate::types::builders::EnclaveOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EnclaveOptions;
/// A builder for [`EnclaveOptions`](crate::types::EnclaveOptions).
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
pub struct EnclaveOptionsBuilder {
    pub(crate) enabled: std::option::Option<bool>,
}
impl EnclaveOptionsBuilder {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// Consumes the builder and constructs a [`EnclaveOptions`](crate::types::EnclaveOptions).
    pub fn build(self) -> crate::types::EnclaveOptions {
        crate::types::EnclaveOptions {
            enabled: self.enabled,
        }
    }
}

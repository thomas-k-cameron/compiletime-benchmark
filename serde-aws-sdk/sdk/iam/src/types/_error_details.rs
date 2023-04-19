// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains information about the reason that the operation failed.</p>
/// <p>This data type is used as a response element in the <code>GetOrganizationsAccessReport</code>, <code>GetServiceLastAccessedDetails</code>, and <code>GetServiceLastAccessedDetailsWithEntities</code> operations.</p>
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
pub struct ErrorDetails {
    /// <p>Detailed information about the reason that the operation failed.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>The error code associated with the operation failure.</p>
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
}
impl ErrorDetails {
    /// <p>Detailed information about the reason that the operation failed.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The error code associated with the operation failure.</p>
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
}
impl ErrorDetails {
    /// Creates a new builder-style object to manufacture [`ErrorDetails`](crate::types::ErrorDetails).
    pub fn builder() -> crate::types::builders::ErrorDetailsBuilder {
        crate::types::builders::ErrorDetailsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ErrorDetails;
/// A builder for [`ErrorDetails`](crate::types::ErrorDetails).
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
pub struct ErrorDetailsBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) code: std::option::Option<std::string::String>,
}
impl ErrorDetailsBuilder {
    /// <p>Detailed information about the reason that the operation failed.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>Detailed information about the reason that the operation failed.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The error code associated with the operation failure.</p>
    pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
        self.code = Some(input.into());
        self
    }
    /// <p>The error code associated with the operation failure.</p>
    pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// Consumes the builder and constructs a [`ErrorDetails`](crate::types::ErrorDetails).
    pub fn build(self) -> crate::types::ErrorDetails {
        crate::types::ErrorDetails {
            message: self.message,
            code: self.code,
        }
    }
}

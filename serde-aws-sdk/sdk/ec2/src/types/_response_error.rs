// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the error that's returned when you cannot delete a launch template version.</p>
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
pub struct ResponseError {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::LaunchTemplateErrorCode>,
    /// <p>The error message, if applicable.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl ResponseError {
    /// <p>The error code.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::LaunchTemplateErrorCode> {
        self.code.as_ref()
    }
    /// <p>The error message, if applicable.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ResponseError {
    /// Creates a new builder-style object to manufacture [`ResponseError`](crate::types::ResponseError).
    pub fn builder() -> crate::types::builders::ResponseErrorBuilder {
        crate::types::builders::ResponseErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ResponseError;
/// A builder for [`ResponseError`](crate::types::ResponseError).
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
pub struct ResponseErrorBuilder {
    pub(crate) code: std::option::Option<crate::types::LaunchTemplateErrorCode>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl ResponseErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(mut self, input: crate::types::LaunchTemplateErrorCode) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::LaunchTemplateErrorCode>,
    ) -> Self {
        self.code = input;
        self
    }
    /// <p>The error message, if applicable.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The error message, if applicable.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`ResponseError`](crate::types::ResponseError).
    pub fn build(self) -> crate::types::ResponseError {
        crate::types::ResponseError {
            code: self.code,
            message: self.message,
        }
    }
}

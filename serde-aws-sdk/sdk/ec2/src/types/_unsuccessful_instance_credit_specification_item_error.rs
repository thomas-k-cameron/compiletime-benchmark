// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the error for the burstable performance instance whose credit option for CPU usage was not modified.</p>
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
pub struct UnsuccessfulInstanceCreditSpecificationItemError {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::UnsuccessfulInstanceCreditSpecificationErrorCode>,
    /// <p>The applicable error message.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl UnsuccessfulInstanceCreditSpecificationItemError {
    /// <p>The error code.</p>
    pub fn code(
        &self,
    ) -> std::option::Option<&crate::types::UnsuccessfulInstanceCreditSpecificationErrorCode> {
        self.code.as_ref()
    }
    /// <p>The applicable error message.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl UnsuccessfulInstanceCreditSpecificationItemError {
    /// Creates a new builder-style object to manufacture [`UnsuccessfulInstanceCreditSpecificationItemError`](crate::types::UnsuccessfulInstanceCreditSpecificationItemError).
    pub fn builder(
    ) -> crate::types::builders::UnsuccessfulInstanceCreditSpecificationItemErrorBuilder {
        crate::types::builders::UnsuccessfulInstanceCreditSpecificationItemErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::UnsuccessfulInstanceCreditSpecificationItemError;
/// A builder for [`UnsuccessfulInstanceCreditSpecificationItemError`](crate::types::UnsuccessfulInstanceCreditSpecificationItemError).
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
pub struct UnsuccessfulInstanceCreditSpecificationItemErrorBuilder {
    pub(crate) code:
        std::option::Option<crate::types::UnsuccessfulInstanceCreditSpecificationErrorCode>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl UnsuccessfulInstanceCreditSpecificationItemErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(
        mut self,
        input: crate::types::UnsuccessfulInstanceCreditSpecificationErrorCode,
    ) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::UnsuccessfulInstanceCreditSpecificationErrorCode>,
    ) -> Self {
        self.code = input;
        self
    }
    /// <p>The applicable error message.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The applicable error message.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`UnsuccessfulInstanceCreditSpecificationItemError`](crate::types::UnsuccessfulInstanceCreditSpecificationItemError).
    pub fn build(self) -> crate::types::UnsuccessfulInstanceCreditSpecificationItemError {
        crate::types::UnsuccessfulInstanceCreditSpecificationItemError {
            code: self.code,
            message: self.message,
        }
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Spot Fleet error.</p>
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
pub struct CancelSpotFleetRequestsError {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::CancelBatchErrorCode>,
    /// <p>The description for the error code.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl CancelSpotFleetRequestsError {
    /// <p>The error code.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::CancelBatchErrorCode> {
        self.code.as_ref()
    }
    /// <p>The description for the error code.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl CancelSpotFleetRequestsError {
    /// Creates a new builder-style object to manufacture [`CancelSpotFleetRequestsError`](crate::types::CancelSpotFleetRequestsError).
    pub fn builder() -> crate::types::builders::CancelSpotFleetRequestsErrorBuilder {
        crate::types::builders::CancelSpotFleetRequestsErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::CancelSpotFleetRequestsError;
/// A builder for [`CancelSpotFleetRequestsError`](crate::types::CancelSpotFleetRequestsError).
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
pub struct CancelSpotFleetRequestsErrorBuilder {
    pub(crate) code: std::option::Option<crate::types::CancelBatchErrorCode>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl CancelSpotFleetRequestsErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(mut self, input: crate::types::CancelBatchErrorCode) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::CancelBatchErrorCode>,
    ) -> Self {
        self.code = input;
        self
    }
    /// <p>The description for the error code.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The description for the error code.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotFleetRequestsError`](crate::types::CancelSpotFleetRequestsError).
    pub fn build(self) -> crate::types::CancelSpotFleetRequestsError {
        crate::types::CancelSpotFleetRequestsError {
            code: self.code,
            message: self.message,
        }
    }
}

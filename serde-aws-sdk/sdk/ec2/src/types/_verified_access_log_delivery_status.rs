// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a log delivery status.</p>
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
pub struct VerifiedAccessLogDeliveryStatus {
    /// <p>The status code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<crate::types::VerifiedAccessLogDeliveryStatusCode>,
    /// <p>The status message.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl VerifiedAccessLogDeliveryStatus {
    /// <p>The status code.</p>
    pub fn code(&self) -> std::option::Option<&crate::types::VerifiedAccessLogDeliveryStatusCode> {
        self.code.as_ref()
    }
    /// <p>The status message.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl VerifiedAccessLogDeliveryStatus {
    /// Creates a new builder-style object to manufacture [`VerifiedAccessLogDeliveryStatus`](crate::types::VerifiedAccessLogDeliveryStatus).
    pub fn builder() -> crate::types::builders::VerifiedAccessLogDeliveryStatusBuilder {
        crate::types::builders::VerifiedAccessLogDeliveryStatusBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::VerifiedAccessLogDeliveryStatus;
/// A builder for [`VerifiedAccessLogDeliveryStatus`](crate::types::VerifiedAccessLogDeliveryStatus).
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
pub struct VerifiedAccessLogDeliveryStatusBuilder {
    pub(crate) code: std::option::Option<crate::types::VerifiedAccessLogDeliveryStatusCode>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl VerifiedAccessLogDeliveryStatusBuilder {
    /// <p>The status code.</p>
    pub fn code(mut self, input: crate::types::VerifiedAccessLogDeliveryStatusCode) -> Self {
        self.code = Some(input);
        self
    }
    /// <p>The status code.</p>
    pub fn set_code(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessLogDeliveryStatusCode>,
    ) -> Self {
        self.code = input;
        self
    }
    /// <p>The status message.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The status message.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`VerifiedAccessLogDeliveryStatus`](crate::types::VerifiedAccessLogDeliveryStatus).
    pub fn build(self) -> crate::types::VerifiedAccessLogDeliveryStatus {
        crate::types::VerifiedAccessLogDeliveryStatus {
            code: self.code,
            message: self.message,
        }
    }
}

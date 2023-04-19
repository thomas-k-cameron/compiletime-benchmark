// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Provides status of the delivery of the snapshot or the configuration history to the specified Amazon S3 bucket. Also provides the status of notifications about the Amazon S3 delivery to the specified Amazon SNS topic.</p>
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
pub struct ConfigExportDeliveryInfo {
    /// <p>Status of the last attempted delivery.</p>
    #[doc(hidden)]
    pub last_status: std::option::Option<crate::types::DeliveryStatus>,
    /// <p>The error code from the last attempted delivery.</p>
    #[doc(hidden)]
    pub last_error_code: std::option::Option<std::string::String>,
    /// <p>The error message from the last attempted delivery.</p>
    #[doc(hidden)]
    pub last_error_message: std::option::Option<std::string::String>,
    /// <p>The time of the last attempted delivery.</p>
    #[doc(hidden)]
    pub last_attempt_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time of the last successful delivery.</p>
    #[doc(hidden)]
    pub last_successful_time: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The time that the next delivery occurs.</p>
    #[doc(hidden)]
    pub next_delivery_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl ConfigExportDeliveryInfo {
    /// <p>Status of the last attempted delivery.</p>
    pub fn last_status(&self) -> std::option::Option<&crate::types::DeliveryStatus> {
        self.last_status.as_ref()
    }
    /// <p>The error code from the last attempted delivery.</p>
    pub fn last_error_code(&self) -> std::option::Option<&str> {
        self.last_error_code.as_deref()
    }
    /// <p>The error message from the last attempted delivery.</p>
    pub fn last_error_message(&self) -> std::option::Option<&str> {
        self.last_error_message.as_deref()
    }
    /// <p>The time of the last attempted delivery.</p>
    pub fn last_attempt_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_attempt_time.as_ref()
    }
    /// <p>The time of the last successful delivery.</p>
    pub fn last_successful_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.last_successful_time.as_ref()
    }
    /// <p>The time that the next delivery occurs.</p>
    pub fn next_delivery_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.next_delivery_time.as_ref()
    }
}
impl ConfigExportDeliveryInfo {
    /// Creates a new builder-style object to manufacture [`ConfigExportDeliveryInfo`](crate::types::ConfigExportDeliveryInfo).
    pub fn builder() -> crate::types::builders::ConfigExportDeliveryInfoBuilder {
        crate::types::builders::ConfigExportDeliveryInfoBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ConfigExportDeliveryInfo;
/// A builder for [`ConfigExportDeliveryInfo`](crate::types::ConfigExportDeliveryInfo).
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
pub struct ConfigExportDeliveryInfoBuilder {
    pub(crate) last_status: std::option::Option<crate::types::DeliveryStatus>,
    pub(crate) last_error_code: std::option::Option<std::string::String>,
    pub(crate) last_error_message: std::option::Option<std::string::String>,
    pub(crate) last_attempt_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) last_successful_time: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) next_delivery_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl ConfigExportDeliveryInfoBuilder {
    /// <p>Status of the last attempted delivery.</p>
    pub fn last_status(mut self, input: crate::types::DeliveryStatus) -> Self {
        self.last_status = Some(input);
        self
    }
    /// <p>Status of the last attempted delivery.</p>
    pub fn set_last_status(
        mut self,
        input: std::option::Option<crate::types::DeliveryStatus>,
    ) -> Self {
        self.last_status = input;
        self
    }
    /// <p>The error code from the last attempted delivery.</p>
    pub fn last_error_code(mut self, input: impl Into<std::string::String>) -> Self {
        self.last_error_code = Some(input.into());
        self
    }
    /// <p>The error code from the last attempted delivery.</p>
    pub fn set_last_error_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.last_error_code = input;
        self
    }
    /// <p>The error message from the last attempted delivery.</p>
    pub fn last_error_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.last_error_message = Some(input.into());
        self
    }
    /// <p>The error message from the last attempted delivery.</p>
    pub fn set_last_error_message(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.last_error_message = input;
        self
    }
    /// <p>The time of the last attempted delivery.</p>
    pub fn last_attempt_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_attempt_time = Some(input);
        self
    }
    /// <p>The time of the last attempted delivery.</p>
    pub fn set_last_attempt_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_attempt_time = input;
        self
    }
    /// <p>The time of the last successful delivery.</p>
    pub fn last_successful_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.last_successful_time = Some(input);
        self
    }
    /// <p>The time of the last successful delivery.</p>
    pub fn set_last_successful_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.last_successful_time = input;
        self
    }
    /// <p>The time that the next delivery occurs.</p>
    pub fn next_delivery_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.next_delivery_time = Some(input);
        self
    }
    /// <p>The time that the next delivery occurs.</p>
    pub fn set_next_delivery_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.next_delivery_time = input;
        self
    }
    /// Consumes the builder and constructs a [`ConfigExportDeliveryInfo`](crate::types::ConfigExportDeliveryInfo).
    pub fn build(self) -> crate::types::ConfigExportDeliveryInfo {
        crate::types::ConfigExportDeliveryInfo {
            last_status: self.last_status,
            last_error_code: self.last_error_code,
            last_error_message: self.last_error_message,
            last_attempt_time: self.last_attempt_time,
            last_successful_time: self.last_successful_time,
            next_delivery_time: self.next_delivery_time,
        }
    }
}

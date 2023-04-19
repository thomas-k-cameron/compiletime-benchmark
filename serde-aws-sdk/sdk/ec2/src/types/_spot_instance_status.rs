// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the status of a Spot Instance request.</p>
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
pub struct SpotInstanceStatus {
    /// <p>The status code. For a list of status codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-request-status.html#spot-instance-request-status-understand">Spot request status codes</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
    /// <p>The description for the status code.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>The date and time of the most recent status update, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    #[doc(hidden)]
    pub update_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl SpotInstanceStatus {
    /// <p>The status code. For a list of status codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-request-status.html#spot-instance-request-status-understand">Spot request status codes</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The description for the status code.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
    /// <p>The date and time of the most recent status update, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn update_time(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
}
impl SpotInstanceStatus {
    /// Creates a new builder-style object to manufacture [`SpotInstanceStatus`](crate::types::SpotInstanceStatus).
    pub fn builder() -> crate::types::builders::SpotInstanceStatusBuilder {
        crate::types::builders::SpotInstanceStatusBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SpotInstanceStatus;
/// A builder for [`SpotInstanceStatus`](crate::types::SpotInstanceStatus).
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
pub struct SpotInstanceStatusBuilder {
    pub(crate) code: std::option::Option<std::string::String>,
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) update_time: std::option::Option<aws_smithy_types::DateTime>,
}
impl SpotInstanceStatusBuilder {
    /// <p>The status code. For a list of status codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-request-status.html#spot-instance-request-status-understand">Spot request status codes</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
        self.code = Some(input.into());
        self
    }
    /// <p>The status code. For a list of status codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-request-status.html#spot-instance-request-status-understand">Spot request status codes</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The description for the status code.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The description for the status code.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The date and time of the most recent status update, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn update_time(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.update_time = Some(input);
        self
    }
    /// <p>The date and time of the most recent status update, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn set_update_time(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.update_time = input;
        self
    }
    /// Consumes the builder and constructs a [`SpotInstanceStatus`](crate::types::SpotInstanceStatus).
    pub fn build(self) -> crate::types::SpotInstanceStatus {
        crate::types::SpotInstanceStatus {
            code: self.code,
            message: self.message,
            update_time: self.update_time,
        }
    }
}

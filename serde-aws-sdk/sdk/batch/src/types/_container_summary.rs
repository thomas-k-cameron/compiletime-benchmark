// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents summary details of a container within a job.</p>
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
pub struct ContainerSummary {
    /// <p>The exit code to return upon completion.</p>
    #[doc(hidden)]
    pub exit_code: std::option::Option<i32>,
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    #[doc(hidden)]
    pub reason: std::option::Option<std::string::String>,
}
impl ContainerSummary {
    /// <p>The exit code to return upon completion.</p>
    pub fn exit_code(&self) -> std::option::Option<i32> {
        self.exit_code
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn reason(&self) -> std::option::Option<&str> {
        self.reason.as_deref()
    }
}
impl ContainerSummary {
    /// Creates a new builder-style object to manufacture [`ContainerSummary`](crate::types::ContainerSummary).
    pub fn builder() -> crate::types::builders::ContainerSummaryBuilder {
        crate::types::builders::ContainerSummaryBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ContainerSummary;
/// A builder for [`ContainerSummary`](crate::types::ContainerSummary).
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
pub struct ContainerSummaryBuilder {
    pub(crate) exit_code: std::option::Option<i32>,
    pub(crate) reason: std::option::Option<std::string::String>,
}
impl ContainerSummaryBuilder {
    /// <p>The exit code to return upon completion.</p>
    pub fn exit_code(mut self, input: i32) -> Self {
        self.exit_code = Some(input);
        self
    }
    /// <p>The exit code to return upon completion.</p>
    pub fn set_exit_code(mut self, input: std::option::Option<i32>) -> Self {
        self.exit_code = input;
        self
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.reason = Some(input.into());
        self
    }
    /// <p>A short (255 max characters) human-readable string to provide additional details for a running or stopped container.</p>
    pub fn set_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.reason = input;
        self
    }
    /// Consumes the builder and constructs a [`ContainerSummary`](crate::types::ContainerSummary).
    pub fn build(self) -> crate::types::ContainerSummary {
        crate::types::ContainerSummary {
            exit_code: self.exit_code,
            reason: self.reason,
        }
    }
}

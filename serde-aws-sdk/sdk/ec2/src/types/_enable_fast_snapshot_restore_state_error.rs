// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an error that occurred when enabling fast snapshot restores.</p>
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
pub struct EnableFastSnapshotRestoreStateError {
    /// <p>The error code.</p>
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
    /// <p>The error message.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
}
impl EnableFastSnapshotRestoreStateError {
    /// <p>The error code.</p>
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The error message.</p>
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl EnableFastSnapshotRestoreStateError {
    /// Creates a new builder-style object to manufacture [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
    pub fn builder() -> crate::types::builders::EnableFastSnapshotRestoreStateErrorBuilder {
        crate::types::builders::EnableFastSnapshotRestoreStateErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EnableFastSnapshotRestoreStateError;
/// A builder for [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
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
pub struct EnableFastSnapshotRestoreStateErrorBuilder {
    pub(crate) code: std::option::Option<std::string::String>,
    pub(crate) message: std::option::Option<std::string::String>,
}
impl EnableFastSnapshotRestoreStateErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
        self.code = Some(input.into());
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The error message.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>The error message.</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Consumes the builder and constructs a [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
    pub fn build(self) -> crate::types::EnableFastSnapshotRestoreStateError {
        crate::types::EnableFastSnapshotRestoreStateError {
            code: self.code,
            message: self.message,
        }
    }
}

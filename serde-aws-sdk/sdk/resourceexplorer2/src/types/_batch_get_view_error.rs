// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A collection of error messages for any views that Amazon Web Services Resource Explorer couldn't retrieve details.</p>
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
pub struct BatchGetViewError {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view for which Resource Explorer failed to retrieve details.</p>
    #[doc(hidden)]
    pub view_arn: std::option::Option<std::string::String>,
    /// <p>The description of the error for the specified view.</p>
    #[doc(hidden)]
    pub error_message: std::option::Option<std::string::String>,
}
impl BatchGetViewError {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view for which Resource Explorer failed to retrieve details.</p>
    pub fn view_arn(&self) -> std::option::Option<&str> {
        self.view_arn.as_deref()
    }
    /// <p>The description of the error for the specified view.</p>
    pub fn error_message(&self) -> std::option::Option<&str> {
        self.error_message.as_deref()
    }
}
impl BatchGetViewError {
    /// Creates a new builder-style object to manufacture [`BatchGetViewError`](crate::types::BatchGetViewError).
    pub fn builder() -> crate::types::builders::BatchGetViewErrorBuilder {
        crate::types::builders::BatchGetViewErrorBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::BatchGetViewError;
/// A builder for [`BatchGetViewError`](crate::types::BatchGetViewError).
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
pub struct BatchGetViewErrorBuilder {
    pub(crate) view_arn: std::option::Option<std::string::String>,
    pub(crate) error_message: std::option::Option<std::string::String>,
}
impl BatchGetViewErrorBuilder {
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view for which Resource Explorer failed to retrieve details.</p>
    pub fn view_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.view_arn = Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon resource name (ARN)</a> of the view for which Resource Explorer failed to retrieve details.</p>
    pub fn set_view_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.view_arn = input;
        self
    }
    /// <p>The description of the error for the specified view.</p>
    pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
        self.error_message = Some(input.into());
        self
    }
    /// <p>The description of the error for the specified view.</p>
    pub fn set_error_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.error_message = input;
        self
    }
    /// Consumes the builder and constructs a [`BatchGetViewError`](crate::types::BatchGetViewError).
    pub fn build(self) -> crate::types::BatchGetViewError {
        crate::types::BatchGetViewError {
            view_arn: self.view_arn,
            error_message: self.error_message,
        }
    }
}
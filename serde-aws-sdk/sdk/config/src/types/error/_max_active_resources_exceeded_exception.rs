// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>You have reached the limit of active custom resource types in your account. There is a limit of 100,000. Delete unused resources using <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_DeleteResourceConfig.html">DeleteResourceConfig</a> <code></code>.</p>
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
pub struct MaxActiveResourcesExceededException {
    /// <p>Error executing the command</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl MaxActiveResourcesExceededException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for MaxActiveResourcesExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxActiveResourcesExceededException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for MaxActiveResourcesExceededException {}
impl aws_http::request_id::RequestId for crate::types::error::MaxActiveResourcesExceededException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for MaxActiveResourcesExceededException
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl MaxActiveResourcesExceededException {
    /// Creates a new builder-style object to manufacture [`MaxActiveResourcesExceededException`](crate::types::error::MaxActiveResourcesExceededException).
    pub fn builder() -> crate::types::error::builders::MaxActiveResourcesExceededExceptionBuilder {
        crate::types::error::builders::MaxActiveResourcesExceededExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::MaxActiveResourcesExceededException;
/// A builder for [`MaxActiveResourcesExceededException`](crate::types::error::MaxActiveResourcesExceededException).
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
pub struct MaxActiveResourcesExceededExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl MaxActiveResourcesExceededExceptionBuilder {
    /// <p>Error executing the command</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>Error executing the command</p>
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// Sets error metadata
    pub fn meta(mut self, meta: aws_smithy_types::error::ErrorMetadata) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Sets error metadata
    pub fn set_meta(
        &mut self,
        meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
    ) -> &mut Self {
        self.meta = meta;
        self
    }
    /// Consumes the builder and constructs a [`MaxActiveResourcesExceededException`](crate::types::error::MaxActiveResourcesExceededException).
    pub fn build(self) -> crate::types::error::MaxActiveResourcesExceededException {
        crate::types::error::MaxActiveResourcesExceededException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

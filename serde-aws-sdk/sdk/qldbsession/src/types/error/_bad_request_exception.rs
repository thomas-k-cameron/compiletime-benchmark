// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returned if the request is malformed or contains an error such as an invalid parameter value or a missing required parameter.</p>
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
pub struct BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl BadRequestException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
}
impl BadRequestException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BadRequestException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BadRequestException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for BadRequestException {}
impl aws_http::request_id::RequestId for crate::types::error::BadRequestException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for BadRequestException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl BadRequestException {
    /// Creates a new builder-style object to manufacture [`BadRequestException`](crate::types::error::BadRequestException).
    pub fn builder() -> crate::types::error::builders::BadRequestExceptionBuilder {
        crate::types::error::builders::BadRequestExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::BadRequestException;
/// A builder for [`BadRequestException`](crate::types::error::BadRequestException).
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
pub struct BadRequestExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) code: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl BadRequestExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_message(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.message = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
        self.code = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code = input;
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
    /// Consumes the builder and constructs a [`BadRequestException`](crate::types::error::BadRequestException).
    pub fn build(self) -> crate::types::error::BadRequestException {
        crate::types::error::BadRequestException {
            message: self.message,
            code: self.code,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

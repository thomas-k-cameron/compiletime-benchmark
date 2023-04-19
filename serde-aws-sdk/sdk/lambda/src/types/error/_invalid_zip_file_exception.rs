// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lambda could not unzip the deployment package.</p>
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
pub struct InvalidZipFileException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub r#type: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl InvalidZipFileException {
    #[allow(missing_docs)] // documentation missing in model
    pub fn r#type(&self) -> std::option::Option<&str> {
        self.r#type.as_deref()
    }
}
impl InvalidZipFileException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for InvalidZipFileException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidZipFileException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for InvalidZipFileException {}
impl aws_http::request_id::RequestId for crate::types::error::InvalidZipFileException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for InvalidZipFileException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl InvalidZipFileException {
    /// Creates a new builder-style object to manufacture [`InvalidZipFileException`](crate::types::error::InvalidZipFileException).
    pub fn builder() -> crate::types::error::builders::InvalidZipFileExceptionBuilder {
        crate::types::error::builders::InvalidZipFileExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::InvalidZipFileException;
/// A builder for [`InvalidZipFileException`](crate::types::error::InvalidZipFileException).
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
pub struct InvalidZipFileExceptionBuilder {
    pub(crate) r#type: std::option::Option<std::string::String>,
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl InvalidZipFileExceptionBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn r#type(mut self, input: impl Into<std::string::String>) -> Self {
        self.r#type = Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.r#type = input;
        self
    }
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
    /// Consumes the builder and constructs a [`InvalidZipFileException`](crate::types::error::InvalidZipFileException).
    pub fn build(self) -> crate::types::error::InvalidZipFileException {
        crate::types::error::InvalidZipFileException {
            r#type: self.r#type,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The request was rejected because the specified KMS key cannot decrypt the data. The <code>KeyId</code> in a <code>Decrypt</code> request and the <code>SourceKeyId</code> in a <code>ReEncrypt</code> request must identify the same KMS key that was used to encrypt the ciphertext.</p>
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
pub struct IncorrectKeyException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl IncorrectKeyException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for IncorrectKeyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IncorrectKeyException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for IncorrectKeyException {}
impl aws_http::request_id::RequestId for crate::types::error::IncorrectKeyException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for IncorrectKeyException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl IncorrectKeyException {
    /// Creates a new builder-style object to manufacture [`IncorrectKeyException`](crate::types::error::IncorrectKeyException).
    pub fn builder() -> crate::types::error::builders::IncorrectKeyExceptionBuilder {
        crate::types::error::builders::IncorrectKeyExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::IncorrectKeyException;
/// A builder for [`IncorrectKeyException`](crate::types::error::IncorrectKeyException).
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
pub struct IncorrectKeyExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl IncorrectKeyExceptionBuilder {
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
    /// Consumes the builder and constructs a [`IncorrectKeyException`](crate::types::error::IncorrectKeyException).
    pub fn build(self) -> crate::types::error::IncorrectKeyException {
        crate::types::error::IncorrectKeyException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

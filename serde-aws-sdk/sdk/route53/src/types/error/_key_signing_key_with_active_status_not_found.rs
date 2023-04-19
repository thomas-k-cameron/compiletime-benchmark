// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A key-signing key (KSK) with <code>ACTIVE</code> status wasn't found.</p>
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
pub struct KeySigningKeyWithActiveStatusNotFound {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl KeySigningKeyWithActiveStatusNotFound {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for KeySigningKeyWithActiveStatusNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "KeySigningKeyWithActiveStatusNotFound")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for KeySigningKeyWithActiveStatusNotFound {}
impl aws_http::request_id::RequestId
    for crate::types::error::KeySigningKeyWithActiveStatusNotFound
{
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for KeySigningKeyWithActiveStatusNotFound
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl KeySigningKeyWithActiveStatusNotFound {
    /// Creates a new builder-style object to manufacture [`KeySigningKeyWithActiveStatusNotFound`](crate::types::error::KeySigningKeyWithActiveStatusNotFound).
    pub fn builder() -> crate::types::error::builders::KeySigningKeyWithActiveStatusNotFoundBuilder
    {
        crate::types::error::builders::KeySigningKeyWithActiveStatusNotFoundBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::KeySigningKeyWithActiveStatusNotFound;
/// A builder for [`KeySigningKeyWithActiveStatusNotFound`](crate::types::error::KeySigningKeyWithActiveStatusNotFound).
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
pub struct KeySigningKeyWithActiveStatusNotFoundBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl KeySigningKeyWithActiveStatusNotFoundBuilder {
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
    /// Consumes the builder and constructs a [`KeySigningKeyWithActiveStatusNotFound`](crate::types::error::KeySigningKeyWithActiveStatusNotFound).
    pub fn build(self) -> crate::types::error::KeySigningKeyWithActiveStatusNotFound {
        crate::types::error::KeySigningKeyWithActiveStatusNotFound {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

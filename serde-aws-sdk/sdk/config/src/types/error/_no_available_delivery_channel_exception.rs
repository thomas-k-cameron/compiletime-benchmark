// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>There is no delivery channel available to record configurations.</p>
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
pub struct NoAvailableDeliveryChannelException {
    /// <p>Error executing the command</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl NoAvailableDeliveryChannelException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for NoAvailableDeliveryChannelException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoAvailableDeliveryChannelException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for NoAvailableDeliveryChannelException {}
impl aws_http::request_id::RequestId for crate::types::error::NoAvailableDeliveryChannelException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata
    for NoAvailableDeliveryChannelException
{
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl NoAvailableDeliveryChannelException {
    /// Creates a new builder-style object to manufacture [`NoAvailableDeliveryChannelException`](crate::types::error::NoAvailableDeliveryChannelException).
    pub fn builder() -> crate::types::error::builders::NoAvailableDeliveryChannelExceptionBuilder {
        crate::types::error::builders::NoAvailableDeliveryChannelExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::NoAvailableDeliveryChannelException;
/// A builder for [`NoAvailableDeliveryChannelException`](crate::types::error::NoAvailableDeliveryChannelException).
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
pub struct NoAvailableDeliveryChannelExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl NoAvailableDeliveryChannelExceptionBuilder {
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
    /// Consumes the builder and constructs a [`NoAvailableDeliveryChannelException`](crate::types::error::NoAvailableDeliveryChannelException).
    pub fn build(self) -> crate::types::error::NoAvailableDeliveryChannelException {
        crate::types::error::NoAvailableDeliveryChannelException {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

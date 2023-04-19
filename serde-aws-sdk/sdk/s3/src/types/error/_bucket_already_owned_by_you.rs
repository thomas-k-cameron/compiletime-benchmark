// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The bucket you tried to create already exists, and you own it. Amazon S3 returns this error in all Amazon Web Services Regions except in the North Virginia Region. For legacy compatibility, if you re-create an existing bucket that you already own in the North Virginia Region, Amazon S3 returns 200 OK and resets the bucket access control lists (ACLs).</p>
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
pub struct BucketAlreadyOwnedByYou {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl BucketAlreadyOwnedByYou {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for BucketAlreadyOwnedByYou {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BucketAlreadyOwnedByYou")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for BucketAlreadyOwnedByYou {}
impl crate::s3_request_id::RequestIdExt for crate::types::error::BucketAlreadyOwnedByYou {
    fn extended_request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().extended_request_id()
    }
}
impl aws_http::request_id::RequestId for crate::types::error::BucketAlreadyOwnedByYou {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for BucketAlreadyOwnedByYou {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl BucketAlreadyOwnedByYou {
    /// Creates a new builder-style object to manufacture [`BucketAlreadyOwnedByYou`](crate::types::error::BucketAlreadyOwnedByYou).
    pub fn builder() -> crate::types::error::builders::BucketAlreadyOwnedByYouBuilder {
        crate::types::error::builders::BucketAlreadyOwnedByYouBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::BucketAlreadyOwnedByYou;
/// A builder for [`BucketAlreadyOwnedByYou`](crate::types::error::BucketAlreadyOwnedByYou).
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
pub struct BucketAlreadyOwnedByYouBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl BucketAlreadyOwnedByYouBuilder {
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
    /// Consumes the builder and constructs a [`BucketAlreadyOwnedByYou`](crate::types::error::BucketAlreadyOwnedByYou).
    pub fn build(self) -> crate::types::error::BucketAlreadyOwnedByYou {
        crate::types::error::BucketAlreadyOwnedByYou {
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

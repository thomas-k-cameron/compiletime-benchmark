// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Returned if a retrieval job would exceed the current data policy's retrieval rate limit. For more information about data retrieval policies,</p>
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
pub struct PolicyEnforcedException {
    /// <p>Client</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<std::string::String>,
    /// <p>PolicyEnforcedException</p>
    #[doc(hidden)]
    pub code: std::option::Option<std::string::String>,
    /// <p>InitiateJob request denied by current data retrieval policy.</p>
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl PolicyEnforcedException {
    /// <p>Client</p>
    pub fn r#type(&self) -> std::option::Option<&str> {
        self.r#type.as_deref()
    }
    /// <p>PolicyEnforcedException</p>
    pub fn code(&self) -> std::option::Option<&str> {
        self.code.as_deref()
    }
}
impl PolicyEnforcedException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for PolicyEnforcedException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PolicyEnforcedException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for PolicyEnforcedException {}
impl aws_http::request_id::RequestId for crate::types::error::PolicyEnforcedException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for PolicyEnforcedException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl PolicyEnforcedException {
    /// Creates a new builder-style object to manufacture [`PolicyEnforcedException`](crate::types::error::PolicyEnforcedException).
    pub fn builder() -> crate::types::error::builders::PolicyEnforcedExceptionBuilder {
        crate::types::error::builders::PolicyEnforcedExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::PolicyEnforcedException;
/// A builder for [`PolicyEnforcedException`](crate::types::error::PolicyEnforcedException).
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
pub struct PolicyEnforcedExceptionBuilder {
    pub(crate) r#type: std::option::Option<std::string::String>,
    pub(crate) code: std::option::Option<std::string::String>,
    pub(crate) message: std::option::Option<std::string::String>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl PolicyEnforcedExceptionBuilder {
    /// <p>Client</p>
    pub fn r#type(mut self, input: impl Into<std::string::String>) -> Self {
        self.r#type = Some(input.into());
        self
    }
    /// <p>Client</p>
    pub fn set_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>PolicyEnforcedException</p>
    pub fn code(mut self, input: impl Into<std::string::String>) -> Self {
        self.code = Some(input.into());
        self
    }
    /// <p>PolicyEnforcedException</p>
    pub fn set_code(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>InitiateJob request denied by current data retrieval policy.</p>
    pub fn message(mut self, input: impl Into<std::string::String>) -> Self {
        self.message = Some(input.into());
        self
    }
    /// <p>InitiateJob request denied by current data retrieval policy.</p>
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
    /// Consumes the builder and constructs a [`PolicyEnforcedException`](crate::types::error::PolicyEnforcedException).
    pub fn build(self) -> crate::types::error::PolicyEnforcedException {
        crate::types::error::PolicyEnforcedException {
            r#type: self.r#type,
            code: self.code,
            message: self.message,
            meta: self.meta.unwrap_or_default(),
        }
    }
}

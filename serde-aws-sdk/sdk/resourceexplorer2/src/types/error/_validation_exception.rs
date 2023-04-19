// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>You provided an invalid value for one of the operation's parameters. Check the syntax for the operation, and try again.</p>
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
pub struct ValidationException {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub message: std::option::Option<std::string::String>,
    /// <p>An array of the request fields that had validation errors.</p>
    #[doc(hidden)]
    pub field_list: std::option::Option<std::vec::Vec<crate::types::ValidationExceptionField>>,
    pub(crate) meta: aws_smithy_types::error::ErrorMetadata,
}
impl ValidationException {
    /// <p>An array of the request fields that had validation errors.</p>
    pub fn field_list(&self) -> std::option::Option<&[crate::types::ValidationExceptionField]> {
        self.field_list.as_deref()
    }
}
impl ValidationException {
    /// Returns the error message.
    pub fn message(&self) -> std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl std::fmt::Display for ValidationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ValidationException")?;
        if let Some(inner_1) = &self.message {
            {
                write!(f, ": {}", inner_1)?;
            }
        }
        Ok(())
    }
}
impl std::error::Error for ValidationException {}
impl aws_http::request_id::RequestId for crate::types::error::ValidationException {
    fn request_id(&self) -> Option<&str> {
        use aws_smithy_types::error::metadata::ProvideErrorMetadata;
        self.meta().request_id()
    }
}
impl aws_smithy_types::error::metadata::ProvideErrorMetadata for ValidationException {
    fn meta(&self) -> &aws_smithy_types::error::ErrorMetadata {
        &self.meta
    }
}
impl ValidationException {
    /// Creates a new builder-style object to manufacture [`ValidationException`](crate::types::error::ValidationException).
    pub fn builder() -> crate::types::error::builders::ValidationExceptionBuilder {
        crate::types::error::builders::ValidationExceptionBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::error::ValidationException;
/// A builder for [`ValidationException`](crate::types::error::ValidationException).
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
pub struct ValidationExceptionBuilder {
    pub(crate) message: std::option::Option<std::string::String>,
    pub(crate) field_list:
        std::option::Option<std::vec::Vec<crate::types::ValidationExceptionField>>,
    meta: std::option::Option<aws_smithy_types::error::ErrorMetadata>,
}
impl ValidationExceptionBuilder {
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
    /// Appends an item to `field_list`.
    ///
    /// To override the contents of this collection use [`set_field_list`](Self::set_field_list).
    ///
    /// <p>An array of the request fields that had validation errors.</p>
    pub fn field_list(mut self, input: crate::types::ValidationExceptionField) -> Self {
        let mut v = self.field_list.unwrap_or_default();
        v.push(input);
        self.field_list = Some(v);
        self
    }
    /// <p>An array of the request fields that had validation errors.</p>
    pub fn set_field_list(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ValidationExceptionField>>,
    ) -> Self {
        self.field_list = input;
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
    /// Consumes the builder and constructs a [`ValidationException`](crate::types::error::ValidationException).
    pub fn build(self) -> crate::types::error::ValidationException {
        crate::types::error::ValidationException {
            message: self.message,
            field_list: self.field_list,
            meta: self.meta.unwrap_or_default(),
        }
    }
}
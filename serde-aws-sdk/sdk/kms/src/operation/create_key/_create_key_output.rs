// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct CreateKeyOutput {
    /// <p>Metadata associated with the KMS key.</p>
    #[doc(hidden)]
    pub key_metadata: std::option::Option<crate::types::KeyMetadata>,
    _request_id: Option<String>,
}
impl CreateKeyOutput {
    /// <p>Metadata associated with the KMS key.</p>
    pub fn key_metadata(&self) -> std::option::Option<&crate::types::KeyMetadata> {
        self.key_metadata.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateKeyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateKeyOutput {
    /// Creates a new builder-style object to manufacture [`CreateKeyOutput`](crate::operation::create_key::CreateKeyOutput).
    pub fn builder() -> crate::operation::create_key::builders::CreateKeyOutputBuilder {
        crate::operation::create_key::builders::CreateKeyOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_key::CreateKeyOutput;
/// A builder for [`CreateKeyOutput`](crate::operation::create_key::CreateKeyOutput).
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
pub struct CreateKeyOutputBuilder {
    pub(crate) key_metadata: std::option::Option<crate::types::KeyMetadata>,
    _request_id: Option<String>,
}
impl CreateKeyOutputBuilder {
    /// <p>Metadata associated with the KMS key.</p>
    pub fn key_metadata(mut self, input: crate::types::KeyMetadata) -> Self {
        self.key_metadata = Some(input);
        self
    }
    /// <p>Metadata associated with the KMS key.</p>
    pub fn set_key_metadata(
        mut self,
        input: std::option::Option<crate::types::KeyMetadata>,
    ) -> Self {
        self.key_metadata = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateKeyOutput`](crate::operation::create_key::CreateKeyOutput).
    pub fn build(self) -> crate::operation::create_key::CreateKeyOutput {
        crate::operation::create_key::CreateKeyOutput {
            key_metadata: self.key_metadata,
            _request_id: self._request_id,
        }
    }
}
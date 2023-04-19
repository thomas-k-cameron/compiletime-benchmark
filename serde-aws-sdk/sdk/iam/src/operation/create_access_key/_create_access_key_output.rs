// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>CreateAccessKey</code> request. </p>
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
pub struct CreateAccessKeyOutput {
    /// <p>A structure with details about the access key.</p>
    #[doc(hidden)]
    pub access_key: std::option::Option<crate::types::AccessKey>,
    _request_id: Option<String>,
}
impl CreateAccessKeyOutput {
    /// <p>A structure with details about the access key.</p>
    pub fn access_key(&self) -> std::option::Option<&crate::types::AccessKey> {
        self.access_key.as_ref()
    }
}
impl aws_http::request_id::RequestId for CreateAccessKeyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateAccessKeyOutput {
    /// Creates a new builder-style object to manufacture [`CreateAccessKeyOutput`](crate::operation::create_access_key::CreateAccessKeyOutput).
    pub fn builder() -> crate::operation::create_access_key::builders::CreateAccessKeyOutputBuilder
    {
        crate::operation::create_access_key::builders::CreateAccessKeyOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_access_key::CreateAccessKeyOutput;
/// A builder for [`CreateAccessKeyOutput`](crate::operation::create_access_key::CreateAccessKeyOutput).
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
pub struct CreateAccessKeyOutputBuilder {
    pub(crate) access_key: std::option::Option<crate::types::AccessKey>,
    _request_id: Option<String>,
}
impl CreateAccessKeyOutputBuilder {
    /// <p>A structure with details about the access key.</p>
    pub fn access_key(mut self, input: crate::types::AccessKey) -> Self {
        self.access_key = Some(input);
        self
    }
    /// <p>A structure with details about the access key.</p>
    pub fn set_access_key(mut self, input: std::option::Option<crate::types::AccessKey>) -> Self {
        self.access_key = input;
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
    /// Consumes the builder and constructs a [`CreateAccessKeyOutput`](crate::operation::create_access_key::CreateAccessKeyOutput).
    pub fn build(self) -> crate::operation::create_access_key::CreateAccessKeyOutput {
        crate::operation::create_access_key::CreateAccessKeyOutput {
            access_key: self.access_key,
            _request_id: self._request_id,
        }
    }
}
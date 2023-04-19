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
pub struct ModifyVerifiedAccessGroupOutput {
    /// <p>Details of Amazon Web Services Verified Access group.</p>
    #[doc(hidden)]
    pub verified_access_group: std::option::Option<crate::types::VerifiedAccessGroup>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessGroupOutput {
    /// <p>Details of Amazon Web Services Verified Access group.</p>
    pub fn verified_access_group(&self) -> std::option::Option<&crate::types::VerifiedAccessGroup> {
        self.verified_access_group.as_ref()
    }
}
impl aws_http::request_id::RequestId for ModifyVerifiedAccessGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVerifiedAccessGroupOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessGroupOutput`](crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput).
    pub fn builder() -> crate::operation::modify_verified_access_group::builders::ModifyVerifiedAccessGroupOutputBuilder{
        crate::operation::modify_verified_access_group::builders::ModifyVerifiedAccessGroupOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput;
/// A builder for [`ModifyVerifiedAccessGroupOutput`](crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput).
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
pub struct ModifyVerifiedAccessGroupOutputBuilder {
    pub(crate) verified_access_group: std::option::Option<crate::types::VerifiedAccessGroup>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessGroupOutputBuilder {
    /// <p>Details of Amazon Web Services Verified Access group.</p>
    pub fn verified_access_group(mut self, input: crate::types::VerifiedAccessGroup) -> Self {
        self.verified_access_group = Some(input);
        self
    }
    /// <p>Details of Amazon Web Services Verified Access group.</p>
    pub fn set_verified_access_group(
        mut self,
        input: std::option::Option<crate::types::VerifiedAccessGroup>,
    ) -> Self {
        self.verified_access_group = input;
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
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessGroupOutput`](crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput).
    pub fn build(
        self,
    ) -> crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput {
        crate::operation::modify_verified_access_group::ModifyVerifiedAccessGroupOutput {
            verified_access_group: self.verified_access_group,
            _request_id: self._request_id,
        }
    }
}

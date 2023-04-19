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
pub struct UpdateRoleDescriptionOutput {
    /// <p>A structure that contains details about the modified role.</p>
    #[doc(hidden)]
    pub role: std::option::Option<crate::types::Role>,
    _request_id: Option<String>,
}
impl UpdateRoleDescriptionOutput {
    /// <p>A structure that contains details about the modified role.</p>
    pub fn role(&self) -> std::option::Option<&crate::types::Role> {
        self.role.as_ref()
    }
}
impl aws_http::request_id::RequestId for UpdateRoleDescriptionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateRoleDescriptionOutput {
    /// Creates a new builder-style object to manufacture [`UpdateRoleDescriptionOutput`](crate::operation::update_role_description::UpdateRoleDescriptionOutput).
    pub fn builder(
    ) -> crate::operation::update_role_description::builders::UpdateRoleDescriptionOutputBuilder
    {
        crate::operation::update_role_description::builders::UpdateRoleDescriptionOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::update_role_description::UpdateRoleDescriptionOutput;
/// A builder for [`UpdateRoleDescriptionOutput`](crate::operation::update_role_description::UpdateRoleDescriptionOutput).
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
pub struct UpdateRoleDescriptionOutputBuilder {
    pub(crate) role: std::option::Option<crate::types::Role>,
    _request_id: Option<String>,
}
impl UpdateRoleDescriptionOutputBuilder {
    /// <p>A structure that contains details about the modified role.</p>
    pub fn role(mut self, input: crate::types::Role) -> Self {
        self.role = Some(input);
        self
    }
    /// <p>A structure that contains details about the modified role.</p>
    pub fn set_role(mut self, input: std::option::Option<crate::types::Role>) -> Self {
        self.role = input;
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
    /// Consumes the builder and constructs a [`UpdateRoleDescriptionOutput`](crate::operation::update_role_description::UpdateRoleDescriptionOutput).
    pub fn build(self) -> crate::operation::update_role_description::UpdateRoleDescriptionOutput {
        crate::operation::update_role_description::UpdateRoleDescriptionOutput {
            role: self.role,
            _request_id: self._request_id,
        }
    }
}
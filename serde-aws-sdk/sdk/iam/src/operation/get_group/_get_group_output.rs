// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>GetGroup</code> request. </p>
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
pub struct GetGroupOutput {
    /// <p>A structure that contains details about the group.</p>
    #[doc(hidden)]
    pub group: std::option::Option<crate::types::Group>,
    /// <p>A list of users in the group.</p>
    #[doc(hidden)]
    pub users: std::option::Option<std::vec::Vec<crate::types::User>>,
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetGroupOutput {
    /// <p>A structure that contains details about the group.</p>
    pub fn group(&self) -> std::option::Option<&crate::types::Group> {
        self.group.as_ref()
    }
    /// <p>A list of users in the group.</p>
    pub fn users(&self) -> std::option::Option<&[crate::types::User]> {
        self.users.as_deref()
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(&self) -> std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl aws_http::request_id::RequestId for GetGroupOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetGroupOutput {
    /// Creates a new builder-style object to manufacture [`GetGroupOutput`](crate::operation::get_group::GetGroupOutput).
    pub fn builder() -> crate::operation::get_group::builders::GetGroupOutputBuilder {
        crate::operation::get_group::builders::GetGroupOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_group::GetGroupOutput;
/// A builder for [`GetGroupOutput`](crate::operation::get_group::GetGroupOutput).
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
pub struct GetGroupOutputBuilder {
    pub(crate) group: std::option::Option<crate::types::Group>,
    pub(crate) users: std::option::Option<std::vec::Vec<crate::types::User>>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl GetGroupOutputBuilder {
    /// <p>A structure that contains details about the group.</p>
    pub fn group(mut self, input: crate::types::Group) -> Self {
        self.group = Some(input);
        self
    }
    /// <p>A structure that contains details about the group.</p>
    pub fn set_group(mut self, input: std::option::Option<crate::types::Group>) -> Self {
        self.group = input;
        self
    }
    /// Appends an item to `users`.
    ///
    /// To override the contents of this collection use [`set_users`](Self::set_users).
    ///
    /// <p>A list of users in the group.</p>
    pub fn users(mut self, input: crate::types::User) -> Self {
        let mut v = self.users.unwrap_or_default();
        v.push(input);
        self.users = Some(v);
        self
    }
    /// <p>A list of users in the group.</p>
    pub fn set_users(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::User>>,
    ) -> Self {
        self.users = input;
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn set_is_truncated(mut self, input: std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(mut self, input: impl Into<std::string::String>) -> Self {
        self.marker = Some(input.into());
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn set_marker(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.marker = input;
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
    /// Consumes the builder and constructs a [`GetGroupOutput`](crate::operation::get_group::GetGroupOutput).
    pub fn build(self) -> crate::operation::get_group::GetGroupOutput {
        crate::operation::get_group::GetGroupOutput {
            group: self.group,
            users: self.users,
            is_truncated: self.is_truncated.unwrap_or_default(),
            marker: self.marker,
            _request_id: self._request_id,
        }
    }
}

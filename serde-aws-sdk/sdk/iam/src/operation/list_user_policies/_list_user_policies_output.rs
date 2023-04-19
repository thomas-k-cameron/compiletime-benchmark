// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>ListUserPolicies</code> request. </p>
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
pub struct ListUserPoliciesOutput {
    /// <p>A list of policy names.</p>
    #[doc(hidden)]
    pub policy_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListUserPoliciesOutput {
    /// <p>A list of policy names.</p>
    pub fn policy_names(&self) -> std::option::Option<&[std::string::String]> {
        self.policy_names.as_deref()
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
impl aws_http::request_id::RequestId for ListUserPoliciesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListUserPoliciesOutput {
    /// Creates a new builder-style object to manufacture [`ListUserPoliciesOutput`](crate::operation::list_user_policies::ListUserPoliciesOutput).
    pub fn builder() -> crate::operation::list_user_policies::builders::ListUserPoliciesOutputBuilder
    {
        crate::operation::list_user_policies::builders::ListUserPoliciesOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_user_policies::ListUserPoliciesOutput;
/// A builder for [`ListUserPoliciesOutput`](crate::operation::list_user_policies::ListUserPoliciesOutput).
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
pub struct ListUserPoliciesOutputBuilder {
    pub(crate) policy_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListUserPoliciesOutputBuilder {
    /// Appends an item to `policy_names`.
    ///
    /// To override the contents of this collection use [`set_policy_names`](Self::set_policy_names).
    ///
    /// <p>A list of policy names.</p>
    pub fn policy_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.policy_names.unwrap_or_default();
        v.push(input.into());
        self.policy_names = Some(v);
        self
    }
    /// <p>A list of policy names.</p>
    pub fn set_policy_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.policy_names = input;
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
    /// Consumes the builder and constructs a [`ListUserPoliciesOutput`](crate::operation::list_user_policies::ListUserPoliciesOutput).
    pub fn build(self) -> crate::operation::list_user_policies::ListUserPoliciesOutput {
        crate::operation::list_user_policies::ListUserPoliciesOutput {
            policy_names: self.policy_names,
            is_truncated: self.is_truncated.unwrap_or_default(),
            marker: self.marker,
            _request_id: self._request_id,
        }
    }
}

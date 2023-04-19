// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <code>ListPolicyVersions</code> request. </p>
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
pub struct ListPolicyVersionsOutput {
    /// <p>A list of policy versions.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    #[doc(hidden)]
    pub versions: std::option::Option<std::vec::Vec<crate::types::PolicyVersion>>,
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    #[doc(hidden)]
    pub is_truncated: bool,
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    #[doc(hidden)]
    pub marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListPolicyVersionsOutput {
    /// <p>A list of policy versions.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn versions(&self) -> std::option::Option<&[crate::types::PolicyVersion]> {
        self.versions.as_deref()
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
impl aws_http::request_id::RequestId for ListPolicyVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListPolicyVersionsOutput {
    /// Creates a new builder-style object to manufacture [`ListPolicyVersionsOutput`](crate::operation::list_policy_versions::ListPolicyVersionsOutput).
    pub fn builder(
    ) -> crate::operation::list_policy_versions::builders::ListPolicyVersionsOutputBuilder {
        crate::operation::list_policy_versions::builders::ListPolicyVersionsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::list_policy_versions::ListPolicyVersionsOutput;
/// A builder for [`ListPolicyVersionsOutput`](crate::operation::list_policy_versions::ListPolicyVersionsOutput).
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
pub struct ListPolicyVersionsOutputBuilder {
    pub(crate) versions: std::option::Option<std::vec::Vec<crate::types::PolicyVersion>>,
    pub(crate) is_truncated: std::option::Option<bool>,
    pub(crate) marker: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListPolicyVersionsOutputBuilder {
    /// Appends an item to `versions`.
    ///
    /// To override the contents of this collection use [`set_versions`](Self::set_versions).
    ///
    /// <p>A list of policy versions.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn versions(mut self, input: crate::types::PolicyVersion) -> Self {
        let mut v = self.versions.unwrap_or_default();
        v.push(input);
        self.versions = Some(v);
        self
    }
    /// <p>A list of policy versions.</p>
    /// <p>For more information about managed policy versions, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/policies-managed-versions.html">Versioning for managed policies</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_versions(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PolicyVersion>>,
    ) -> Self {
        self.versions = input;
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
    /// Consumes the builder and constructs a [`ListPolicyVersionsOutput`](crate::operation::list_policy_versions::ListPolicyVersionsOutput).
    pub fn build(self) -> crate::operation::list_policy_versions::ListPolicyVersionsOutput {
        crate::operation::list_policy_versions::ListPolicyVersionsOutput {
            versions: self.versions,
            is_truncated: self.is_truncated.unwrap_or_default(),
            marker: self.marker,
            _request_id: self._request_id,
        }
    }
}

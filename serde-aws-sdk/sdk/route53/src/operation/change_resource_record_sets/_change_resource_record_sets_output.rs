// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A complex type containing the response for the request.</p>
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
pub struct ChangeResourceRecordSetsOutput {
    /// <p>A complex type that contains information about changes made to your hosted zone.</p>
    /// <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
    #[doc(hidden)]
    pub change_info: std::option::Option<crate::types::ChangeInfo>,
    _request_id: Option<String>,
}
impl ChangeResourceRecordSetsOutput {
    /// <p>A complex type that contains information about changes made to your hosted zone.</p>
    /// <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
    pub fn change_info(&self) -> std::option::Option<&crate::types::ChangeInfo> {
        self.change_info.as_ref()
    }
}
impl aws_http::request_id::RequestId for ChangeResourceRecordSetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ChangeResourceRecordSetsOutput {
    /// Creates a new builder-style object to manufacture [`ChangeResourceRecordSetsOutput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput).
    pub fn builder() -> crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsOutputBuilder{
        crate::operation::change_resource_record_sets::builders::ChangeResourceRecordSetsOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput;
/// A builder for [`ChangeResourceRecordSetsOutput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput).
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
pub struct ChangeResourceRecordSetsOutputBuilder {
    pub(crate) change_info: std::option::Option<crate::types::ChangeInfo>,
    _request_id: Option<String>,
}
impl ChangeResourceRecordSetsOutputBuilder {
    /// <p>A complex type that contains information about changes made to your hosted zone.</p>
    /// <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
    pub fn change_info(mut self, input: crate::types::ChangeInfo) -> Self {
        self.change_info = Some(input);
        self
    }
    /// <p>A complex type that contains information about changes made to your hosted zone.</p>
    /// <p>This element contains an ID that you use when performing a <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_GetChange.html">GetChange</a> action to get detailed information about the change.</p>
    pub fn set_change_info(mut self, input: std::option::Option<crate::types::ChangeInfo>) -> Self {
        self.change_info = input;
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
    /// Consumes the builder and constructs a [`ChangeResourceRecordSetsOutput`](crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput).
    pub fn build(
        self,
    ) -> crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput {
        crate::operation::change_resource_record_sets::ChangeResourceRecordSetsOutput {
            change_info: self.change_info,
            _request_id: self._request_id,
        }
    }
}
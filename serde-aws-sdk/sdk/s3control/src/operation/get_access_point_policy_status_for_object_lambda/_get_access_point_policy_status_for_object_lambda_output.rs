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
pub struct GetAccessPointPolicyStatusForObjectLambdaOutput {
    /// <p>Indicates whether this access point policy is public. For more information about how Amazon S3 evaluates policies to determine whether they are public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
    #[doc(hidden)]
    pub policy_status: std::option::Option<crate::types::PolicyStatus>,
    _request_id: Option<String>,
}
impl GetAccessPointPolicyStatusForObjectLambdaOutput {
    /// <p>Indicates whether this access point policy is public. For more information about how Amazon S3 evaluates policies to determine whether they are public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
    pub fn policy_status(&self) -> std::option::Option<&crate::types::PolicyStatus> {
        self.policy_status.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetAccessPointPolicyStatusForObjectLambdaOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAccessPointPolicyStatusForObjectLambdaOutput {
    /// Creates a new builder-style object to manufacture [`GetAccessPointPolicyStatusForObjectLambdaOutput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput).
    pub fn builder() -> crate::operation::get_access_point_policy_status_for_object_lambda::builders::GetAccessPointPolicyStatusForObjectLambdaOutputBuilder{
        crate::operation::get_access_point_policy_status_for_object_lambda::builders::GetAccessPointPolicyStatusForObjectLambdaOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput;
/// A builder for [`GetAccessPointPolicyStatusForObjectLambdaOutput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput).
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
pub struct GetAccessPointPolicyStatusForObjectLambdaOutputBuilder {
    pub(crate) policy_status: std::option::Option<crate::types::PolicyStatus>,
    _request_id: Option<String>,
}
impl GetAccessPointPolicyStatusForObjectLambdaOutputBuilder {
    /// <p>Indicates whether this access point policy is public. For more information about how Amazon S3 evaluates policies to determine whether they are public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
    pub fn policy_status(mut self, input: crate::types::PolicyStatus) -> Self {
        self.policy_status = Some(input);
        self
    }
    /// <p>Indicates whether this access point policy is public. For more information about how Amazon S3 evaluates policies to determine whether they are public, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/access-control-block-public-access.html#access-control-block-public-access-policy-status">The Meaning of "Public"</a> in the <i>Amazon S3 User Guide</i>. </p>
    pub fn set_policy_status(
        mut self,
        input: std::option::Option<crate::types::PolicyStatus>,
    ) -> Self {
        self.policy_status = input;
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
    /// Consumes the builder and constructs a [`GetAccessPointPolicyStatusForObjectLambdaOutput`](crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput).
    pub fn build(self) -> crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput{
        crate::operation::get_access_point_policy_status_for_object_lambda::GetAccessPointPolicyStatusForObjectLambdaOutput {
            policy_status: self.policy_status
            ,
            _request_id: self._request_id,
        }
    }
}

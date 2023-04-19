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
pub struct GetBucketVersioningOutput {
    /// <p>The versioning state of the S3 on Outposts bucket.</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::BucketVersioningStatus>,
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is returned only if the bucket has been configured with MFA delete. If MFA delete has never been configured for the bucket, this element is not returned.</p>
    #[doc(hidden)]
    pub mfa_delete: std::option::Option<crate::types::MfaDeleteStatus>,
    _request_id: Option<String>,
}
impl GetBucketVersioningOutput {
    /// <p>The versioning state of the S3 on Outposts bucket.</p>
    pub fn status(&self) -> std::option::Option<&crate::types::BucketVersioningStatus> {
        self.status.as_ref()
    }
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is returned only if the bucket has been configured with MFA delete. If MFA delete has never been configured for the bucket, this element is not returned.</p>
    pub fn mfa_delete(&self) -> std::option::Option<&crate::types::MfaDeleteStatus> {
        self.mfa_delete.as_ref()
    }
}
impl aws_http::request_id::RequestId for GetBucketVersioningOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetBucketVersioningOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketVersioningOutput`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput).
    pub fn builder(
    ) -> crate::operation::get_bucket_versioning::builders::GetBucketVersioningOutputBuilder {
        crate::operation::get_bucket_versioning::builders::GetBucketVersioningOutputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_bucket_versioning::GetBucketVersioningOutput;
/// A builder for [`GetBucketVersioningOutput`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput).
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
pub struct GetBucketVersioningOutputBuilder {
    pub(crate) status: std::option::Option<crate::types::BucketVersioningStatus>,
    pub(crate) mfa_delete: std::option::Option<crate::types::MfaDeleteStatus>,
    _request_id: Option<String>,
}
impl GetBucketVersioningOutputBuilder {
    /// <p>The versioning state of the S3 on Outposts bucket.</p>
    pub fn status(mut self, input: crate::types::BucketVersioningStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The versioning state of the S3 on Outposts bucket.</p>
    pub fn set_status(
        mut self,
        input: std::option::Option<crate::types::BucketVersioningStatus>,
    ) -> Self {
        self.status = input;
        self
    }
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is returned only if the bucket has been configured with MFA delete. If MFA delete has never been configured for the bucket, this element is not returned.</p>
    pub fn mfa_delete(mut self, input: crate::types::MfaDeleteStatus) -> Self {
        self.mfa_delete = Some(input);
        self
    }
    /// <p>Specifies whether MFA delete is enabled in the bucket versioning configuration. This element is returned only if the bucket has been configured with MFA delete. If MFA delete has never been configured for the bucket, this element is not returned.</p>
    pub fn set_mfa_delete(
        mut self,
        input: std::option::Option<crate::types::MfaDeleteStatus>,
    ) -> Self {
        self.mfa_delete = input;
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
    /// Consumes the builder and constructs a [`GetBucketVersioningOutput`](crate::operation::get_bucket_versioning::GetBucketVersioningOutput).
    pub fn build(self) -> crate::operation::get_bucket_versioning::GetBucketVersioningOutput {
        crate::operation::get_bucket_versioning::GetBucketVersioningOutput {
            status: self.status,
            mfa_delete: self.mfa_delete,
            _request_id: self._request_id,
        }
    }
}
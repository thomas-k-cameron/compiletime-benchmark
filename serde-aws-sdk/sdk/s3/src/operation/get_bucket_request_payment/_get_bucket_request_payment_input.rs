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
pub struct GetBucketRequestPaymentInput {
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl GetBucketRequestPaymentInput {
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl GetBucketRequestPaymentInput {
    /// Creates a new builder-style object to manufacture [`GetBucketRequestPaymentInput`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput).
    pub fn builder(
    ) -> crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentInputBuilder
    {
        crate::operation::get_bucket_request_payment::builders::GetBucketRequestPaymentInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput;
/// A builder for [`GetBucketRequestPaymentInput`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput).
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
pub struct GetBucketRequestPaymentInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl GetBucketRequestPaymentInputBuilder {
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket for which to get the payment request configuration</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.expected_bucket_owner = Some(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.expected_bucket_owner = input;
        self
    }
    /// Consumes the builder and constructs a [`GetBucketRequestPaymentInput`](crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_bucket_request_payment::GetBucketRequestPaymentInput {
                bucket: self.bucket,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}

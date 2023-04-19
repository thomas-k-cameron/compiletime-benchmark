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
pub struct GetBucketWebsiteInput {
    /// <p>The bucket name for which to get the website configuration.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl GetBucketWebsiteInput {
    /// <p>The bucket name for which to get the website configuration.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl GetBucketWebsiteInput {
    /// Creates a new builder-style object to manufacture [`GetBucketWebsiteInput`](crate::operation::get_bucket_website::GetBucketWebsiteInput).
    pub fn builder() -> crate::operation::get_bucket_website::builders::GetBucketWebsiteInputBuilder
    {
        crate::operation::get_bucket_website::builders::GetBucketWebsiteInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::get_bucket_website::GetBucketWebsiteInput;
/// A builder for [`GetBucketWebsiteInput`](crate::operation::get_bucket_website::GetBucketWebsiteInput).
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
pub struct GetBucketWebsiteInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl GetBucketWebsiteInputBuilder {
    /// <p>The bucket name for which to get the website configuration.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The bucket name for which to get the website configuration.</p>
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
    /// Consumes the builder and constructs a [`GetBucketWebsiteInput`](crate::operation::get_bucket_website::GetBucketWebsiteInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::get_bucket_website::GetBucketWebsiteInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::get_bucket_website::GetBucketWebsiteInput {
                bucket: self.bucket,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}
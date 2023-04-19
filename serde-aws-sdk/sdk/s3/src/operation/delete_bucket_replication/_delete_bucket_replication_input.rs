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
pub struct DeleteBucketReplicationInput {
    /// <p> The bucket name. </p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl DeleteBucketReplicationInput {
    /// <p> The bucket name. </p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl DeleteBucketReplicationInput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketReplicationInput`](crate::operation::delete_bucket_replication::DeleteBucketReplicationInput).
    pub fn builder(
    ) -> crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationInputBuilder
    {
        crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::delete_bucket_replication::DeleteBucketReplicationInput;
/// A builder for [`DeleteBucketReplicationInput`](crate::operation::delete_bucket_replication::DeleteBucketReplicationInput).
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
pub struct DeleteBucketReplicationInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl DeleteBucketReplicationInputBuilder {
    /// <p> The bucket name. </p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p> The bucket name. </p>
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
    /// Consumes the builder and constructs a [`DeleteBucketReplicationInput`](crate::operation::delete_bucket_replication::DeleteBucketReplicationInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::delete_bucket_replication::DeleteBucketReplicationInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::delete_bucket_replication::DeleteBucketReplicationInput {
                bucket: self.bucket,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}
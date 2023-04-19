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
pub struct PutBucketPolicyInput {
    /// <p>The name of the bucket.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    #[doc(hidden)]
    pub content_md5: std::option::Option<std::string::String>,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    #[doc(hidden)]
    pub confirm_remove_self_bucket_access: std::option::Option<bool>,
    /// <p>The bucket policy as a JSON document.</p>
    #[doc(hidden)]
    pub policy: std::option::Option<std::string::String>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl PutBucketPolicyInput {
    /// <p>The name of the bucket.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(&self) -> std::option::Option<&str> {
        self.content_md5.as_deref()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    pub fn confirm_remove_self_bucket_access(&self) -> std::option::Option<bool> {
        self.confirm_remove_self_bucket_access
    }
    /// <p>The bucket policy as a JSON document.</p>
    pub fn policy(&self) -> std::option::Option<&str> {
        self.policy.as_deref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl PutBucketPolicyInput {
    /// Creates a new builder-style object to manufacture [`PutBucketPolicyInput`](crate::operation::put_bucket_policy::PutBucketPolicyInput).
    pub fn builder() -> crate::operation::put_bucket_policy::builders::PutBucketPolicyInputBuilder {
        crate::operation::put_bucket_policy::builders::PutBucketPolicyInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::put_bucket_policy::PutBucketPolicyInput;
/// A builder for [`PutBucketPolicyInput`](crate::operation::put_bucket_policy::PutBucketPolicyInput).
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
pub struct PutBucketPolicyInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) content_md5: std::option::Option<std::string::String>,
    pub(crate) checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    pub(crate) confirm_remove_self_bucket_access: std::option::Option<bool>,
    pub(crate) policy: std::option::Option<std::string::String>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl PutBucketPolicyInputBuilder {
    /// <p>The name of the bucket.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.content_md5 = Some(input.into());
        self
    }
    /// <p>The MD5 hash of the request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.content_md5 = input;
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.checksum_algorithm = Some(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.checksum_algorithm = input;
        self
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    pub fn confirm_remove_self_bucket_access(mut self, input: bool) -> Self {
        self.confirm_remove_self_bucket_access = Some(input);
        self
    }
    /// <p>Set this parameter to true to confirm that you want to remove your permissions to change this bucket policy in the future.</p>
    pub fn set_confirm_remove_self_bucket_access(
        mut self,
        input: std::option::Option<bool>,
    ) -> Self {
        self.confirm_remove_self_bucket_access = input;
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    pub fn policy(mut self, input: impl Into<std::string::String>) -> Self {
        self.policy = Some(input.into());
        self
    }
    /// <p>The bucket policy as a JSON document.</p>
    pub fn set_policy(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.policy = input;
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
    /// Consumes the builder and constructs a [`PutBucketPolicyInput`](crate::operation::put_bucket_policy::PutBucketPolicyInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_bucket_policy::PutBucketPolicyInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::put_bucket_policy::PutBucketPolicyInput {
            bucket: self.bucket,
            content_md5: self.content_md5,
            checksum_algorithm: self.checksum_algorithm,
            confirm_remove_self_bucket_access: self.confirm_remove_self_bucket_access,
            policy: self.policy,
            expected_bucket_owner: self.expected_bucket_owner,
        })
    }
}
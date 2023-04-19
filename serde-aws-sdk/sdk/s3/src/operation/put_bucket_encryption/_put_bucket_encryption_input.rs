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
pub struct PutBucketEncryptionInput {
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or customer managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    #[doc(hidden)]
    pub content_md5: std::option::Option<std::string::String>,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    /// <p>Specifies the default server-side-encryption configuration.</p>
    #[doc(hidden)]
    pub server_side_encryption_configuration:
        std::option::Option<crate::types::ServerSideEncryptionConfiguration>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
}
impl PutBucketEncryptionInput {
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or customer managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(&self) -> std::option::Option<&str> {
        self.content_md5.as_deref()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn server_side_encryption_configuration(
        &self,
    ) -> std::option::Option<&crate::types::ServerSideEncryptionConfiguration> {
        self.server_side_encryption_configuration.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
}
impl PutBucketEncryptionInput {
    /// Creates a new builder-style object to manufacture [`PutBucketEncryptionInput`](crate::operation::put_bucket_encryption::PutBucketEncryptionInput).
    pub fn builder(
    ) -> crate::operation::put_bucket_encryption::builders::PutBucketEncryptionInputBuilder {
        crate::operation::put_bucket_encryption::builders::PutBucketEncryptionInputBuilder::default(
        )
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::put_bucket_encryption::PutBucketEncryptionInput;
/// A builder for [`PutBucketEncryptionInput`](crate::operation::put_bucket_encryption::PutBucketEncryptionInput).
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
pub struct PutBucketEncryptionInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) content_md5: std::option::Option<std::string::String>,
    pub(crate) checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
    pub(crate) server_side_encryption_configuration:
        std::option::Option<crate::types::ServerSideEncryptionConfiguration>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
}
impl PutBucketEncryptionInputBuilder {
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or customer managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>Specifies default encryption for a bucket using server-side encryption with Amazon S3-managed keys (SSE-S3) or customer managed keys (SSE-KMS). For information about the Amazon S3 default encryption feature, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-encryption.html">Amazon S3 Default Bucket Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.content_md5 = Some(input.into());
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the server-side encryption configuration.</p>
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
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn server_side_encryption_configuration(
        mut self,
        input: crate::types::ServerSideEncryptionConfiguration,
    ) -> Self {
        self.server_side_encryption_configuration = Some(input);
        self
    }
    /// <p>Specifies the default server-side-encryption configuration.</p>
    pub fn set_server_side_encryption_configuration(
        mut self,
        input: std::option::Option<crate::types::ServerSideEncryptionConfiguration>,
    ) -> Self {
        self.server_side_encryption_configuration = input;
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
    /// Consumes the builder and constructs a [`PutBucketEncryptionInput`](crate::operation::put_bucket_encryption::PutBucketEncryptionInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::put_bucket_encryption::PutBucketEncryptionInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(
            crate::operation::put_bucket_encryption::PutBucketEncryptionInput {
                bucket: self.bucket,
                content_md5: self.content_md5,
                checksum_algorithm: self.checksum_algorithm,
                server_side_encryption_configuration: self.server_side_encryption_configuration,
                expected_bucket_owner: self.expected_bucket_owner,
            },
        )
    }
}

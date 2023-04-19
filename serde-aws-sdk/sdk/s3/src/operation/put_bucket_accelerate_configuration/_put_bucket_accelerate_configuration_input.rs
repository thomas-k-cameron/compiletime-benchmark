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
pub struct PutBucketAccelerateConfigurationInput {
    /// <p>The name of the bucket for which the accelerate configuration is set.</p>
    #[doc(hidden)]
    pub bucket: std::option::Option<std::string::String>,
    /// <p>Container for setting the transfer acceleration state.</p>
    #[doc(hidden)]
    pub accelerate_configuration: std::option::Option<crate::types::AccelerateConfiguration>,
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    #[doc(hidden)]
    pub expected_bucket_owner: std::option::Option<std::string::String>,
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    #[doc(hidden)]
    pub checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
}
impl PutBucketAccelerateConfigurationInput {
    /// <p>The name of the bucket for which the accelerate configuration is set.</p>
    pub fn bucket(&self) -> std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>Container for setting the transfer acceleration state.</p>
    pub fn accelerate_configuration(
        &self,
    ) -> std::option::Option<&crate::types::AccelerateConfiguration> {
        self.accelerate_configuration.as_ref()
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(&self) -> std::option::Option<&str> {
        self.expected_bucket_owner.as_deref()
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(&self) -> std::option::Option<&crate::types::ChecksumAlgorithm> {
        self.checksum_algorithm.as_ref()
    }
}
impl PutBucketAccelerateConfigurationInput {
    /// Creates a new builder-style object to manufacture [`PutBucketAccelerateConfigurationInput`](crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput).
    pub fn builder() -> crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationInputBuilder{
        crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput;
/// A builder for [`PutBucketAccelerateConfigurationInput`](crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput).
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
pub struct PutBucketAccelerateConfigurationInputBuilder {
    pub(crate) bucket: std::option::Option<std::string::String>,
    pub(crate) accelerate_configuration: std::option::Option<crate::types::AccelerateConfiguration>,
    pub(crate) expected_bucket_owner: std::option::Option<std::string::String>,
    pub(crate) checksum_algorithm: std::option::Option<crate::types::ChecksumAlgorithm>,
}
impl PutBucketAccelerateConfigurationInputBuilder {
    /// <p>The name of the bucket for which the accelerate configuration is set.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.bucket = Some(input.into());
        self
    }
    /// <p>The name of the bucket for which the accelerate configuration is set.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>Container for setting the transfer acceleration state.</p>
    pub fn accelerate_configuration(
        mut self,
        input: crate::types::AccelerateConfiguration,
    ) -> Self {
        self.accelerate_configuration = Some(input);
        self
    }
    /// <p>Container for setting the transfer acceleration state.</p>
    pub fn set_accelerate_configuration(
        mut self,
        input: std::option::Option<crate::types::AccelerateConfiguration>,
    ) -> Self {
        self.accelerate_configuration = input;
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
    /// Consumes the builder and constructs a [`PutBucketAccelerateConfigurationInput`](crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput).
    pub fn build(self) -> Result<crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput, aws_smithy_http::operation::error::BuildError>{
        Ok(
            crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationInput {
                bucket: self.bucket
                ,
                accelerate_configuration: self.accelerate_configuration
                ,
                expected_bucket_owner: self.expected_bucket_owner
                ,
                checksum_algorithm: self.checksum_algorithm
                ,
            }
        )
    }
}

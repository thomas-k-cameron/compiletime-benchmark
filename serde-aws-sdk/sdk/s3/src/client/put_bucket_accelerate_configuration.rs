// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketAccelerateConfiguration`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::set_bucket): <p>The name of the bucket for which the accelerate configuration is set.</p>
    ///   - [`accelerate_configuration(AccelerateConfiguration)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::accelerate_configuration) / [`set_accelerate_configuration(Option<AccelerateConfiguration>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::set_accelerate_configuration): <p>Container for setting the transfer acceleration state.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    /// - On success, responds with [`PutBucketAccelerateConfigurationOutput`](crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutBucketAccelerateConfigurationError>`](crate::operation::put_bucket_accelerate_configuration::PutBucketAccelerateConfigurationError)
    pub fn put_bucket_accelerate_configuration(&self) -> crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder{
        crate::operation::put_bucket_accelerate_configuration::builders::PutBucketAccelerateConfigurationFluentBuilder::new(self.handle.clone())
    }
}

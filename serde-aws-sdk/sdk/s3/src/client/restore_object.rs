// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RestoreObject`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_bucket): <p>The bucket name containing the object to restore. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl Into<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_key): <p>Object key for which the action was initiated.</p>
    ///   - [`version_id(impl Into<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_version_id): <p>VersionId used to reference a specific version of the object.</p>
    ///   - [`restore_request(RestoreRequest)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::restore_request) / [`set_restore_request(Option<RestoreRequest>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_restore_request): <p>Container for restore job parameters.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::restore_object::builders::RestoreObjectFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`RestoreObjectOutput`](crate::operation::restore_object::RestoreObjectOutput) with field(s):
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::restore_object::RestoreObjectOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    ///   - [`restore_output_path(Option<String>)`](crate::operation::restore_object::RestoreObjectOutput::restore_output_path): <p>Indicates the path in the provided S3 output location where Select results will be restored to.</p>
    /// - On failure, responds with [`SdkError<RestoreObjectError>`](crate::operation::restore_object::RestoreObjectError)
    pub fn restore_object(
        &self,
    ) -> crate::operation::restore_object::builders::RestoreObjectFluentBuilder {
        crate::operation::restore_object::builders::RestoreObjectFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

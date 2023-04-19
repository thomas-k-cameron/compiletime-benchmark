// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObjectTagging`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_bucket): <p>The bucket name containing the object for which to get the tagging information. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`key(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_key): <p>Object key for which to get the tagging information.</p>
    ///   - [`version_id(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_version_id): <p>The versionId of the object for which to get the tagging information.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// - On success, responds with [`GetObjectTaggingOutput`](crate::operation::get_object_tagging::GetObjectTaggingOutput) with field(s):
    ///   - [`version_id(Option<String>)`](crate::operation::get_object_tagging::GetObjectTaggingOutput::version_id): <p>The versionId of the object for which you got the tagging information.</p>
    ///   - [`tag_set(Option<Vec<Tag>>)`](crate::operation::get_object_tagging::GetObjectTaggingOutput::tag_set): <p>Contains the tag set.</p>
    /// - On failure, responds with [`SdkError<GetObjectTaggingError>`](crate::operation::get_object_tagging::GetObjectTaggingError)
    pub fn get_object_tagging(
        &self,
    ) -> crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder {
        crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

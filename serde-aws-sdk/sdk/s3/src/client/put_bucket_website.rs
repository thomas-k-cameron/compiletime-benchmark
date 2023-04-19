// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketWebsite`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::set_bucket): <p>The bucket name.</p>
    ///   - [`content_md5(impl Into<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::set_content_md5): <p>The base64-encoded 128-bit MD5 digest of the data. You must use this header as a message integrity check to verify that the request body was not corrupted in transit. For more information, see <a href="http://www.ietf.org/rfc/rfc1864.txt">RFC 1864</a>.</p>  <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    ///   - [`checksum_algorithm(ChecksumAlgorithm)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::checksum_algorithm) / [`set_checksum_algorithm(Option<ChecksumAlgorithm>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::set_checksum_algorithm): <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    ///   - [`website_configuration(WebsiteConfiguration)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::website_configuration) / [`set_website_configuration(Option<WebsiteConfiguration>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::set_website_configuration): <p>Container for the request.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`PutBucketWebsiteOutput`](crate::operation::put_bucket_website::PutBucketWebsiteOutput)
    /// - On failure, responds with [`SdkError<PutBucketWebsiteError>`](crate::operation::put_bucket_website::PutBucketWebsiteError)
    pub fn put_bucket_website(
        &self,
    ) -> crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder {
        crate::operation::put_bucket_website::builders::PutBucketWebsiteFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
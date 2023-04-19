// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UploadPartCopy`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_bucket): <p>The bucket name.</p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`copy_source(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source) / [`set_copy_source(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source): <p>Specifies the source object for the copy operation. You specify the value in one of two formats, depending on whether you want to access the source object through an <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/access-points.html">access point</a>:</p>  <ul>   <li> <p>For objects not accessed through an access point, specify the name of the source bucket and key of the source object, separated by a slash (/). For example, to copy the object <code>reports/january.pdf</code> from the bucket <code>awsexamplebucket</code>, use <code>awsexamplebucket/reports/january.pdf</code>. The value must be URL-encoded.</p> </li>   <li> <p>For objects accessed through access points, specify the Amazon Resource Name (ARN) of the object as accessed through the access point, in the format <code>arn:aws:s3:     <region>      :      <account-id>       :accesspoint/       <access-point-name>        /object/        <key></key>       </access-point-name>      </account-id>     </region></code>. For example, to copy the object <code>reports/january.pdf</code> through access point <code>my-access-point</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3:us-west-2:123456789012:accesspoint/my-access-point/object/reports/january.pdf</code>. The value must be URL encoded.</p> <note>     <p>Amazon S3 supports copy operations using access points only when the source and destination buckets are in the same Amazon Web Services Region.</p>    </note> <p>Alternatively, for objects accessed through Amazon S3 on Outposts, specify the ARN of the object as accessed in the format <code>arn:aws:s3-outposts:     <region>      :      <account-id>       :outpost/       <outpost-id>        /object/        <key></key>       </outpost-id>      </account-id>     </region></code>. For example, to copy the object <code>reports/january.pdf</code> through outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/object/reports/january.pdf</code>. The value must be URL-encoded. </p> </li>  </ul>  <p>To copy a specific version of an object, append <code>?versionId=   <version-id></version-id></code> to the value (for example, <code>awsexamplebucket/reports/january.pdf?versionId=QUpfdndhfd8438MNFDN93jdnJFkdmqnh893</code>). If you don't specify a version ID, Amazon S3 copies the latest version of the source object.</p>
    ///   - [`copy_source_if_match(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_if_match) / [`set_copy_source_if_match(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_if_match): <p>Copies the object if its entity tag (ETag) matches the specified tag.</p>
    ///   - [`copy_source_if_modified_since(DateTime)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_if_modified_since) / [`set_copy_source_if_modified_since(Option<DateTime>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_if_modified_since): <p>Copies the object if it has been modified since the specified time.</p>
    ///   - [`copy_source_if_none_match(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_if_none_match) / [`set_copy_source_if_none_match(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_if_none_match): <p>Copies the object if its entity tag (ETag) is different than the specified ETag.</p>
    ///   - [`copy_source_if_unmodified_since(DateTime)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_if_unmodified_since) / [`set_copy_source_if_unmodified_since(Option<DateTime>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_if_unmodified_since): <p>Copies the object if it hasn't been modified since the specified time.</p>
    ///   - [`copy_source_range(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_range) / [`set_copy_source_range(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_range): <p>The range of bytes to copy from the source object. The range value must use the form bytes=first-last, where the first and last are the zero-based byte offsets to copy. For example, bytes=0-9 indicates that you want to copy the first 10 bytes of the source. You can copy a range only if the source object is greater than 5 MB.</p>
    ///   - [`key(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_key): <p>Object key for which the multipart upload was initiated.</p>
    ///   - [`part_number(i32)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::part_number) / [`set_part_number(Option<i32>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_part_number): <p>Part number of part being copied. This is a positive integer between 1 and 10,000.</p>
    ///   - [`upload_id(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::upload_id) / [`set_upload_id(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_upload_id): <p>Upload ID identifying the multipart upload whose part is being copied.</p>
    ///   - [`sse_customer_algorithm(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::sse_customer_algorithm) / [`set_sse_customer_algorithm(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_sse_customer_algorithm): <p>Specifies the algorithm to use to when encrypting the object (for example, AES256).</p>
    ///   - [`sse_customer_key(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::sse_customer_key) / [`set_sse_customer_key(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_sse_customer_key): <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header. This must be the same encryption key specified in the initiate multipart upload request.</p>
    ///   - [`sse_customer_key_md5(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::sse_customer_key_md5) / [`set_sse_customer_key_md5(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_sse_customer_key_md5): <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    ///   - [`copy_source_sse_customer_algorithm(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_sse_customer_algorithm) / [`set_copy_source_sse_customer_algorithm(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_sse_customer_algorithm): <p>Specifies the algorithm to use when decrypting the source object (for example, AES256).</p>
    ///   - [`copy_source_sse_customer_key(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_sse_customer_key) / [`set_copy_source_sse_customer_key(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_sse_customer_key): <p>Specifies the customer-provided encryption key for Amazon S3 to use to decrypt the source object. The encryption key provided in this header must be one that was used when the source object was created.</p>
    ///   - [`copy_source_sse_customer_key_md5(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::copy_source_sse_customer_key_md5) / [`set_copy_source_sse_customer_key_md5(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_copy_source_sse_customer_key_md5): <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected destination bucket owner. If the destination bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    ///   - [`expected_source_bucket_owner(impl Into<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::expected_source_bucket_owner) / [`set_expected_source_bucket_owner(Option<String>)`](crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::set_expected_source_bucket_owner): <p>The account ID of the expected source bucket owner. If the source bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput) with field(s):
    ///   - [`copy_source_version_id(Option<String>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::copy_source_version_id): <p>The version of the source object that was copied, if you have enabled versioning on the source bucket.</p>
    ///   - [`copy_part_result(Option<CopyPartResult>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::copy_part_result): <p>Container for all response elements.</p>
    ///   - [`server_side_encryption(Option<ServerSideEncryption>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::server_side_encryption): <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    ///   - [`sse_customer_algorithm(Option<String>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::sse_customer_algorithm): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    ///   - [`sse_customer_key_md5(Option<String>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::sse_customer_key_md5): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    ///   - [`ssekms_key_id(Option<String>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::ssekms_key_id): <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    ///   - [`bucket_key_enabled(bool)`](crate::operation::upload_part_copy::UploadPartCopyOutput::bucket_key_enabled): <p>Indicates whether the multipart upload uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::upload_part_copy::UploadPartCopyOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    /// - On failure, responds with [`SdkError<UploadPartCopyError>`](crate::operation::upload_part_copy::UploadPartCopyError)
    pub fn upload_part_copy(
        &self,
    ) -> crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder {
        crate::operation::upload_part_copy::builders::UploadPartCopyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
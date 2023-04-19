// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObject`](crate::operation::get_object::builders::GetObjectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_bucket): <p>The bucket name containing the object. </p>  <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>  <p>When using an Object Lambda access point the hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-object-lambda.<i>Region</i>.amazonaws.com.</p>  <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`if_match(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::if_match) / [`set_if_match(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_if_match): <p>Return the object only if its entity tag (ETag) is the same as the one specified; otherwise, return a 412 (precondition failed) error.</p>
    ///   - [`if_modified_since(DateTime)`](crate::operation::get_object::builders::GetObjectFluentBuilder::if_modified_since) / [`set_if_modified_since(Option<DateTime>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_if_modified_since): <p>Return the object only if it has been modified since the specified time; otherwise, return a 304 (not modified) error.</p>
    ///   - [`if_none_match(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::if_none_match) / [`set_if_none_match(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_if_none_match): <p>Return the object only if its entity tag (ETag) is different from the one specified; otherwise, return a 304 (not modified) error.</p>
    ///   - [`if_unmodified_since(DateTime)`](crate::operation::get_object::builders::GetObjectFluentBuilder::if_unmodified_since) / [`set_if_unmodified_since(Option<DateTime>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_if_unmodified_since): <p>Return the object only if it has not been modified since the specified time; otherwise, return a 412 (precondition failed) error.</p>
    ///   - [`key(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::key) / [`set_key(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_key): <p>Key of the object to get.</p>
    ///   - [`range(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::range) / [`set_range(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_range): <p>Downloads the specified range bytes of an object. For more information about the HTTP Range header, see <a href="https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35">https://www.w3.org/Protocols/rfc2616/rfc2616-sec14.html#sec14.35</a>.</p> <note>   <p>Amazon S3 doesn't support retrieving multiple ranges of data per <code>GET</code> request.</p>  </note>
    ///   - [`response_cache_control(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_cache_control) / [`set_response_cache_control(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_cache_control): <p>Sets the <code>Cache-Control</code> header of the response.</p>
    ///   - [`response_content_disposition(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_content_disposition) / [`set_response_content_disposition(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_content_disposition): <p>Sets the <code>Content-Disposition</code> header of the response</p>
    ///   - [`response_content_encoding(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_content_encoding) / [`set_response_content_encoding(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_content_encoding): <p>Sets the <code>Content-Encoding</code> header of the response.</p>
    ///   - [`response_content_language(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_content_language) / [`set_response_content_language(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_content_language): <p>Sets the <code>Content-Language</code> header of the response.</p>
    ///   - [`response_content_type(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_content_type) / [`set_response_content_type(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_content_type): <p>Sets the <code>Content-Type</code> header of the response.</p>
    ///   - [`response_expires(DateTime)`](crate::operation::get_object::builders::GetObjectFluentBuilder::response_expires) / [`set_response_expires(Option<DateTime>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_response_expires): <p>Sets the <code>Expires</code> header of the response.</p>
    ///   - [`version_id(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_version_id): <p>VersionId used to reference a specific version of the object.</p>
    ///   - [`sse_customer_algorithm(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::sse_customer_algorithm) / [`set_sse_customer_algorithm(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_sse_customer_algorithm): <p>Specifies the algorithm to use to when decrypting the object (for example, AES256).</p>
    ///   - [`sse_customer_key(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::sse_customer_key) / [`set_sse_customer_key(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_sse_customer_key): <p>Specifies the customer-provided encryption key for Amazon S3 used to encrypt the data. This value is used to decrypt the object when recovering it and must match the one used when storing the data. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm</code> header.</p>
    ///   - [`sse_customer_key_md5(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::sse_customer_key_md5) / [`set_sse_customer_key_md5(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_sse_customer_key_md5): <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    ///   - [`request_payer(RequestPayer)`](crate::operation::get_object::builders::GetObjectFluentBuilder::request_payer) / [`set_request_payer(Option<RequestPayer>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_request_payer): <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`part_number(i32)`](crate::operation::get_object::builders::GetObjectFluentBuilder::part_number) / [`set_part_number(Option<i32>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_part_number): <p>Part number of the object being read. This is a positive integer between 1 and 10,000. Effectively performs a 'ranged' GET request for the part specified. Useful for downloading just a part of an object.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    ///   - [`checksum_mode(ChecksumMode)`](crate::operation::get_object::builders::GetObjectFluentBuilder::checksum_mode) / [`set_checksum_mode(Option<ChecksumMode>)`](crate::operation::get_object::builders::GetObjectFluentBuilder::set_checksum_mode): <p>To retrieve the checksum, this mode must be enabled.</p>
    /// - On success, responds with [`GetObjectOutput`](crate::operation::get_object::GetObjectOutput) with field(s):
    ///   - [`body(ByteStream)`](crate::operation::get_object::GetObjectOutput::body): <p>Object data.</p>
    ///   - [`delete_marker(bool)`](crate::operation::get_object::GetObjectOutput::delete_marker): <p>Specifies whether the object retrieved was (true) or was not (false) a Delete Marker. If false, this response header does not appear in the response.</p>
    ///   - [`accept_ranges(Option<String>)`](crate::operation::get_object::GetObjectOutput::accept_ranges): <p>Indicates that a range of bytes was specified.</p>
    ///   - [`expiration(Option<String>)`](crate::operation::get_object::GetObjectOutput::expiration): <p>If the object expiration is configured (see PUT Bucket lifecycle), the response includes this header. It includes the <code>expiry-date</code> and <code>rule-id</code> key-value pairs providing object expiration information. The value of the <code>rule-id</code> is URL-encoded.</p>
    ///   - [`restore(Option<String>)`](crate::operation::get_object::GetObjectOutput::restore): <p>Provides information about object restoration action and expiration time of the restored object copy.</p>
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::get_object::GetObjectOutput::last_modified): <p>Creation date of the object.</p>
    ///   - [`content_length(i64)`](crate::operation::get_object::GetObjectOutput::content_length): <p>Size of the body in bytes.</p>
    ///   - [`e_tag(Option<String>)`](crate::operation::get_object::GetObjectOutput::e_tag): <p>An entity tag (ETag) is an opaque identifier assigned by a web server to a specific version of a resource found at a URL.</p>
    ///   - [`checksum_crc32(Option<String>)`](crate::operation::get_object::GetObjectOutput::checksum_crc32): <p>The base64-encoded, 32-bit CRC32 checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_crc32_c(Option<String>)`](crate::operation::get_object::GetObjectOutput::checksum_crc32_c): <p>The base64-encoded, 32-bit CRC32C checksum of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha1(Option<String>)`](crate::operation::get_object::GetObjectOutput::checksum_sha1): <p>The base64-encoded, 160-bit SHA-1 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`checksum_sha256(Option<String>)`](crate::operation::get_object::GetObjectOutput::checksum_sha256): <p>The base64-encoded, 256-bit SHA-256 digest of the object. This will only be present if it was uploaded with the object. With multipart uploads, this may not be a checksum value of the object. For more information about how checksums are calculated with multipart uploads, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html#large-object-checksums"> Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    ///   - [`missing_meta(i32)`](crate::operation::get_object::GetObjectOutput::missing_meta): <p>This is set to the number of metadata entries not returned in <code>x-amz-meta</code> headers. This can happen if you create metadata using an API like SOAP that supports more flexible metadata than the REST API. For example, using SOAP, you can create metadata whose values are not legal HTTP headers.</p>
    ///   - [`version_id(Option<String>)`](crate::operation::get_object::GetObjectOutput::version_id): <p>Version of the object.</p>
    ///   - [`cache_control(Option<String>)`](crate::operation::get_object::GetObjectOutput::cache_control): <p>Specifies caching behavior along the request/reply chain.</p>
    ///   - [`content_disposition(Option<String>)`](crate::operation::get_object::GetObjectOutput::content_disposition): <p>Specifies presentational information for the object.</p>
    ///   - [`content_encoding(Option<String>)`](crate::operation::get_object::GetObjectOutput::content_encoding): <p>Specifies what content encodings have been applied to the object and thus what decoding mechanisms must be applied to obtain the media-type referenced by the Content-Type header field.</p>
    ///   - [`content_language(Option<String>)`](crate::operation::get_object::GetObjectOutput::content_language): <p>The language the content is in.</p>
    ///   - [`content_range(Option<String>)`](crate::operation::get_object::GetObjectOutput::content_range): <p>The portion of the object returned in the response.</p>
    ///   - [`content_type(Option<String>)`](crate::operation::get_object::GetObjectOutput::content_type): <p>A standard MIME type describing the format of the object data.</p>
    ///   - [`expires(Option<DateTime>)`](crate::operation::get_object::GetObjectOutput::expires): <p>The date and time at which the object is no longer cacheable.</p>
    ///   - [`website_redirect_location(Option<String>)`](crate::operation::get_object::GetObjectOutput::website_redirect_location): <p>If the bucket is configured as a website, redirects requests for this object to another object in the same bucket or to an external URL. Amazon S3 stores the value of this header in the object metadata.</p>
    ///   - [`server_side_encryption(Option<ServerSideEncryption>)`](crate::operation::get_object::GetObjectOutput::server_side_encryption): <p>The server-side encryption algorithm used when storing this object in Amazon S3 (for example, AES256, aws:kms).</p>
    ///   - [`metadata(Option<HashMap<String, String>>)`](crate::operation::get_object::GetObjectOutput::metadata): <p>A map of metadata to store with the object in S3.</p>
    ///   - [`sse_customer_algorithm(Option<String>)`](crate::operation::get_object::GetObjectOutput::sse_customer_algorithm): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header confirming the encryption algorithm used.</p>
    ///   - [`sse_customer_key_md5(Option<String>)`](crate::operation::get_object::GetObjectOutput::sse_customer_key_md5): <p>If server-side encryption with a customer-provided encryption key was requested, the response will include this header to provide round-trip message integrity verification of the customer-provided encryption key.</p>
    ///   - [`ssekms_key_id(Option<String>)`](crate::operation::get_object::GetObjectOutput::ssekms_key_id): <p>If present, specifies the ID of the Amazon Web Services Key Management Service (Amazon Web Services KMS) symmetric customer managed key that was used for the object.</p>
    ///   - [`bucket_key_enabled(bool)`](crate::operation::get_object::GetObjectOutput::bucket_key_enabled): <p>Indicates whether the object uses an S3 Bucket Key for server-side encryption with Amazon Web Services KMS (SSE-KMS).</p>
    ///   - [`storage_class(Option<StorageClass>)`](crate::operation::get_object::GetObjectOutput::storage_class): <p>Provides storage class information of the object. Amazon S3 returns this header for all objects except for S3 Standard storage class objects.</p>
    ///   - [`request_charged(Option<RequestCharged>)`](crate::operation::get_object::GetObjectOutput::request_charged): <p>If present, indicates that the requester was successfully charged for the request.</p>
    ///   - [`replication_status(Option<ReplicationStatus>)`](crate::operation::get_object::GetObjectOutput::replication_status): <p>Amazon S3 can return this if your request involves a bucket that is either a source or destination in a replication rule.</p>
    ///   - [`parts_count(i32)`](crate::operation::get_object::GetObjectOutput::parts_count): <p>The count of parts this object has. This value is only returned if you specify <code>partNumber</code> in your request and the object was uploaded as a multipart upload.</p>
    ///   - [`tag_count(i32)`](crate::operation::get_object::GetObjectOutput::tag_count): <p>The number of tags, if any, on the object.</p>
    ///   - [`object_lock_mode(Option<ObjectLockMode>)`](crate::operation::get_object::GetObjectOutput::object_lock_mode): <p>The Object Lock mode currently in place for this object.</p>
    ///   - [`object_lock_retain_until_date(Option<DateTime>)`](crate::operation::get_object::GetObjectOutput::object_lock_retain_until_date): <p>The date and time when this object's Object Lock will expire.</p>
    ///   - [`object_lock_legal_hold_status(Option<ObjectLockLegalHoldStatus>)`](crate::operation::get_object::GetObjectOutput::object_lock_legal_hold_status): <p>Indicates whether this object has an active legal hold. This field is only returned if you have permission to view an object's legal hold status. </p>
    /// - On failure, responds with [`SdkError<GetObjectError>`](crate::operation::get_object::GetObjectError)
    pub fn get_object(&self) -> crate::operation::get_object::builders::GetObjectFluentBuilder {
        crate::operation::get_object::builders::GetObjectFluentBuilder::new(self.handle.clone())
    }
}

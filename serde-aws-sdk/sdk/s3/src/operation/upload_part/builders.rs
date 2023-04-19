// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::upload_part::_upload_part_output::UploadPartOutputBuilder;

pub use crate::operation::upload_part::_upload_part_input::UploadPartInputBuilder;

/// Fluent builder constructing a request to `UploadPart`.
///
/// <p>Uploads a part in a multipart upload.</p> <note>
/// <p>In this operation, you provide part data in your request. However, you have an option to specify your existing Amazon S3 object as a data source for the part you are uploading. To upload a part from an existing object, you use the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_UploadPartCopy.html">UploadPartCopy</a> operation. </p>
/// </note>
/// <p>You must initiate a multipart upload (see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>) before you can upload any part. In response to your initiate request, Amazon S3 returns an upload ID, a unique identifier, that you must include in your upload part request.</p>
/// <p>Part numbers can be any number from 1 to 10,000, inclusive. A part number uniquely identifies a part and also defines its position within the object being created. If you upload a new part using the same part number that was used with a previous part, the previously uploaded part is overwritten.</p>
/// <p>For information about maximum and minimum part sizes and other multipart upload specifications, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/qfacts.html">Multipart upload limits</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>To ensure that data is not corrupted when traversing the network, specify the <code>Content-MD5</code> header in the upload part request. Amazon S3 checks the part data against the provided MD5 value. If they do not match, Amazon S3 returns an error. </p>
/// <p>If the upload request is signed with Signature Version 4, then Amazon Web Services S3 uses the <code>x-amz-content-sha256</code> header as a checksum instead of <code>Content-MD5</code>. For more information see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-auth-using-authorization-header.html">Authenticating Requests: Using the Authorization Header (Amazon Web Services Signature Version 4)</a>. </p>
/// <p> <b>Note:</b> After you initiate multipart upload and upload one or more parts, you must either complete or abort multipart upload in order to stop getting charged for storage of the uploaded parts. Only after you either complete or abort multipart upload, Amazon S3 frees up the parts storage and stops charging you for the parts storage.</p>
/// <p>For more information on multipart uploads, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html">Multipart Upload Overview</a> in the <i>Amazon S3 User Guide </i>.</p>
/// <p>For information on the permissions required to use the multipart upload API, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuAndPermissions.html">Multipart Upload and Permissions</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>You can optionally request server-side encryption where Amazon S3 encrypts your data as it writes it to disks in its data centers and decrypts it for you when you access it. You have the option of providing your own encryption key, or you can use the Amazon Web Services managed encryption keys. If you choose to provide your own encryption key, the request headers you provide in the request must match the headers you used in the request to initiate the upload by using <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>. For more information, go to <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingServerSideEncryption.html">Using Server-Side Encryption</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>Server-side encryption is supported by the S3 Multipart Upload actions. Unless you are using a customer-provided encryption key, you don't need to specify the encryption parameters in each UploadPart request. Instead, you only need to specify the server-side encryption parameters in the initial Initiate Multipart request. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a>.</p>
/// <p>If you requested server-side encryption using a customer-provided encryption key in your initiate multipart upload request, you must provide identical encryption information in each part upload using the following headers.</p>
/// <ul>
/// <li> <p>x-amz-server-side-encryption-customer-algorithm</p> </li>
/// <li> <p>x-amz-server-side-encryption-customer-key</p> </li>
/// <li> <p>x-amz-server-side-encryption-customer-key-MD5</p> </li>
/// </ul>
/// <p class="title"> <b>Special Errors</b> </p>
/// <ul>
/// <li>
/// <ul>
/// <li> <p> <i>Code: NoSuchUpload</i> </p> </li>
/// <li> <p> <i>Cause: The specified multipart upload does not exist. The upload ID might be invalid, or the multipart upload might have been aborted or completed.</i> </p> </li>
/// <li> <p> <i> HTTP Status Code: 404 Not Found </i> </p> </li>
/// <li> <p> <i>SOAP Fault Code Prefix: Client</i> </p> </li>
/// </ul> </li>
/// </ul>
/// <p class="title"> <b>Related Resources</b> </p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateMultipartUpload.html">CreateMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CompleteMultipartUpload.html">CompleteMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_AbortMultipartUpload.html">AbortMultipartUpload</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListParts.html">ListParts</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListMultipartUploads.html">ListMultipartUploads</a> </p> </li>
/// </ul>
#[derive(std::fmt::Debug)]
pub struct UploadPartFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::upload_part::builders::UploadPartInputBuilder,
}
impl UploadPartFluentBuilder {
    /// Creates a new `UploadPart`.
    pub(crate) fn new(handle: std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: Default::default(),
        }
    }

    /// Consume this builder, creating a customizable operation that can be modified before being
    /// sent. The operation's inner [http::Request] can be modified as well.
    pub async fn customize(
        self,
    ) -> std::result::Result<
        crate::client::customize::CustomizableOperation<
            crate::operation::upload_part::UploadPart,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::upload_part::UploadPartError>,
    > {
        let handle = self.handle.clone();
        let operation = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        Ok(crate::client::customize::CustomizableOperation { handle, operation })
    }

    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> std::result::Result<
        crate::operation::upload_part::UploadPartOutput,
        aws_smithy_http::result::SdkError<crate::operation::upload_part::UploadPartError>,
    > {
        let op = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?
            .make_operation(&self.handle.conf)
            .await
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        self.handle.client.call(op).await
    }
    #[cfg(aws_sdk_unstable)]
    /// This function replaces the parameter with new one.
    /// It is useful when you want to replace the existing data with de-serialized data.
    /// ```compile_fail
    /// let result_future = async {
    ///     let deserialized_parameters: crate::operation::upload_part::builders::UploadPartInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.upload_part().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::upload_part::builders::UploadPartInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    ///
    /// Creates a presigned request for this operation.
    ///
    /// The `presigning_config` provides additional presigning-specific config values, such as the
    /// amount of time the request should be valid for after creation.
    ///
    /// Presigned requests can be given to other users or applications to access a resource or perform
    /// an operation without having access to the AWS security credentials.
    ///
    pub async fn presigned(
        self,
        presigning_config: crate::presigning::PresigningConfig,
    ) -> Result<
        crate::presigning::PresignedRequest,
        aws_smithy_http::result::SdkError<crate::operation::upload_part::UploadPartError>,
    > {
        let input = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        input.presigned(&self.handle.conf, presigning_config).await
    }
    /// <p>Object data.</p>
    pub fn body(mut self, input: aws_smithy_http::byte_stream::ByteStream) -> Self {
        self.inner = self.inner.body(input);
        self
    }
    /// <p>Object data.</p>
    pub fn set_body(
        mut self,
        input: std::option::Option<aws_smithy_http::byte_stream::ByteStream>,
    ) -> Self {
        self.inner = self.inner.set_body(input);
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket to which the multipart upload was initiated.</p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub fn content_length(mut self, input: i64) -> Self {
        self.inner = self.inner.content_length(input);
        self
    }
    /// <p>Size of the body in bytes. This parameter is useful when the size of the body cannot be determined automatically.</p>
    pub fn set_content_length(mut self, input: std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_content_length(input);
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI. This parameter is required if object lock parameters are specified.</p>
    pub fn content_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The base64-encoded 128-bit MD5 digest of the part data. This parameter is auto-populated when using the command from the CLI. This parameter is required if object lock parameters are specified.</p>
    pub fn set_content_md5(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    /// <p>This checksum algorithm must be the same for all parts and it match the checksum value supplied in the <code>CreateMultipartUpload</code> request.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    /// <p>This checksum algorithm must be the same for all parts and it match the checksum value supplied in the <code>CreateMultipartUpload</code> request.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.checksum_crc32(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32 checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_checksum_crc32(input);
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_crc32_c(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.checksum_crc32_c(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 32-bit CRC32C checksum of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_crc32_c(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_checksum_crc32_c(input);
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha1(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.checksum_sha1(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 160-bit SHA-1 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha1(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_checksum_sha1(input);
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn checksum_sha256(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.checksum_sha256(input.into());
        self
    }
    /// <p>This header can be used as a data integrity check to verify that the data received is the same data that was originally sent. This header specifies the base64-encoded, 256-bit SHA-256 digest of the object. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_checksum_sha256(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_checksum_sha256(input);
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>Object key for which the multipart upload was initiated.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>Part number of part being uploaded. This is a positive integer between 1 and 10,000.</p>
    pub fn part_number(mut self, input: i32) -> Self {
        self.inner = self.inner.part_number(input);
        self
    }
    /// <p>Part number of part being uploaded. This is a positive integer between 1 and 10,000.</p>
    pub fn set_part_number(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_part_number(input);
        self
    }
    /// <p>Upload ID identifying the multipart upload whose part is being uploaded.</p>
    pub fn upload_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    /// <p>Upload ID identifying the multipart upload whose part is being uploaded.</p>
    pub fn set_upload_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
    /// <p>Specifies the algorithm to use to when encrypting the object (for example, AES256).</p>
    pub fn sse_customer_algorithm(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_algorithm(input.into());
        self
    }
    /// <p>Specifies the algorithm to use to when encrypting the object (for example, AES256).</p>
    pub fn set_sse_customer_algorithm(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sse_customer_algorithm(input);
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm header</code>. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub fn sse_customer_key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key(input.into());
        self
    }
    /// <p>Specifies the customer-provided encryption key for Amazon S3 to use in encrypting data. This value is used to store the object and then it is discarded; Amazon S3 does not store the encryption key. The key must be appropriate for use with the algorithm specified in the <code>x-amz-server-side-encryption-customer-algorithm header</code>. This must be the same encryption key specified in the initiate multipart upload request.</p>
    pub fn set_sse_customer_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_sse_customer_key(input);
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn sse_customer_key_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.sse_customer_key_md5(input.into());
        self
    }
    /// <p>Specifies the 128-bit MD5 digest of the encryption key according to RFC 1321. Amazon S3 uses this header for a message integrity check to ensure that the encryption key was transmitted without error.</p>
    pub fn set_sse_customer_key_md5(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_sse_customer_key_md5(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn request_payer(mut self, input: crate::types::RequestPayer) -> Self {
        self.inner = self.inner.request_payer(input);
        self
    }
    /// <p>Confirms that the requester knows that they will be charged for the request. Bucket owners need not specify this parameter in their requests. For information about downloading objects from Requester Pays buckets, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/ObjectsinRequesterPaysBuckets.html">Downloading Objects in Requester Pays Buckets</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_request_payer(
        mut self,
        input: std::option::Option<crate::types::RequestPayer>,
    ) -> Self {
        self.inner = self.inner.set_request_payer(input);
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn expected_bucket_owner(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.expected_bucket_owner(input.into());
        self
    }
    /// <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    pub fn set_expected_bucket_owner(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_expected_bucket_owner(input);
        self
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_logging::_put_bucket_logging_output::PutBucketLoggingOutputBuilder;

pub use crate::operation::put_bucket_logging::_put_bucket_logging_input::PutBucketLoggingInputBuilder;

/// Fluent builder constructing a request to `PutBucketLogging`.
///
/// <p>Set the logging parameters for a bucket and to specify permissions for who can view and modify the logging parameters. All logs are saved to buckets in the same Amazon Web Services Region as the source bucket. To set the logging status of a bucket, you must be the bucket owner.</p>
/// <p>The bucket owner is automatically granted FULL_CONTROL to all logs. You use the <code>Grantee</code> request element to grant access to other people. The <code>Permissions</code> request element specifies the kind of access the grantee has to the logs.</p> <important>
/// <p>If the target bucket for log delivery uses the bucket owner enforced setting for S3 Object Ownership, you can't use the <code>Grantee</code> request element to grant access to others. Permissions can only be granted using policies. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/enable-server-access-logging.html#grant-log-delivery-permissions-general">Permissions for server access log delivery</a> in the <i>Amazon S3 User Guide</i>.</p>
/// </important>
/// <p> <b>Grantee Values</b> </p>
/// <p>You can specify the person (grantee) to whom you're assigning access rights (using request elements) in the following ways:</p>
/// <ul>
/// <li> <p>By the person's ID:</p> <p> <code>
/// <grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="CanonicalUser">
/// <id>
/// &lt;&gt;ID&lt;&gt;
/// </id>
/// <displayname>
/// &lt;&gt;GranteesEmail&lt;&gt;
/// </displayname>
/// </grantee></code> </p> <p>DisplayName is optional and ignored in the request.</p> </li>
/// <li> <p>By Email address:</p> <p> <code>
/// <grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="AmazonCustomerByEmail">
/// <emailaddress>
/// &lt;&gt;Grantees@email.com&lt;&gt;
/// </emailaddress>
/// </grantee></code> </p> <p>The grantee is resolved to the CanonicalUser and, in a response to a GET Object acl request, appears as the CanonicalUser.</p> </li>
/// <li> <p>By URI:</p> <p> <code>
/// <grantee xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:type="Group">
/// <uri>
/// &lt;&gt;http://acs.amazonaws.com/groups/global/AuthenticatedUsers&lt;&gt;
/// </uri>
/// </grantee></code> </p> </li>
/// </ul>
/// <p>To enable logging, you use LoggingEnabled and its children request elements. To disable logging, you use an empty BucketLoggingStatus request element:</p>
/// <p> <code>
/// <bucketloggingstatus xmlns="http://doc.s3.amazonaws.com/2006-03-01" /></code> </p>
/// <p>For more information about server access logging, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/ServerLogs.html">Server Access Logging</a> in the <i>Amazon S3 User Guide</i>. </p>
/// <p>For more information about creating a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a>. For more information about returning the logging status of a bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLogging.html">GetBucketLogging</a>.</p>
/// <p>The following operations are related to <code>PutBucketLogging</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_DeleteBucket.html">DeleteBucket</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_CreateBucket.html">CreateBucket</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetBucketLogging.html">GetBucketLogging</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutBucketLoggingFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_logging::builders::PutBucketLoggingInputBuilder,
}
impl PutBucketLoggingFluentBuilder {
    /// Creates a new `PutBucketLogging`.
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
            crate::operation::put_bucket_logging::PutBucketLogging,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_logging::PutBucketLoggingError,
        >,
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
        crate::operation::put_bucket_logging::PutBucketLoggingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_logging::PutBucketLoggingError,
        >,
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
    ///     let deserialized_parameters: crate::operation::put_bucket_logging::builders::PutBucketLoggingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_bucket_logging().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_bucket_logging::builders::PutBucketLoggingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the bucket for which to set the logging parameters.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The name of the bucket for which to set the logging parameters.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Container for logging status information.</p>
    pub fn bucket_logging_status(mut self, input: crate::types::BucketLoggingStatus) -> Self {
        self.inner = self.inner.bucket_logging_status(input);
        self
    }
    /// <p>Container for logging status information.</p>
    pub fn set_bucket_logging_status(
        mut self,
        input: std::option::Option<crate::types::BucketLoggingStatus>,
    ) -> Self {
        self.inner = self.inner.set_bucket_logging_status(input);
        self
    }
    /// <p>The MD5 hash of the <code>PutBucketLogging</code> request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn content_md5(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.content_md5(input.into());
        self
    }
    /// <p>The MD5 hash of the <code>PutBucketLogging</code> request body.</p>
    /// <p>For requests made using the Amazon Web Services Command Line Interface (CLI) or Amazon Web Services SDKs, this field is calculated automatically.</p>
    pub fn set_content_md5(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_content_md5(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn checksum_algorithm(mut self, input: crate::types::ChecksumAlgorithm) -> Self {
        self.inner = self.inner.checksum_algorithm(input);
        self
    }
    /// <p>Indicates the algorithm used to create the checksum for the object when using the SDK. This header will not provide any additional functionality if not using the SDK. When sending this header, there must be a corresponding <code>x-amz-checksum</code> or <code>x-amz-trailer</code> header sent. Otherwise, Amazon S3 fails the request with the HTTP status code <code>400 Bad Request</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html">Checking object integrity</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>If you provide an individual checksum, Amazon S3 ignores any provided <code>ChecksumAlgorithm</code> parameter.</p>
    pub fn set_checksum_algorithm(
        mut self,
        input: std::option::Option<crate::types::ChecksumAlgorithm>,
    ) -> Self {
        self.inner = self.inner.set_checksum_algorithm(input);
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

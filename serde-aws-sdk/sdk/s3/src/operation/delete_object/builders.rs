// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_object::_delete_object_output::DeleteObjectOutputBuilder;

pub use crate::operation::delete_object::_delete_object_input::DeleteObjectInputBuilder;

/// Fluent builder constructing a request to `DeleteObject`.
///
/// <p>Removes the null version (if there is one) of an object and inserts a delete marker, which becomes the latest version of the object. If there isn't a null version, Amazon S3 does not remove any objects but will still respond that the command was successful.</p>
/// <p>To remove a specific version, you must be the bucket owner and you must use the version Id subresource. Using this subresource permanently deletes the version. If the object deleted is a delete marker, Amazon S3 sets the response header, <code>x-amz-delete-marker</code>, to true. </p>
/// <p>If the object you want to delete is in a bucket where the bucket versioning configuration is MFA Delete enabled, you must include the <code>x-amz-mfa</code> request header in the DELETE <code>versionId</code> request. Requests that include <code>x-amz-mfa</code> must use HTTPS. </p>
/// <p> For more information about MFA Delete, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMFADelete.html">Using MFA Delete</a>. To see sample requests that use versioning, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/RESTObjectDELETE.html#ExampleVersionObjectDelete">Sample Request</a>. </p>
/// <p>You can delete objects by explicitly calling DELETE Object or configure its lifecycle (<a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketLifecycle.html">PutBucketLifecycle</a>) to enable Amazon S3 to remove them for you. If you want to block users or accounts from removing or deleting objects from your bucket, you must deny them the <code>s3:DeleteObject</code>, <code>s3:DeleteObjectVersion</code>, and <code>s3:PutLifeCycleConfiguration</code> actions. </p>
/// <p>The following action is related to <code>DeleteObject</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html">PutObject</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct DeleteObjectFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_object::builders::DeleteObjectInputBuilder,
}
impl DeleteObjectFluentBuilder {
    /// Creates a new `DeleteObject`.
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
            crate::operation::delete_object::DeleteObject,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<crate::operation::delete_object::DeleteObjectError>,
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
        crate::operation::delete_object::DeleteObjectOutput,
        aws_smithy_http::result::SdkError<crate::operation::delete_object::DeleteObjectError>,
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
    ///     let deserialized_parameters: crate::operation::delete_object::builders::DeleteObjectInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.delete_object().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::delete_object::builders::DeleteObjectInputBuilder,
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
        aws_smithy_http::result::SdkError<crate::operation::delete_object::DeleteObjectError>,
    > {
        let input = self
            .inner
            .build()
            .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
        input.presigned(&self.handle.conf, presigning_config).await
    }
    /// <p>The bucket name of the bucket containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The bucket name of the bucket containing the object. </p>
    /// <p>When using this action with an access point, you must direct requests to the access point hostname. The access point hostname takes the form <i>AccessPointName</i>-<i>AccountId</i>.s3-accesspoint.<i>Region</i>.amazonaws.com. When using this action with an access point through the Amazon Web Services SDKs, you provide the access point ARN in place of the bucket name. For more information about access point ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-access-points.html">Using access points</a> in the <i>Amazon S3 User Guide</i>.</p>
    /// <p>When using this action with Amazon S3 on Outposts, you must direct requests to the S3 on Outposts hostname. The S3 on Outposts hostname takes the form <code> <i>AccessPointName</i>-<i>AccountId</i>.<i>outpostID</i>.s3-outposts.<i>Region</i>.amazonaws.com</code>. When using this action with S3 on Outposts through the Amazon Web Services SDKs, you provide the Outposts bucket ARN in place of the bucket name. For more information about S3 on Outposts ARNs, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p>Key name of the object to delete.</p>
    pub fn key(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>Key name of the object to delete.</p>
    pub fn set_key(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device. Required to permanently delete a versioned object if versioning is configured with MFA delete enabled.</p>
    pub fn mfa(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.mfa(input.into());
        self
    }
    /// <p>The concatenation of the authentication device's serial number, a space, and the value that is displayed on your authentication device. Required to permanently delete a versioned object if versioning is configured with MFA delete enabled.</p>
    pub fn set_mfa(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_mfa(input);
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn version_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>VersionId used to reference a specific version of the object.</p>
    pub fn set_version_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
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
    /// <p>Indicates whether S3 Object Lock should bypass Governance-mode restrictions to process this operation. To use this header, you must have the <code>s3:BypassGovernanceRetention</code> permission.</p>
    pub fn bypass_governance_retention(mut self, input: bool) -> Self {
        self.inner = self.inner.bypass_governance_retention(input);
        self
    }
    /// <p>Indicates whether S3 Object Lock should bypass Governance-mode restrictions to process this operation. To use this header, you must have the <code>s3:BypassGovernanceRetention</code> permission.</p>
    pub fn set_bypass_governance_retention(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_bypass_governance_retention(input);
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

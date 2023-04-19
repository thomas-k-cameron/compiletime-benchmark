// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_tagging::_put_bucket_tagging_output::PutBucketTaggingOutputBuilder;

pub use crate::operation::put_bucket_tagging::_put_bucket_tagging_input::PutBucketTaggingInputBuilder;

/// Fluent builder constructing a request to `PutBucketTagging`.
///
/// <note>
/// <p>This action puts tags on an Amazon S3 on Outposts bucket. To put tags on an S3 bucket, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutBucketTagging.html">PutBucketTagging</a> in the <i>Amazon S3 API Reference</i>. </p>
/// </note>
/// <p>Sets the tags for an S3 on Outposts bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/S3onOutposts.html">Using Amazon S3 on Outposts</a> in the <i>Amazon S3 User Guide</i>.</p>
/// <p>Use tags to organize your Amazon Web Services bill to reflect your own cost structure. To do this, sign up to get your Amazon Web Services account bill with tag key values included. Then, to see the cost of combined resources, organize your billing information according to resources with the same tag key values. For example, you can tag several resources with a specific application name, and then organize your billing information to see the total cost of that application across several services. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Cost allocation and tagging</a>.</p> <note>
/// <p>Within a bucket, if you add a tag that has the same key as an existing tag, the new value overwrites the old value. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/CostAllocTagging.html"> Using cost allocation in Amazon S3 bucket tags</a>.</p>
/// </note>
/// <p>To use this action, you must have permissions to perform the <code>s3-outposts:PutBucketTagging</code> action. The Outposts bucket owner has this permission by default and can grant this permission to others. For more information about permissions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-actions.html#using-with-s3-actions-related-to-bucket-subresources"> Permissions Related to Bucket Subresource Operations</a> and <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/s3-access-control.html">Managing access permissions to your Amazon S3 resources</a>.</p>
/// <p> <code>PutBucketTagging</code> has the following special errors:</p>
/// <ul>
/// <li> <p>Error code: <code>InvalidTagError</code> </p>
/// <ul>
/// <li> <p>Description: The tag provided was not a valid tag. This error can occur if the tag did not pass input validation. For information about tag restrictions, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/allocation-tag-restrictions.html"> User-Defined Tag Restrictions</a> and <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/aws-tag-restrictions.html"> Amazon Web Services-Generated Cost Allocation Tag Restrictions</a>.</p> </li>
/// </ul> </li>
/// <li> <p>Error code: <code>MalformedXMLError</code> </p>
/// <ul>
/// <li> <p>Description: The XML provided does not match the schema.</p> </li>
/// </ul> </li>
/// <li> <p>Error code: <code>OperationAbortedError </code> </p>
/// <ul>
/// <li> <p>Description: A conflicting conditional action is currently in progress against this resource. Try again.</p> </li>
/// </ul> </li>
/// <li> <p>Error code: <code>InternalError</code> </p>
/// <ul>
/// <li> <p>Description: The service was unable to apply the provided tag to the bucket.</p> </li>
/// </ul> </li>
/// </ul>
/// <p>All Amazon S3 on Outposts REST API requests for this action require an additional parameter of <code>x-amz-outpost-id</code> to be passed with the request. In addition, you must use an S3 on Outposts endpoint hostname prefix instead of <code>s3-control</code>. For an example of the request syntax for Amazon S3 on Outposts that uses the S3 on Outposts endpoint hostname prefix and the <code>x-amz-outpost-id</code> derived by using the access point ARN, see the <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_PutBucketTagging.html#API_control_PutBucketTagging_Examples">Examples</a> section.</p>
/// <p>The following actions are related to <code>PutBucketTagging</code>:</p>
/// <ul>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_GetBucketTagging.html">GetBucketTagging</a> </p> </li>
/// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_control_DeleteBucketTagging.html">DeleteBucketTagging</a> </p> </li>
/// </ul>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct PutBucketTaggingFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_bucket_tagging::builders::PutBucketTaggingInputBuilder,
}
impl PutBucketTaggingFluentBuilder {
    /// Creates a new `PutBucketTagging`.
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
            crate::operation::put_bucket_tagging::PutBucketTagging,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_tagging::PutBucketTaggingError,
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
        crate::operation::put_bucket_tagging::PutBucketTaggingOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::put_bucket_tagging::PutBucketTaggingError,
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
    ///     let deserialized_parameters: crate::operation::put_bucket_tagging::builders::PutBucketTaggingInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.put_bucket_tagging().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::put_bucket_tagging::builders::PutBucketTaggingInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the Outposts bucket.</p>
    pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    pub fn bucket(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bucket.</p>
    /// <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>
    /// <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:
    /// <region>
    /// :
    /// <account-id>
    /// :outpost/
    /// <outpost-id>
    /// /bucket/
    /// <my-bucket-name></my-bucket-name>
    /// </outpost-id>
    /// </account-id>
    /// </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    pub fn set_bucket(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    /// <p></p>
    pub fn tagging(mut self, input: crate::types::Tagging) -> Self {
        self.inner = self.inner.tagging(input);
        self
    }
    /// <p></p>
    pub fn set_tagging(mut self, input: std::option::Option<crate::types::Tagging>) -> Self {
        self.inner = self.inner.set_tagging(input);
        self
    }
}

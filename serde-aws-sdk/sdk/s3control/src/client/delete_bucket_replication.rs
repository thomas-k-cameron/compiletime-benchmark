// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketReplication`](crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder::set_account_id): <p>The Amazon Web Services account ID of the Outposts bucket to delete the replication configuration for.</p>
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder::set_bucket): <p>Specifies the S3 on Outposts bucket to delete the replication configuration for.</p>  <p>For using this parameter with Amazon S3 on Outposts with the REST API, you must specify the name and the x-amz-outpost-id as well.</p>  <p>For using this parameter with S3 on Outposts with the Amazon Web Services SDK and CLI, you must specify the ARN of the bucket accessed in the format <code>arn:aws:s3-outposts:   <region>    :    <account-id>     :outpost/     <outpost-id>      /bucket/      <my-bucket-name></my-bucket-name>     </outpost-id>    </account-id>   </region></code>. For example, to access the bucket <code>reports</code> through Outpost <code>my-outpost</code> owned by account <code>123456789012</code> in Region <code>us-west-2</code>, use the URL encoding of <code>arn:aws:s3-outposts:us-west-2:123456789012:outpost/my-outpost/bucket/reports</code>. The value must be URL encoded. </p>
    /// - On success, responds with [`DeleteBucketReplicationOutput`](crate::operation::delete_bucket_replication::DeleteBucketReplicationOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketReplicationError>`](crate::operation::delete_bucket_replication::DeleteBucketReplicationError)
    pub fn delete_bucket_replication(
        &self,
    ) -> crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder
    {
        crate::operation::delete_bucket_replication::builders::DeleteBucketReplicationFluentBuilder::new(self.handle.clone())
    }
}

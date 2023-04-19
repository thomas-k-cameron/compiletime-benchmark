// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketReplication`](crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder::set_bucket): <p>The bucket name for which to get the replication information.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketReplicationOutput`](crate::operation::get_bucket_replication::GetBucketReplicationOutput) with field(s):
    ///   - [`replication_configuration(Option<ReplicationConfiguration>)`](crate::operation::get_bucket_replication::GetBucketReplicationOutput::replication_configuration): <p>A container for replication rules. You can add up to 1,000 rules. The maximum size of a replication configuration is 2 MB.</p>
    /// - On failure, responds with [`SdkError<GetBucketReplicationError>`](crate::operation::get_bucket_replication::GetBucketReplicationError)
    pub fn get_bucket_replication(
        &self,
    ) -> crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder {
        crate::operation::get_bucket_replication::builders::GetBucketReplicationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
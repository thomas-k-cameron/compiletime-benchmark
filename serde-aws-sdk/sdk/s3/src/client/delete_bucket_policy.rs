// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketPolicy`](crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder::set_bucket): <p>The bucket name.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`DeleteBucketPolicyOutput`](crate::operation::delete_bucket_policy::DeleteBucketPolicyOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketPolicyError>`](crate::operation::delete_bucket_policy::DeleteBucketPolicyError)
    pub fn delete_bucket_policy(
        &self,
    ) -> crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder {
        crate::operation::delete_bucket_policy::builders::DeleteBucketPolicyFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

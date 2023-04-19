// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeletePublicAccessBlock`](crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder::set_bucket): <p>The Amazon S3 bucket whose <code>PublicAccessBlock</code> configuration you want to delete. </p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`DeletePublicAccessBlockOutput`](crate::operation::delete_public_access_block::DeletePublicAccessBlockOutput)
    /// - On failure, responds with [`SdkError<DeletePublicAccessBlockError>`](crate::operation::delete_public_access_block::DeletePublicAccessBlockError)
    pub fn delete_public_access_block(
        &self,
    ) -> crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder
    {
        crate::operation::delete_public_access_block::builders::DeletePublicAccessBlockFluentBuilder::new(self.handle.clone())
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketWebsite`](crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder::set_bucket): <p>The bucket name for which to get the website configuration.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketWebsiteOutput`](crate::operation::get_bucket_website::GetBucketWebsiteOutput) with field(s):
    ///   - [`redirect_all_requests_to(Option<RedirectAllRequestsTo>)`](crate::operation::get_bucket_website::GetBucketWebsiteOutput::redirect_all_requests_to): <p>Specifies the redirect behavior of all requests to a website endpoint of an Amazon S3 bucket.</p>
    ///   - [`index_document(Option<IndexDocument>)`](crate::operation::get_bucket_website::GetBucketWebsiteOutput::index_document): <p>The name of the index document for the website (for example <code>index.html</code>).</p>
    ///   - [`error_document(Option<ErrorDocument>)`](crate::operation::get_bucket_website::GetBucketWebsiteOutput::error_document): <p>The object key name of the website error document to use for 4XX class errors.</p>
    ///   - [`routing_rules(Option<Vec<RoutingRule>>)`](crate::operation::get_bucket_website::GetBucketWebsiteOutput::routing_rules): <p>Rules that define when a redirect is applied and the redirect behavior.</p>
    /// - On failure, responds with [`SdkError<GetBucketWebsiteError>`](crate::operation::get_bucket_website::GetBucketWebsiteError)
    pub fn get_bucket_website(
        &self,
    ) -> crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder {
        crate::operation::get_bucket_website::builders::GetBucketWebsiteFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

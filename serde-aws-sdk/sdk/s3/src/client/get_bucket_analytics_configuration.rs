// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketAnalyticsConfiguration`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::set_bucket): <p>The name of the bucket from which an analytics configuration is retrieved.</p>
    ///   - [`id(impl Into<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::set_id): <p>The ID that identifies the analytics configuration.</p>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::set_expected_bucket_owner): <p>The account ID of the expected bucket owner. If the bucket is owned by a different account, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p>
    /// - On success, responds with [`GetBucketAnalyticsConfigurationOutput`](crate::operation::get_bucket_analytics_configuration::GetBucketAnalyticsConfigurationOutput) with field(s):
    ///   - [`analytics_configuration(Option<AnalyticsConfiguration>)`](crate::operation::get_bucket_analytics_configuration::GetBucketAnalyticsConfigurationOutput::analytics_configuration): <p>The configuration and any analyses for the analytics filter.</p>
    /// - On failure, responds with [`SdkError<GetBucketAnalyticsConfigurationError>`](crate::operation::get_bucket_analytics_configuration::GetBucketAnalyticsConfigurationError)
    pub fn get_bucket_analytics_configuration(&self) -> crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder{
        crate::operation::get_bucket_analytics_configuration::builders::GetBucketAnalyticsConfigurationFluentBuilder::new(self.handle.clone())
    }
}

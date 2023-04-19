// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetJobTagging`](crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder::set_account_id): <p>The Amazon Web Services account ID associated with the S3 Batch Operations job.</p>
    ///   - [`job_id(impl Into<String>)`](crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder::set_job_id): <p>The ID for the S3 Batch Operations job whose tags you want to retrieve.</p>
    /// - On success, responds with [`GetJobTaggingOutput`](crate::operation::get_job_tagging::GetJobTaggingOutput) with field(s):
    ///   - [`tags(Option<Vec<S3Tag>>)`](crate::operation::get_job_tagging::GetJobTaggingOutput::tags): <p>The set of tags associated with the S3 Batch Operations job.</p>
    /// - On failure, responds with [`SdkError<GetJobTaggingError>`](crate::operation::get_job_tagging::GetJobTaggingError)
    pub fn get_job_tagging(
        &self,
    ) -> crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder {
        crate::operation::get_job_tagging::builders::GetJobTaggingFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

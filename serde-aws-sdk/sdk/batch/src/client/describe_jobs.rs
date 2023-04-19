// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeJobs`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`jobs(Vec<String>)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::jobs) / [`set_jobs(Option<Vec<String>>)`](crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::set_jobs): <p>A list of up to 100 job IDs.</p>
    /// - On success, responds with [`DescribeJobsOutput`](crate::operation::describe_jobs::DescribeJobsOutput) with field(s):
    ///   - [`jobs(Option<Vec<JobDetail>>)`](crate::operation::describe_jobs::DescribeJobsOutput::jobs): <p>The list of jobs.</p>
    /// - On failure, responds with [`SdkError<DescribeJobsError>`](crate::operation::describe_jobs::DescribeJobsError)
    pub fn describe_jobs(
        &self,
    ) -> crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder {
        crate::operation::describe_jobs::builders::DescribeJobsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
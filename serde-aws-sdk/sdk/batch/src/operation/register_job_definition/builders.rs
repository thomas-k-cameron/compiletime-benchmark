// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_job_definition::_register_job_definition_output::RegisterJobDefinitionOutputBuilder;

pub use crate::operation::register_job_definition::_register_job_definition_input::RegisterJobDefinitionInputBuilder;

/// Fluent builder constructing a request to `RegisterJobDefinition`.
///
/// <p>Registers an Batch job definition.</p>
#[derive(std::clone::Clone, std::fmt::Debug)]
pub struct RegisterJobDefinitionFluentBuilder {
    handle: std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_job_definition::builders::RegisterJobDefinitionInputBuilder,
}
impl RegisterJobDefinitionFluentBuilder {
    /// Creates a new `RegisterJobDefinition`.
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
            crate::operation::register_job_definition::RegisterJobDefinition,
            aws_http::retry::AwsResponseRetryClassifier,
        >,
        aws_smithy_http::result::SdkError<
            crate::operation::register_job_definition::RegisterJobDefinitionError,
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
        crate::operation::register_job_definition::RegisterJobDefinitionOutput,
        aws_smithy_http::result::SdkError<
            crate::operation::register_job_definition::RegisterJobDefinitionError,
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
    ///     let deserialized_parameters: crate::operation::register_job_definition::builders::RegisterJobDefinitionInputBuilder  = serde_json::from_str(&json_string).unwrap();
    ///     client.register_job_definition().set_fields(&deserialized_parameters).send().await
    /// };
    /// ```
    pub fn set_fields(
        mut self,
        data: crate::operation::register_job_definition::builders::RegisterJobDefinitionInputBuilder,
    ) -> Self {
        self.inner = data;
        self
    }
    /// <p>The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn job_definition_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.inner = self.inner.job_definition_name(input.into());
        self
    }
    /// <p>The name of the job definition to register. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn set_job_definition_name(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_job_definition_name(input);
        self
    }
    /// <p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <note>
    /// <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p>
    /// </note>
    pub fn r#type(mut self, input: crate::types::JobDefinitionType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of job definition. For more information about multi-node parallel jobs, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-job-def.html">Creating a multi-node parallel job definition</a> in the <i>Batch User Guide</i>.</p> <note>
    /// <p>If the job is run on Fargate resources, then <code>multinode</code> isn't supported.</p>
    /// </note>
    pub fn set_type(mut self, input: std::option::Option<crate::types::JobDefinitionType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// Adds a key-value pair to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    pub fn parameters(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.parameters(k.into(), v.into());
        self
    }
    /// <p>Default parameter substitution placeholders to set in the job definition. Parameters are specified as a key-value pair mapping. Parameters in a <code>SubmitJob</code> request override any corresponding parameter defaults from the job definition.</p>
    pub fn set_parameters(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_parameters(input);
        self
    }
    /// <p>The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p>
    /// <p>The minimum supported value is 0 and the maximum supported value is 9999.</p>
    pub fn scheduling_priority(mut self, input: i32) -> Self {
        self.inner = self.inner.scheduling_priority(input);
        self
    }
    /// <p>The scheduling priority for jobs that are submitted with this job definition. This only affects jobs in job queues with a fair share policy. Jobs with a higher scheduling priority are scheduled before jobs with a lower scheduling priority.</p>
    /// <p>The minimum supported value is 0 and the maximum supported value is 9999.</p>
    pub fn set_scheduling_priority(mut self, input: std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_scheduling_priority(input);
        self
    }
    /// <p>An object with various properties specific to Amazon ECS based single-node container-based jobs. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>. This must not be specified for Amazon EKS based job definitions.</p> <note>
    /// <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use only <code>containerProperties</code>.</p>
    /// </note>
    pub fn container_properties(mut self, input: crate::types::ContainerProperties) -> Self {
        self.inner = self.inner.container_properties(input);
        self
    }
    /// <p>An object with various properties specific to Amazon ECS based single-node container-based jobs. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>. This must not be specified for Amazon EKS based job definitions.</p> <note>
    /// <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use only <code>containerProperties</code>.</p>
    /// </note>
    pub fn set_container_properties(
        mut self,
        input: std::option::Option<crate::types::ContainerProperties>,
    ) -> Self {
        self.inner = self.inner.set_container_properties(input);
        self
    }
    /// <p>An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-parallel-jobs.html">Multi-node Parallel Jobs</a> in the <i>Batch User Guide</i>. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p> <note>
    /// <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use <code>containerProperties</code> instead.</p>
    /// </note> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>nodeProperties</code>.</p>
    /// </note>
    pub fn node_properties(mut self, input: crate::types::NodeProperties) -> Self {
        self.inner = self.inner.node_properties(input);
        self
    }
    /// <p>An object with various properties specific to multi-node parallel jobs. If you specify node properties for a job, it becomes a multi-node parallel job. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/multi-node-parallel-jobs.html">Multi-node Parallel Jobs</a> in the <i>Batch User Guide</i>. If the job definition's <code>type</code> parameter is <code>container</code>, then you must specify either <code>containerProperties</code> or <code>nodeProperties</code>.</p> <note>
    /// <p>If the job runs on Fargate resources, then you must not specify <code>nodeProperties</code>; use <code>containerProperties</code> instead.</p>
    /// </note> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>nodeProperties</code>.</p>
    /// </note>
    pub fn set_node_properties(
        mut self,
        input: std::option::Option<crate::types::NodeProperties>,
    ) -> Self {
        self.inner = self.inner.set_node_properties(input);
        self
    }
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a <code>SubmitJob</code> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.</p>
    pub fn retry_strategy(mut self, input: crate::types::RetryStrategy) -> Self {
        self.inner = self.inner.retry_strategy(input);
        self
    }
    /// <p>The retry strategy to use for failed jobs that are submitted with this job definition. Any retry strategy that's specified during a <code>SubmitJob</code> operation overrides the retry strategy defined here. If a job is terminated due to a timeout, it isn't retried.</p>
    pub fn set_retry_strategy(
        mut self,
        input: std::option::Option<crate::types::RetryStrategy>,
    ) -> Self {
        self.inner = self.inner.set_retry_strategy(input);
        self
    }
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>propagateTags</code>.</p>
    /// </note>
    pub fn propagate_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.propagate_tags(input);
        self
    }
    /// <p>Specifies whether to propagate the tags from the job or job definition to the corresponding Amazon ECS task. If no value is specified, the tags are not propagated. Tags can only be propagated to the tasks during task creation. For tags with the same name, job tags are given priority over job definitions tags. If the total number of combined tags from the job and job definition is over 50, the job is moved to the <code>FAILED</code> state.</p> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>propagateTags</code>.</p>
    /// </note>
    pub fn set_propagate_tags(mut self, input: std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_propagate_tags(input);
        self
    }
    /// <p>The timeout configuration for jobs that are submitted with this job definition, after which Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that's specified during a <code>SubmitJob</code> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_timeouts.html">Job Timeouts</a> in the <i>Batch User Guide</i>.</p>
    pub fn timeout(mut self, input: crate::types::JobTimeout) -> Self {
        self.inner = self.inner.timeout(input);
        self
    }
    /// <p>The timeout configuration for jobs that are submitted with this job definition, after which Batch terminates your jobs if they have not finished. If a job is terminated due to a timeout, it isn't retried. The minimum value for the timeout is 60 seconds. Any timeout configuration that's specified during a <code>SubmitJob</code> operation overrides the timeout configuration defined here. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/job_timeouts.html">Job Timeouts</a> in the <i>Batch User Guide</i>.</p>
    pub fn set_timeout(mut self, input: std::option::Option<crate::types::JobTimeout>) -> Self {
        self.inner = self.inner.set_timeout(input);
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that you apply to the job definition to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging Amazon Web Services Resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags that you apply to the job definition to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging Amazon Web Services Resources</a> in <i>Batch User Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// Appends an item to `platformCapabilities`.
    ///
    /// To override the contents of this collection use [`set_platform_capabilities`](Self::set_platform_capabilities).
    ///
    /// <p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. To run the job on Fargate resources, specify <code>FARGATE</code>.</p> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>platformCapabilities</code>.</p>
    /// </note>
    pub fn platform_capabilities(mut self, input: crate::types::PlatformCapability) -> Self {
        self.inner = self.inner.platform_capabilities(input);
        self
    }
    /// <p>The platform capabilities required by the job definition. If no value is specified, it defaults to <code>EC2</code>. To run the job on Fargate resources, specify <code>FARGATE</code>.</p> <note>
    /// <p>If the job runs on Amazon EKS resources, then you must not specify <code>platformCapabilities</code>.</p>
    /// </note>
    pub fn set_platform_capabilities(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::PlatformCapability>>,
    ) -> Self {
        self.inner = self.inner.set_platform_capabilities(input);
        self
    }
    /// <p>An object with various properties that are specific to Amazon EKS based jobs. This must not be specified for Amazon ECS based job definitions.</p>
    pub fn eks_properties(mut self, input: crate::types::EksProperties) -> Self {
        self.inner = self.inner.eks_properties(input);
        self
    }
    /// <p>An object with various properties that are specific to Amazon EKS based jobs. This must not be specified for Amazon ECS based job definitions.</p>
    pub fn set_eks_properties(
        mut self,
        input: std::option::Option<crate::types::EksProperties>,
    ) -> Self {
        self.inner = self.inner.set_eks_properties(input);
        self
    }
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>An object that represents the details for an Batch job queue.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct JobQueueDetail {
    /// <p>The job queue name.</p>
    #[doc(hidden)]
    pub job_queue_name: std::option::Option<std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    #[doc(hidden)]
    pub job_queue_arn: std::option::Option<std::string::String>,
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::JqState>,
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    #[doc(hidden)]
    pub scheduling_policy_arn: std::option::Option<std::string::String>,
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    #[doc(hidden)]
    pub status: std::option::Option<crate::types::JqStatus>,
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    #[doc(hidden)]
    pub status_reason: std::option::Option<std::string::String>,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). EC2 and Fargate compute environments can't be mixed.</p>
    #[doc(hidden)]
    pub priority: std::option::Option<i32>,
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    #[doc(hidden)]
    pub compute_environment_order:
        std::option::Option<std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl JobQueueDetail {
    /// <p>The job queue name.</p>
    pub fn job_queue_name(&self) -> std::option::Option<&str> {
        self.job_queue_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn job_queue_arn(&self) -> std::option::Option<&str> {
        self.job_queue_arn.as_deref()
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::JqState> {
        self.state.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(&self) -> std::option::Option<&str> {
        self.scheduling_policy_arn.as_deref()
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn status(&self) -> std::option::Option<&crate::types::JqStatus> {
        self.status.as_ref()
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn status_reason(&self) -> std::option::Option<&str> {
        self.status_reason.as_deref()
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). EC2 and Fargate compute environments can't be mixed.</p>
    pub fn priority(&self) -> std::option::Option<i32> {
        self.priority
    }
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn compute_environment_order(
        &self,
    ) -> std::option::Option<&[crate::types::ComputeEnvironmentOrder]> {
        self.compute_environment_order.as_deref()
    }
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl JobQueueDetail {
    /// Creates a new builder-style object to manufacture [`JobQueueDetail`](crate::types::JobQueueDetail).
    pub fn builder() -> crate::types::builders::JobQueueDetailBuilder {
        crate::types::builders::JobQueueDetailBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::JobQueueDetail;
/// A builder for [`JobQueueDetail`](crate::types::JobQueueDetail).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct JobQueueDetailBuilder {
    pub(crate) job_queue_name: std::option::Option<std::string::String>,
    pub(crate) job_queue_arn: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::JqState>,
    pub(crate) scheduling_policy_arn: std::option::Option<std::string::String>,
    pub(crate) status: std::option::Option<crate::types::JqStatus>,
    pub(crate) status_reason: std::option::Option<std::string::String>,
    pub(crate) priority: std::option::Option<i32>,
    pub(crate) compute_environment_order:
        std::option::Option<std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl JobQueueDetailBuilder {
    /// <p>The job queue name.</p>
    pub fn job_queue_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_queue_name = Some(input.into());
        self
    }
    /// <p>The job queue name.</p>
    pub fn set_job_queue_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_queue_name = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn job_queue_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_queue_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the job queue.</p>
    pub fn set_job_queue_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_queue_arn = input;
        self
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn state(mut self, input: crate::types::JqState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>Describes the ability of the queue to accept new jobs. If the job queue state is <code>ENABLED</code>, it can accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn set_state(mut self, input: std::option::Option<crate::types::JqState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.scheduling_policy_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. For example, <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn set_scheduling_policy_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.scheduling_policy_arn = input;
        self
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn status(mut self, input: crate::types::JqStatus) -> Self {
        self.status = Some(input);
        self
    }
    /// <p>The status of the job queue (for example, <code>CREATING</code> or <code>VALID</code>).</p>
    pub fn set_status(mut self, input: std::option::Option<crate::types::JqStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn status_reason(mut self, input: impl Into<std::string::String>) -> Self {
        self.status_reason = Some(input.into());
        self
    }
    /// <p>A short, human-readable string to provide additional details for the current status of the job queue.</p>
    pub fn set_status_reason(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). EC2 and Fargate compute environments can't be mixed.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = Some(input);
        self
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>). EC2 and Fargate compute environments can't be mixed.</p>
    pub fn set_priority(mut self, input: std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// Appends an item to `compute_environment_order`.
    ///
    /// To override the contents of this collection use [`set_compute_environment_order`](Self::set_compute_environment_order).
    ///
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn compute_environment_order(
        mut self,
        input: crate::types::ComputeEnvironmentOrder,
    ) -> Self {
        let mut v = self.compute_environment_order.unwrap_or_default();
        v.push(input);
        self.compute_environment_order = Some(v);
        self
    }
    /// <p>The compute environments that are attached to the job queue and the order that job placement is preferred. Compute environments are selected for job placement in ascending order.</p>
    pub fn set_compute_environment_order(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    ) -> Self {
        self.compute_environment_order = input;
        self
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(
        mut self,
        k: impl Into<std::string::String>,
        v: impl Into<std::string::String>,
    ) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = Some(hash_map);
        self
    }
    /// <p>The tags that are applied to the job queue. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`JobQueueDetail`](crate::types::JobQueueDetail).
    pub fn build(self) -> crate::types::JobQueueDetail {
        crate::types::JobQueueDetail {
            job_queue_name: self.job_queue_name,
            job_queue_arn: self.job_queue_arn,
            state: self.state,
            scheduling_policy_arn: self.scheduling_policy_arn,
            status: self.status,
            status_reason: self.status_reason,
            priority: self.priority,
            compute_environment_order: self.compute_environment_order,
            tags: self.tags,
        }
    }
}
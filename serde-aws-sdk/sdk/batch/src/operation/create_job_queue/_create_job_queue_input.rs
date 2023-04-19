// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for <code>CreateJobQueue</code>.</p>
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
pub struct CreateJobQueueInput {
    /// <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    #[doc(hidden)]
    pub job_queue_name: std::option::Option<std::string::String>,
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::types::JqState>,
    /// <p>The Amazon Resource Name (ARN) of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. An example is <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    #[doc(hidden)]
    pub scheduling_policy_arn: std::option::Option<std::string::String>,
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p>
    #[doc(hidden)]
    pub priority: std::option::Option<i32>,
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p> <note>
    /// <p>All compute environments that are associated with a job queue must share the same architecture. Batch doesn't support mixing compute environment architecture types in a single job queue.</p>
    /// </note>
    #[doc(hidden)]
    pub compute_environment_order:
        std::option::Option<std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    /// <p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl CreateJobQueueInput {
    /// <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn job_queue_name(&self) -> std::option::Option<&str> {
        self.job_queue_name.as_deref()
    }
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn state(&self) -> std::option::Option<&crate::types::JqState> {
        self.state.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. An example is <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(&self) -> std::option::Option<&str> {
        self.scheduling_policy_arn.as_deref()
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p>
    pub fn priority(&self) -> std::option::Option<i32> {
        self.priority
    }
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p> <note>
    /// <p>All compute environments that are associated with a job queue must share the same architecture. Batch doesn't support mixing compute environment architecture types in a single job queue.</p>
    /// </note>
    pub fn compute_environment_order(
        &self,
    ) -> std::option::Option<&[crate::types::ComputeEnvironmentOrder]> {
        self.compute_environment_order.as_deref()
    }
    /// <p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl CreateJobQueueInput {
    /// Creates a new builder-style object to manufacture [`CreateJobQueueInput`](crate::operation::create_job_queue::CreateJobQueueInput).
    pub fn builder() -> crate::operation::create_job_queue::builders::CreateJobQueueInputBuilder {
        crate::operation::create_job_queue::builders::CreateJobQueueInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::create_job_queue::CreateJobQueueInput;
/// A builder for [`CreateJobQueueInput`](crate::operation::create_job_queue::CreateJobQueueInput).
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
pub struct CreateJobQueueInputBuilder {
    pub(crate) job_queue_name: std::option::Option<std::string::String>,
    pub(crate) state: std::option::Option<crate::types::JqState>,
    pub(crate) scheduling_policy_arn: std::option::Option<std::string::String>,
    pub(crate) priority: std::option::Option<i32>,
    pub(crate) compute_environment_order:
        std::option::Option<std::vec::Vec<crate::types::ComputeEnvironmentOrder>>,
    pub(crate) tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
}
impl CreateJobQueueInputBuilder {
    /// <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn job_queue_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.job_queue_name = Some(input.into());
        self
    }
    /// <p>The name of the job queue. It can be up to 128 letters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn set_job_queue_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.job_queue_name = input;
        self
    }
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn state(mut self, input: crate::types::JqState) -> Self {
        self.state = Some(input);
        self
    }
    /// <p>The state of the job queue. If the job queue state is <code>ENABLED</code>, it is able to accept jobs. If the job queue state is <code>DISABLED</code>, new jobs can't be added to the queue, but jobs already in the queue can finish.</p>
    pub fn set_state(mut self, input: std::option::Option<crate::types::JqState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. An example is <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn scheduling_policy_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.scheduling_policy_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the fair share scheduling policy. If this parameter is specified, the job queue uses a fair share scheduling policy. If this parameter isn't specified, the job queue uses a first in, first out (FIFO) scheduling policy. After a job queue is created, you can replace but can't remove the fair share scheduling policy. The format is <code>aws:<i>Partition</i>:batch:<i>Region</i>:<i>Account</i>:scheduling-policy/<i>Name</i> </code>. An example is <code>aws:aws:batch:us-west-2:123456789012:scheduling-policy/MySchedulingPolicy</code>.</p>
    pub fn set_scheduling_policy_arn(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.scheduling_policy_arn = input;
        self
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p>
    pub fn priority(mut self, input: i32) -> Self {
        self.priority = Some(input);
        self
    }
    /// <p>The priority of the job queue. Job queues with a higher priority (or a higher integer value for the <code>priority</code> parameter) are evaluated first when associated with the same compute environment. Priority is determined in descending order. For example, a job queue with a priority value of <code>10</code> is given scheduling preference over a job queue with a priority value of <code>1</code>. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p>
    pub fn set_priority(mut self, input: std::option::Option<i32>) -> Self {
        self.priority = input;
        self
    }
    /// Appends an item to `compute_environment_order`.
    ///
    /// To override the contents of this collection use [`set_compute_environment_order`](Self::set_compute_environment_order).
    ///
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p> <note>
    /// <p>All compute environments that are associated with a job queue must share the same architecture. Batch doesn't support mixing compute environment architecture types in a single job queue.</p>
    /// </note>
    pub fn compute_environment_order(
        mut self,
        input: crate::types::ComputeEnvironmentOrder,
    ) -> Self {
        let mut v = self.compute_environment_order.unwrap_or_default();
        v.push(input);
        self.compute_environment_order = Some(v);
        self
    }
    /// <p>The set of compute environments mapped to a job queue and their order relative to each other. The job scheduler uses this parameter to determine which compute environment runs a specific job. Compute environments must be in the <code>VALID</code> state before you can associate them with a job queue. You can associate up to three compute environments with a job queue. All of the compute environments must be either EC2 (<code>EC2</code> or <code>SPOT</code>) or Fargate (<code>FARGATE</code> or <code>FARGATE_SPOT</code>); EC2 and Fargate compute environments can't be mixed.</p> <note>
    /// <p>All compute environments that are associated with a job queue must share the same architecture. Batch doesn't support mixing compute environment architecture types in a single job queue.</p>
    /// </note>
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
    /// <p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
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
    /// <p>The tags that you apply to the job queue to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a> in <i>Batch User Guide</i>.</p>
    pub fn set_tags(
        mut self,
        input: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
    ) -> Self {
        self.tags = input;
        self
    }
    /// Consumes the builder and constructs a [`CreateJobQueueInput`](crate::operation::create_job_queue::CreateJobQueueInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::create_job_queue::CreateJobQueueInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::create_job_queue::CreateJobQueueInput {
            job_queue_name: self.job_queue_name,
            state: self.state,
            scheduling_policy_arn: self.scheduling_policy_arn,
            priority: self.priority,
            compute_environment_order: self.compute_environment_order,
            tags: self.tags,
        })
    }
}
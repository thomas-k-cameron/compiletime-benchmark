// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopTask`](crate::operation::stop_task::builders::StopTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task to stop. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`task(impl Into<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::task) / [`set_task(Option<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::set_task): <p>The task ID of the task to stop.</p>
    ///   - [`reason(impl Into<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::reason) / [`set_reason(Option<String>)`](crate::operation::stop_task::builders::StopTaskFluentBuilder::set_reason): <p>An optional message specified when a task is stopped. For example, if you're using a custom scheduler, you can use this parameter to specify the reason for stopping the task here, and the message appears in subsequent <code>DescribeTasks</code> API operations on this task. Up to 255 characters are allowed in this message.</p>
    /// - On success, responds with [`StopTaskOutput`](crate::operation::stop_task::StopTaskOutput) with field(s):
    ///   - [`task(Option<Task>)`](crate::operation::stop_task::StopTaskOutput::task): <p>The task that was stopped.</p>
    /// - On failure, responds with [`SdkError<StopTaskError>`](crate::operation::stop_task::StopTaskError)
    pub fn stop_task(&self) -> crate::operation::stop_task::builders::StopTaskFluentBuilder {
        crate::operation::stop_task::builders::StopTaskFluentBuilder::new(self.handle.clone())
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTaskProtection`](crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the service that the task sets exist in.</p>
    ///   - [`tasks(Vec<String>)`](crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder::tasks) / [`set_tasks(Option<Vec<String>>)`](crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder::set_tasks): <p>A list of up to 100 task IDs or full ARN entries.</p>
    /// - On success, responds with [`GetTaskProtectionOutput`](crate::operation::get_task_protection::GetTaskProtectionOutput) with field(s):
    ///   - [`protected_tasks(Option<Vec<ProtectedTask>>)`](crate::operation::get_task_protection::GetTaskProtectionOutput::protected_tasks): <p>A list of tasks with the following information.</p>  <ul>   <li> <p> <code>taskArn</code>: The task ARN.</p> </li>   <li> <p> <code>protectionEnabled</code>: The protection status of the task. If scale-in protection is enabled for a task, the value is <code>true</code>. Otherwise, it is <code>false</code>.</p> </li>   <li> <p> <code>expirationDate</code>: The epoch time when protection for the task will expire.</p> </li>  </ul>
    ///   - [`failures(Option<Vec<Failure>>)`](crate::operation::get_task_protection::GetTaskProtectionOutput::failures): <p>Any failures associated with the call.</p>
    /// - On failure, responds with [`SdkError<GetTaskProtectionError>`](crate::operation::get_task_protection::GetTaskProtectionError)
    pub fn get_task_protection(
        &self,
    ) -> crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder {
        crate::operation::get_task_protection::builders::GetTaskProtectionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
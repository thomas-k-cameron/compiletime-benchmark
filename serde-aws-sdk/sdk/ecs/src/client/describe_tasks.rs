// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTasks`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that hosts the task or tasks to describe. If you do not specify a cluster, the default cluster is assumed. This parameter is required if the task or tasks you are describing were launched in any cluster other than the default cluster.</p>
    ///   - [`tasks(Vec<String>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::tasks) / [`set_tasks(Option<Vec<String>>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::set_tasks): <p>A list of up to 100 task IDs or full ARN entries.</p>
    ///   - [`include(Vec<TaskField>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::include) / [`set_include(Option<Vec<TaskField>>)`](crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::set_include): <p>Specifies whether you want to see the resource tags for the task. If <code>TAGS</code> is specified, the tags are included in the response. If this field is omitted, tags aren't included in the response.</p>
    /// - On success, responds with [`DescribeTasksOutput`](crate::operation::describe_tasks::DescribeTasksOutput) with field(s):
    ///   - [`tasks(Option<Vec<Task>>)`](crate::operation::describe_tasks::DescribeTasksOutput::tasks): <p>The list of tasks.</p>
    ///   - [`failures(Option<Vec<Failure>>)`](crate::operation::describe_tasks::DescribeTasksOutput::failures): <p>Any failures associated with the call.</p>
    /// - On failure, responds with [`SdkError<DescribeTasksError>`](crate::operation::describe_tasks::DescribeTasksError)
    pub fn describe_tasks(
        &self,
    ) -> crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder {
        crate::operation::describe_tasks::builders::DescribeTasksFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

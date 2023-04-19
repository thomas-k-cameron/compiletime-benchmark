// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTasks`](crate::operation::list_tasks::builders::ListTasksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster to use when filtering the <code>ListTasks</code> results. If you do not specify a cluster, the default cluster is assumed.</p>
    ///   - [`container_instance(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::container_instance) / [`set_container_instance(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_container_instance): <p>The container instance ID or full ARN of the container instance to use when filtering the <code>ListTasks</code> results. Specifying a <code>containerInstance</code> limits the results to tasks that belong to that container instance.</p>
    ///   - [`family(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::family) / [`set_family(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_family): <p>The name of the task definition family to use when filtering the <code>ListTasks</code> results. Specifying a <code>family</code> limits the results to tasks that belong to that family.</p>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_next_token): <p>The <code>nextToken</code> value returned from a <code>ListTasks</code> request indicating that more results are available to fulfill the request and further calls will be needed. If <code>maxResults</code> was provided, it's possible the number of results to be fewer than <code>maxResults</code>.</p> <note>   <p>This token should be treated as an opaque identifier that is only used to retrieve the next items in a list and not for other programmatic purposes.</p>  </note>
    ///   - [`max_results(i32)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_max_results): <p>The maximum number of task results that <code>ListTasks</code> returned in paginated output. When this parameter is used, <code>ListTasks</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListTasks</code> request with the returned <code>nextToken</code> value. This value can be between 1 and 100. If this parameter isn't used, then <code>ListTasks</code> returns up to 100 results and a <code>nextToken</code> value if applicable.</p>
    ///   - [`started_by(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::started_by) / [`set_started_by(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_started_by): <p>The <code>startedBy</code> value to filter the task results with. Specifying a <code>startedBy</code> value limits the results to tasks that were started with that value.</p>  <p>When you specify <code>startedBy</code> as the filter, it must be the only filter that you use.</p>
    ///   - [`service_name(impl Into<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::service_name) / [`set_service_name(Option<String>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_service_name): <p>The name of the service to use when filtering the <code>ListTasks</code> results. Specifying a <code>serviceName</code> limits the results to tasks that belong to that service.</p>
    ///   - [`desired_status(DesiredStatus)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::desired_status) / [`set_desired_status(Option<DesiredStatus>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_desired_status): <p>The task desired status to use when filtering the <code>ListTasks</code> results. Specifying a <code>desiredStatus</code> of <code>STOPPED</code> limits the results to tasks that Amazon ECS has set the desired status to <code>STOPPED</code>. This can be useful for debugging tasks that aren't starting properly or have died or finished. The default status filter is <code>RUNNING</code>, which shows tasks that Amazon ECS has set the desired status to <code>RUNNING</code>.</p> <note>   <p>Although you can filter results based on a desired status of <code>PENDING</code>, this doesn't return any results. Amazon ECS never sets the desired status of a task to that value (only a task's <code>lastStatus</code> may have a value of <code>PENDING</code>).</p>  </note>
    ///   - [`launch_type(LaunchType)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::launch_type) / [`set_launch_type(Option<LaunchType>)`](crate::operation::list_tasks::builders::ListTasksFluentBuilder::set_launch_type): <p>The launch type to use when filtering the <code>ListTasks</code> results.</p>
    /// - On success, responds with [`ListTasksOutput`](crate::operation::list_tasks::ListTasksOutput) with field(s):
    ///   - [`task_arns(Option<Vec<String>>)`](crate::operation::list_tasks::ListTasksOutput::task_arns): <p>The list of task ARN entries for the <code>ListTasks</code> request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tasks::ListTasksOutput::next_token): <p>The <code>nextToken</code> value to include in a future <code>ListTasks</code> request. When the results of a <code>ListTasks</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListTasksError>`](crate::operation::list_tasks::ListTasksError)
    pub fn list_tasks(&self) -> crate::operation::list_tasks::builders::ListTasksFluentBuilder {
        crate::operation::list_tasks::builders::ListTasksFluentBuilder::new(self.handle.clone())
    }
}

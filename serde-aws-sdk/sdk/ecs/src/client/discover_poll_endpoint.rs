// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DiscoverPollEndpoint`](crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_instance(impl Into<String>)`](crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder::container_instance) / [`set_container_instance(Option<String>)`](crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder::set_container_instance): <p>The container instance ID or full ARN of the container instance. For more information about the ARN format, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-account-settings.html#ecs-resource-ids">Amazon Resource Name (ARN)</a> in the <i>Amazon ECS Developer Guide</i>.</p>
    ///   - [`cluster(impl Into<String>)`](crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder::cluster) / [`set_cluster(Option<String>)`](crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder::set_cluster): <p>The short name or full Amazon Resource Name (ARN) of the cluster that the container instance belongs to.</p>
    /// - On success, responds with [`DiscoverPollEndpointOutput`](crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput) with field(s):
    ///   - [`endpoint(Option<String>)`](crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput::endpoint): <p>The endpoint for the Amazon ECS agent to poll.</p>
    ///   - [`telemetry_endpoint(Option<String>)`](crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput::telemetry_endpoint): <p>The telemetry endpoint for the Amazon ECS agent.</p>
    ///   - [`service_connect_endpoint(Option<String>)`](crate::operation::discover_poll_endpoint::DiscoverPollEndpointOutput::service_connect_endpoint): <p>The endpoint for the Amazon ECS agent to poll for Service Connect configuration. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
    /// - On failure, responds with [`SdkError<DiscoverPollEndpointError>`](crate::operation::discover_poll_endpoint::DiscoverPollEndpointError)
    pub fn discover_poll_endpoint(
        &self,
    ) -> crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder {
        crate::operation::discover_poll_endpoint::builders::DiscoverPollEndpointFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

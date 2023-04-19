// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyTrafficMirrorFilterNetworkServices`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_filter_id(impl Into<String>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::traffic_mirror_filter_id) / [`set_traffic_mirror_filter_id(Option<String>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::set_traffic_mirror_filter_id): <p>The ID of the Traffic Mirror filter.</p>
    ///   - [`add_network_services(Vec<TrafficMirrorNetworkService>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::add_network_services) / [`set_add_network_services(Option<Vec<TrafficMirrorNetworkService>>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::set_add_network_services): <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    ///   - [`remove_network_services(Vec<TrafficMirrorNetworkService>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::remove_network_services) / [`set_remove_network_services(Option<Vec<TrafficMirrorNetworkService>>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::set_remove_network_services): <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    ///   - [`dry_run(bool)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ModifyTrafficMirrorFilterNetworkServicesOutput`](crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput) with field(s):
    ///   - [`traffic_mirror_filter(Option<TrafficMirrorFilter>)`](crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput::traffic_mirror_filter): <p>The Traffic Mirror filter that the network service is associated with.</p>
    /// - On failure, responds with [`SdkError<ModifyTrafficMirrorFilterNetworkServicesError>`](crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError)
    pub fn modify_traffic_mirror_filter_network_services(&self) -> crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder{
        crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesFluentBuilder::new(self.handle.clone())
    }
}

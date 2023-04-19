// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTrafficMirrorFilter`](crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_filter_id(impl Into<String>)`](crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder::traffic_mirror_filter_id) / [`set_traffic_mirror_filter_id(Option<String>)`](crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder::set_traffic_mirror_filter_id): <p>The ID of the Traffic Mirror filter.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteTrafficMirrorFilterOutput`](crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterOutput) with field(s):
    ///   - [`traffic_mirror_filter_id(Option<String>)`](crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterOutput::traffic_mirror_filter_id): <p>The ID of the Traffic Mirror filter.</p>
    /// - On failure, responds with [`SdkError<DeleteTrafficMirrorFilterError>`](crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError)
    pub fn delete_traffic_mirror_filter(&self) -> crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder{
        crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterFluentBuilder::new(self.handle.clone())
    }
}

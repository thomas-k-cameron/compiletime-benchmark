// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTrafficMirrorSession`](crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`traffic_mirror_session_id(impl Into<String>)`](crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder::traffic_mirror_session_id) / [`set_traffic_mirror_session_id(Option<String>)`](crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder::set_traffic_mirror_session_id): <p>The ID of the Traffic Mirror session.</p>
    ///   - [`dry_run(bool)`](crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DeleteTrafficMirrorSessionOutput`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput) with field(s):
    ///   - [`traffic_mirror_session_id(Option<String>)`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionOutput::traffic_mirror_session_id): <p>The ID of the deleted Traffic Mirror session.</p>
    /// - On failure, responds with [`SdkError<DeleteTrafficMirrorSessionError>`](crate::operation::delete_traffic_mirror_session::DeleteTrafficMirrorSessionError)
    pub fn delete_traffic_mirror_session(&self) -> crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder{
        crate::operation::delete_traffic_mirror_session::builders::DeleteTrafficMirrorSessionFluentBuilder::new(self.handle.clone())
    }
}

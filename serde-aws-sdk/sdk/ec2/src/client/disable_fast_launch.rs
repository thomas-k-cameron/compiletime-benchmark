// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableFastLaunch`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`image_id(impl Into<String>)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::image_id) / [`set_image_id(Option<String>)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::set_image_id): <p>The ID of the image for which you’re turning off faster launching, and removing pre-provisioned snapshots.</p>
    ///   - [`force(bool)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::force) / [`set_force(Option<bool>)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::set_force): <p>Forces the image settings to turn off faster launching for your Windows AMI. This parameter overrides any errors that are encountered while cleaning up resources in your account.</p>
    ///   - [`dry_run(bool)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`DisableFastLaunchOutput`](crate::operation::disable_fast_launch::DisableFastLaunchOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::image_id): <p>The ID of the image for which faster-launching has been turned off.</p>
    ///   - [`resource_type(Option<FastLaunchResourceType>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::resource_type): <p>The pre-provisioning resource type that must be cleaned after turning off faster launching for the Windows AMI. Supported values include: <code>snapshot</code>.</p>
    ///   - [`snapshot_configuration(Option<FastLaunchSnapshotConfigurationResponse>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::snapshot_configuration): <p>Parameters that were used for faster launching for the Windows AMI before faster launching was turned off. This informs the clean-up process.</p>
    ///   - [`launch_template(Option<FastLaunchLaunchTemplateSpecificationResponse>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::launch_template): <p>The launch template that was used to launch Windows instances from pre-provisioned snapshots.</p>
    ///   - [`max_parallel_launches(Option<i32>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::max_parallel_launches): <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows faster launching.</p>
    ///   - [`owner_id(Option<String>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::owner_id): <p>The owner of the Windows AMI for which faster launching was turned off.</p>
    ///   - [`state(Option<FastLaunchStateCode>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::state): <p>The current state of faster launching for the specified Windows AMI.</p>
    ///   - [`state_transition_reason(Option<String>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::state_transition_reason): <p>The reason that the state changed for faster launching for the Windows AMI.</p>
    ///   - [`state_transition_time(Option<DateTime>)`](crate::operation::disable_fast_launch::DisableFastLaunchOutput::state_transition_time): <p>The time that the state changed for faster launching for the Windows AMI.</p>
    /// - On failure, responds with [`SdkError<DisableFastLaunchError>`](crate::operation::disable_fast_launch::DisableFastLaunchError)
    pub fn disable_fast_launch(
        &self,
    ) -> crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder {
        crate::operation::disable_fast_launch::builders::DisableFastLaunchFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
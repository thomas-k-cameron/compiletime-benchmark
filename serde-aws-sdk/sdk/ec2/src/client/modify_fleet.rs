// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyFleet`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`excess_capacity_termination_policy(FleetExcessCapacityTerminationPolicy)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::excess_capacity_termination_policy) / [`set_excess_capacity_termination_policy(Option<FleetExcessCapacityTerminationPolicy>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_excess_capacity_termination_policy): <p>Indicates whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2 Fleet.</p>  <p>Supported only for fleets of type <code>maintain</code>.</p>
    ///   - [`launch_template_configs(Vec<FleetLaunchTemplateConfigRequest>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::launch_template_configs) / [`set_launch_template_configs(Option<Vec<FleetLaunchTemplateConfigRequest>>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_launch_template_configs): <p>The launch template and overrides.</p>
    ///   - [`fleet_id(impl Into<String>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::fleet_id) / [`set_fleet_id(Option<String>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_fleet_id): <p>The ID of the EC2 Fleet.</p>
    ///   - [`target_capacity_specification(TargetCapacitySpecificationRequest)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::target_capacity_specification) / [`set_target_capacity_specification(Option<TargetCapacitySpecificationRequest>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_target_capacity_specification): <p>The size of the EC2 Fleet.</p>
    ///   - [`context(impl Into<String>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::context) / [`set_context(Option<String>)`](crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::set_context): <p>Reserved.</p>
    /// - On success, responds with [`ModifyFleetOutput`](crate::operation::modify_fleet::ModifyFleetOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_fleet::ModifyFleetOutput::return): <p>If the request succeeds, the response returns <code>true</code>. If the request fails, no response is returned, and instead an error message is returned.</p>
    /// - On failure, responds with [`SdkError<ModifyFleetError>`](crate::operation::modify_fleet::ModifyFleetError)
    pub fn modify_fleet(
        &self,
    ) -> crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder {
        crate::operation::modify_fleet::builders::ModifyFleetFluentBuilder::new(self.handle.clone())
    }
}

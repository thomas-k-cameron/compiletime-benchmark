// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes overrides for a launch template.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct LaunchTemplateOverrides {
    /// <p>The instance type.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<crate::types::InstanceType>,
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    #[doc(hidden)]
    pub spot_price: std::option::Option<std::string::String>,
    /// <p>The ID of the subnet in which to launch the instances.</p>
    #[doc(hidden)]
    pub subnet_id: std::option::Option<std::string::String>,
    /// <p>The Availability Zone in which to launch the instances.</p>
    #[doc(hidden)]
    pub availability_zone: std::option::Option<std::string::String>,
    /// <p>The number of units provided by the specified instance type.</p>
    #[doc(hidden)]
    pub weighted_capacity: std::option::Option<f64>,
    /// <p>The priority for the launch template override. The highest priority is launched first.</p>
    /// <p>If <code>OnDemandAllocationStrategy</code> is set to <code>prioritized</code>, Spot Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity.</p>
    /// <p>If the Spot <code>AllocationStrategy</code> is set to <code>capacityOptimizedPrioritized</code>, Spot Fleet uses priority on a best-effort basis to determine which launch template override to use in fulfilling Spot capacity, but optimizes for capacity first.</p>
    /// <p>Valid values are whole numbers starting at <code>0</code>. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. You can set the same priority for different launch template overrides.</p>
    #[doc(hidden)]
    pub priority: std::option::Option<f64>,
    /// <p>The instance requirements. When you specify instance requirements, Amazon EC2 will identify instance types with the provided requirements, and then use your On-Demand and Spot allocation strategies to launch instances from these instance types, in the same way as when you specify a list of instance types.</p> <note>
    /// <p>If you specify <code>InstanceRequirements</code>, you can't specify <code>InstanceType</code>.</p>
    /// </note>
    #[doc(hidden)]
    pub instance_requirements: std::option::Option<crate::types::InstanceRequirements>,
}
impl LaunchTemplateOverrides {
    /// <p>The instance type.</p>
    pub fn instance_type(&self) -> std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn spot_price(&self) -> std::option::Option<&str> {
        self.spot_price.as_deref()
    }
    /// <p>The ID of the subnet in which to launch the instances.</p>
    pub fn subnet_id(&self) -> std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The Availability Zone in which to launch the instances.</p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The number of units provided by the specified instance type.</p>
    pub fn weighted_capacity(&self) -> std::option::Option<f64> {
        self.weighted_capacity
    }
    /// <p>The priority for the launch template override. The highest priority is launched first.</p>
    /// <p>If <code>OnDemandAllocationStrategy</code> is set to <code>prioritized</code>, Spot Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity.</p>
    /// <p>If the Spot <code>AllocationStrategy</code> is set to <code>capacityOptimizedPrioritized</code>, Spot Fleet uses priority on a best-effort basis to determine which launch template override to use in fulfilling Spot capacity, but optimizes for capacity first.</p>
    /// <p>Valid values are whole numbers starting at <code>0</code>. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. You can set the same priority for different launch template overrides.</p>
    pub fn priority(&self) -> std::option::Option<f64> {
        self.priority
    }
    /// <p>The instance requirements. When you specify instance requirements, Amazon EC2 will identify instance types with the provided requirements, and then use your On-Demand and Spot allocation strategies to launch instances from these instance types, in the same way as when you specify a list of instance types.</p> <note>
    /// <p>If you specify <code>InstanceRequirements</code>, you can't specify <code>InstanceType</code>.</p>
    /// </note>
    pub fn instance_requirements(
        &self,
    ) -> std::option::Option<&crate::types::InstanceRequirements> {
        self.instance_requirements.as_ref()
    }
}
impl LaunchTemplateOverrides {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateOverrides`](crate::types::LaunchTemplateOverrides).
    pub fn builder() -> crate::types::builders::LaunchTemplateOverridesBuilder {
        crate::types::builders::LaunchTemplateOverridesBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateOverrides;
/// A builder for [`LaunchTemplateOverrides`](crate::types::LaunchTemplateOverrides).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct LaunchTemplateOverridesBuilder {
    pub(crate) instance_type: std::option::Option<crate::types::InstanceType>,
    pub(crate) spot_price: std::option::Option<std::string::String>,
    pub(crate) subnet_id: std::option::Option<std::string::String>,
    pub(crate) availability_zone: std::option::Option<std::string::String>,
    pub(crate) weighted_capacity: std::option::Option<f64>,
    pub(crate) priority: std::option::Option<f64>,
    pub(crate) instance_requirements: std::option::Option<crate::types::InstanceRequirements>,
}
impl LaunchTemplateOverridesBuilder {
    /// <p>The instance type.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = Some(input);
        self
    }
    /// <p>The instance type.</p>
    pub fn set_instance_type(
        mut self,
        input: std::option::Option<crate::types::InstanceType>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn spot_price(mut self, input: impl Into<std::string::String>) -> Self {
        self.spot_price = Some(input.into());
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn set_spot_price(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.spot_price = input;
        self
    }
    /// <p>The ID of the subnet in which to launch the instances.</p>
    pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.subnet_id = Some(input.into());
        self
    }
    /// <p>The ID of the subnet in which to launch the instances.</p>
    pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The Availability Zone in which to launch the instances.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.availability_zone = Some(input.into());
        self
    }
    /// <p>The Availability Zone in which to launch the instances.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The number of units provided by the specified instance type.</p>
    pub fn weighted_capacity(mut self, input: f64) -> Self {
        self.weighted_capacity = Some(input);
        self
    }
    /// <p>The number of units provided by the specified instance type.</p>
    pub fn set_weighted_capacity(mut self, input: std::option::Option<f64>) -> Self {
        self.weighted_capacity = input;
        self
    }
    /// <p>The priority for the launch template override. The highest priority is launched first.</p>
    /// <p>If <code>OnDemandAllocationStrategy</code> is set to <code>prioritized</code>, Spot Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity.</p>
    /// <p>If the Spot <code>AllocationStrategy</code> is set to <code>capacityOptimizedPrioritized</code>, Spot Fleet uses priority on a best-effort basis to determine which launch template override to use in fulfilling Spot capacity, but optimizes for capacity first.</p>
    /// <p>Valid values are whole numbers starting at <code>0</code>. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. You can set the same priority for different launch template overrides.</p>
    pub fn priority(mut self, input: f64) -> Self {
        self.priority = Some(input);
        self
    }
    /// <p>The priority for the launch template override. The highest priority is launched first.</p>
    /// <p>If <code>OnDemandAllocationStrategy</code> is set to <code>prioritized</code>, Spot Fleet uses priority to determine which launch template override to use first in fulfilling On-Demand capacity.</p>
    /// <p>If the Spot <code>AllocationStrategy</code> is set to <code>capacityOptimizedPrioritized</code>, Spot Fleet uses priority on a best-effort basis to determine which launch template override to use in fulfilling Spot capacity, but optimizes for capacity first.</p>
    /// <p>Valid values are whole numbers starting at <code>0</code>. The lower the number, the higher the priority. If no number is set, the launch template override has the lowest priority. You can set the same priority for different launch template overrides.</p>
    pub fn set_priority(mut self, input: std::option::Option<f64>) -> Self {
        self.priority = input;
        self
    }
    /// <p>The instance requirements. When you specify instance requirements, Amazon EC2 will identify instance types with the provided requirements, and then use your On-Demand and Spot allocation strategies to launch instances from these instance types, in the same way as when you specify a list of instance types.</p> <note>
    /// <p>If you specify <code>InstanceRequirements</code>, you can't specify <code>InstanceType</code>.</p>
    /// </note>
    pub fn instance_requirements(mut self, input: crate::types::InstanceRequirements) -> Self {
        self.instance_requirements = Some(input);
        self
    }
    /// <p>The instance requirements. When you specify instance requirements, Amazon EC2 will identify instance types with the provided requirements, and then use your On-Demand and Spot allocation strategies to launch instances from these instance types, in the same way as when you specify a list of instance types.</p> <note>
    /// <p>If you specify <code>InstanceRequirements</code>, you can't specify <code>InstanceType</code>.</p>
    /// </note>
    pub fn set_instance_requirements(
        mut self,
        input: std::option::Option<crate::types::InstanceRequirements>,
    ) -> Self {
        self.instance_requirements = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateOverrides`](crate::types::LaunchTemplateOverrides).
    pub fn build(self) -> crate::types::LaunchTemplateOverrides {
        crate::types::LaunchTemplateOverrides {
            instance_type: self.instance_type,
            spot_price: self.spot_price,
            subnet_id: self.subnet_id,
            availability_zone: self.availability_zone,
            weighted_capacity: self.weighted_capacity,
            priority: self.priority,
            instance_requirements: self.instance_requirements,
        }
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the configuration of Spot Instances in an EC2 Fleet.</p>
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
pub struct SpotOptions {
    /// <p>The strategy that determines how to allocate the target Spot Instance capacity across the Spot Instance pools specified by the EC2 Fleet launch configuration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-allocation-strategy.html">Allocation strategies for Spot Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <dl>
    /// <dt>
    /// price-capacity-optimized (recommended)
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. EC2 Fleet then requests Spot Instances from the lowest priced of these pools.</p>
    /// </dd>
    /// <dt>
    /// capacity-optimized
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. To give certain instance types a higher chance of launching first, use <code>capacity-optimized-prioritized</code>. Set a priority for each instance type by using the <code>Priority</code> parameter for <code>LaunchTemplateOverrides</code>. You can assign the same priority to different <code>LaunchTemplateOverrides</code>. EC2 implements the priorities on a best-effort basis, but optimizes for capacity first. <code>capacity-optimized-prioritized</code> is supported only if your EC2 Fleet uses a launch template. Note that if the On-Demand <code>AllocationStrategy</code> is set to <code>prioritized</code>, the same priority is applied when fulfilling On-Demand capacity.</p>
    /// </dd>
    /// <dt>
    /// diversified
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from all of the Spot Instance pools that you specify.</p>
    /// </dd>
    /// <dt>
    /// lowest-price
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from the lowest priced Spot Instance pool that has available capacity. If the lowest priced pool doesn't have available capacity, the Spot Instances come from the next lowest priced pool that has available capacity. If a pool runs out of capacity before fulfilling your desired capacity, EC2 Fleet will continue to fulfill your request by drawing from the next lowest priced pool. To ensure that your desired capacity is met, you might receive Spot Instances from several pools. Because this strategy only considers instance price and not capacity availability, it might lead to high interruption rates.</p>
    /// </dd>
    /// </dl>
    /// <p>Default: <code>lowest-price</code> </p>
    #[doc(hidden)]
    pub allocation_strategy: std::option::Option<crate::types::SpotAllocationStrategy>,
    /// <p>The strategies for managing your workloads on your Spot Instances that will be interrupted. Currently only the capacity rebalance strategy is available.</p>
    #[doc(hidden)]
    pub maintenance_strategies: std::option::Option<crate::types::FleetSpotMaintenanceStrategies>,
    /// <p>The behavior when a Spot Instance is interrupted.</p>
    /// <p>Default: <code>terminate</code> </p>
    #[doc(hidden)]
    pub instance_interruption_behavior:
        std::option::Option<crate::types::SpotInstanceInterruptionBehavior>,
    /// <p>The number of Spot pools across which to allocate your target Spot capacity. Supported only when <code>AllocationStrategy</code> is set to <code>lowest-price</code>. EC2 Fleet selects the cheapest Spot pools and evenly allocates your target Spot capacity across the number of Spot pools that you specify.</p>
    /// <p>Note that EC2 Fleet attempts to draw Spot Instances from the number of pools that you specify on a best effort basis. If a pool runs out of Spot capacity before fulfilling your target capacity, EC2 Fleet will continue to fulfill your request by drawing from the next cheapest pool. To ensure that your target capacity is met, you might receive Spot Instances from more than the number of pools that you specified. Similarly, if most of the pools have no Spot capacity, you might receive your full target capacity from fewer than the number of pools that you specified.</p>
    #[doc(hidden)]
    pub instance_pools_to_use_count: std::option::Option<i32>,
    /// <p>Indicates that the fleet uses a single instance type to launch all Spot Instances in the fleet.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    #[doc(hidden)]
    pub single_instance_type: std::option::Option<bool>,
    /// <p>Indicates that the fleet launches all Spot Instances into a single Availability Zone.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    #[doc(hidden)]
    pub single_availability_zone: std::option::Option<bool>,
    /// <p>The minimum target capacity for Spot Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    /// <p>At least one of the following must be specified: <code>SingleAvailabilityZone</code> | <code>SingleInstanceType</code> </p>
    #[doc(hidden)]
    pub min_target_capacity: std::option::Option<i32>,
    /// <p>The maximum amount per hour for Spot Instances that you're willing to pay. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    #[doc(hidden)]
    pub max_total_price: std::option::Option<std::string::String>,
}
impl SpotOptions {
    /// <p>The strategy that determines how to allocate the target Spot Instance capacity across the Spot Instance pools specified by the EC2 Fleet launch configuration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-allocation-strategy.html">Allocation strategies for Spot Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <dl>
    /// <dt>
    /// price-capacity-optimized (recommended)
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. EC2 Fleet then requests Spot Instances from the lowest priced of these pools.</p>
    /// </dd>
    /// <dt>
    /// capacity-optimized
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. To give certain instance types a higher chance of launching first, use <code>capacity-optimized-prioritized</code>. Set a priority for each instance type by using the <code>Priority</code> parameter for <code>LaunchTemplateOverrides</code>. You can assign the same priority to different <code>LaunchTemplateOverrides</code>. EC2 implements the priorities on a best-effort basis, but optimizes for capacity first. <code>capacity-optimized-prioritized</code> is supported only if your EC2 Fleet uses a launch template. Note that if the On-Demand <code>AllocationStrategy</code> is set to <code>prioritized</code>, the same priority is applied when fulfilling On-Demand capacity.</p>
    /// </dd>
    /// <dt>
    /// diversified
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from all of the Spot Instance pools that you specify.</p>
    /// </dd>
    /// <dt>
    /// lowest-price
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from the lowest priced Spot Instance pool that has available capacity. If the lowest priced pool doesn't have available capacity, the Spot Instances come from the next lowest priced pool that has available capacity. If a pool runs out of capacity before fulfilling your desired capacity, EC2 Fleet will continue to fulfill your request by drawing from the next lowest priced pool. To ensure that your desired capacity is met, you might receive Spot Instances from several pools. Because this strategy only considers instance price and not capacity availability, it might lead to high interruption rates.</p>
    /// </dd>
    /// </dl>
    /// <p>Default: <code>lowest-price</code> </p>
    pub fn allocation_strategy(
        &self,
    ) -> std::option::Option<&crate::types::SpotAllocationStrategy> {
        self.allocation_strategy.as_ref()
    }
    /// <p>The strategies for managing your workloads on your Spot Instances that will be interrupted. Currently only the capacity rebalance strategy is available.</p>
    pub fn maintenance_strategies(
        &self,
    ) -> std::option::Option<&crate::types::FleetSpotMaintenanceStrategies> {
        self.maintenance_strategies.as_ref()
    }
    /// <p>The behavior when a Spot Instance is interrupted.</p>
    /// <p>Default: <code>terminate</code> </p>
    pub fn instance_interruption_behavior(
        &self,
    ) -> std::option::Option<&crate::types::SpotInstanceInterruptionBehavior> {
        self.instance_interruption_behavior.as_ref()
    }
    /// <p>The number of Spot pools across which to allocate your target Spot capacity. Supported only when <code>AllocationStrategy</code> is set to <code>lowest-price</code>. EC2 Fleet selects the cheapest Spot pools and evenly allocates your target Spot capacity across the number of Spot pools that you specify.</p>
    /// <p>Note that EC2 Fleet attempts to draw Spot Instances from the number of pools that you specify on a best effort basis. If a pool runs out of Spot capacity before fulfilling your target capacity, EC2 Fleet will continue to fulfill your request by drawing from the next cheapest pool. To ensure that your target capacity is met, you might receive Spot Instances from more than the number of pools that you specified. Similarly, if most of the pools have no Spot capacity, you might receive your full target capacity from fewer than the number of pools that you specified.</p>
    pub fn instance_pools_to_use_count(&self) -> std::option::Option<i32> {
        self.instance_pools_to_use_count
    }
    /// <p>Indicates that the fleet uses a single instance type to launch all Spot Instances in the fleet.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn single_instance_type(&self) -> std::option::Option<bool> {
        self.single_instance_type
    }
    /// <p>Indicates that the fleet launches all Spot Instances into a single Availability Zone.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn single_availability_zone(&self) -> std::option::Option<bool> {
        self.single_availability_zone
    }
    /// <p>The minimum target capacity for Spot Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    /// <p>At least one of the following must be specified: <code>SingleAvailabilityZone</code> | <code>SingleInstanceType</code> </p>
    pub fn min_target_capacity(&self) -> std::option::Option<i32> {
        self.min_target_capacity
    }
    /// <p>The maximum amount per hour for Spot Instances that you're willing to pay. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn max_total_price(&self) -> std::option::Option<&str> {
        self.max_total_price.as_deref()
    }
}
impl SpotOptions {
    /// Creates a new builder-style object to manufacture [`SpotOptions`](crate::types::SpotOptions).
    pub fn builder() -> crate::types::builders::SpotOptionsBuilder {
        crate::types::builders::SpotOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SpotOptions;
/// A builder for [`SpotOptions`](crate::types::SpotOptions).
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
pub struct SpotOptionsBuilder {
    pub(crate) allocation_strategy: std::option::Option<crate::types::SpotAllocationStrategy>,
    pub(crate) maintenance_strategies:
        std::option::Option<crate::types::FleetSpotMaintenanceStrategies>,
    pub(crate) instance_interruption_behavior:
        std::option::Option<crate::types::SpotInstanceInterruptionBehavior>,
    pub(crate) instance_pools_to_use_count: std::option::Option<i32>,
    pub(crate) single_instance_type: std::option::Option<bool>,
    pub(crate) single_availability_zone: std::option::Option<bool>,
    pub(crate) min_target_capacity: std::option::Option<i32>,
    pub(crate) max_total_price: std::option::Option<std::string::String>,
}
impl SpotOptionsBuilder {
    /// <p>The strategy that determines how to allocate the target Spot Instance capacity across the Spot Instance pools specified by the EC2 Fleet launch configuration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-allocation-strategy.html">Allocation strategies for Spot Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <dl>
    /// <dt>
    /// price-capacity-optimized (recommended)
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. EC2 Fleet then requests Spot Instances from the lowest priced of these pools.</p>
    /// </dd>
    /// <dt>
    /// capacity-optimized
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. To give certain instance types a higher chance of launching first, use <code>capacity-optimized-prioritized</code>. Set a priority for each instance type by using the <code>Priority</code> parameter for <code>LaunchTemplateOverrides</code>. You can assign the same priority to different <code>LaunchTemplateOverrides</code>. EC2 implements the priorities on a best-effort basis, but optimizes for capacity first. <code>capacity-optimized-prioritized</code> is supported only if your EC2 Fleet uses a launch template. Note that if the On-Demand <code>AllocationStrategy</code> is set to <code>prioritized</code>, the same priority is applied when fulfilling On-Demand capacity.</p>
    /// </dd>
    /// <dt>
    /// diversified
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from all of the Spot Instance pools that you specify.</p>
    /// </dd>
    /// <dt>
    /// lowest-price
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from the lowest priced Spot Instance pool that has available capacity. If the lowest priced pool doesn't have available capacity, the Spot Instances come from the next lowest priced pool that has available capacity. If a pool runs out of capacity before fulfilling your desired capacity, EC2 Fleet will continue to fulfill your request by drawing from the next lowest priced pool. To ensure that your desired capacity is met, you might receive Spot Instances from several pools. Because this strategy only considers instance price and not capacity availability, it might lead to high interruption rates.</p>
    /// </dd>
    /// </dl>
    /// <p>Default: <code>lowest-price</code> </p>
    pub fn allocation_strategy(mut self, input: crate::types::SpotAllocationStrategy) -> Self {
        self.allocation_strategy = Some(input);
        self
    }
    /// <p>The strategy that determines how to allocate the target Spot Instance capacity across the Spot Instance pools specified by the EC2 Fleet launch configuration. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-allocation-strategy.html">Allocation strategies for Spot Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <dl>
    /// <dt>
    /// price-capacity-optimized (recommended)
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. EC2 Fleet then requests Spot Instances from the lowest priced of these pools.</p>
    /// </dd>
    /// <dt>
    /// capacity-optimized
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet identifies the pools with the highest capacity availability for the number of instances that are launching. This means that we will request Spot Instances from the pools that we believe have the lowest chance of interruption in the near term. To give certain instance types a higher chance of launching first, use <code>capacity-optimized-prioritized</code>. Set a priority for each instance type by using the <code>Priority</code> parameter for <code>LaunchTemplateOverrides</code>. You can assign the same priority to different <code>LaunchTemplateOverrides</code>. EC2 implements the priorities on a best-effort basis, but optimizes for capacity first. <code>capacity-optimized-prioritized</code> is supported only if your EC2 Fleet uses a launch template. Note that if the On-Demand <code>AllocationStrategy</code> is set to <code>prioritized</code>, the same priority is applied when fulfilling On-Demand capacity.</p>
    /// </dd>
    /// <dt>
    /// diversified
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from all of the Spot Instance pools that you specify.</p>
    /// </dd>
    /// <dt>
    /// lowest-price
    /// </dt>
    /// <dd>
    /// <p>EC2 Fleet requests instances from the lowest priced Spot Instance pool that has available capacity. If the lowest priced pool doesn't have available capacity, the Spot Instances come from the next lowest priced pool that has available capacity. If a pool runs out of capacity before fulfilling your desired capacity, EC2 Fleet will continue to fulfill your request by drawing from the next lowest priced pool. To ensure that your desired capacity is met, you might receive Spot Instances from several pools. Because this strategy only considers instance price and not capacity availability, it might lead to high interruption rates.</p>
    /// </dd>
    /// </dl>
    /// <p>Default: <code>lowest-price</code> </p>
    pub fn set_allocation_strategy(
        mut self,
        input: std::option::Option<crate::types::SpotAllocationStrategy>,
    ) -> Self {
        self.allocation_strategy = input;
        self
    }
    /// <p>The strategies for managing your workloads on your Spot Instances that will be interrupted. Currently only the capacity rebalance strategy is available.</p>
    pub fn maintenance_strategies(
        mut self,
        input: crate::types::FleetSpotMaintenanceStrategies,
    ) -> Self {
        self.maintenance_strategies = Some(input);
        self
    }
    /// <p>The strategies for managing your workloads on your Spot Instances that will be interrupted. Currently only the capacity rebalance strategy is available.</p>
    pub fn set_maintenance_strategies(
        mut self,
        input: std::option::Option<crate::types::FleetSpotMaintenanceStrategies>,
    ) -> Self {
        self.maintenance_strategies = input;
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted.</p>
    /// <p>Default: <code>terminate</code> </p>
    pub fn instance_interruption_behavior(
        mut self,
        input: crate::types::SpotInstanceInterruptionBehavior,
    ) -> Self {
        self.instance_interruption_behavior = Some(input);
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted.</p>
    /// <p>Default: <code>terminate</code> </p>
    pub fn set_instance_interruption_behavior(
        mut self,
        input: std::option::Option<crate::types::SpotInstanceInterruptionBehavior>,
    ) -> Self {
        self.instance_interruption_behavior = input;
        self
    }
    /// <p>The number of Spot pools across which to allocate your target Spot capacity. Supported only when <code>AllocationStrategy</code> is set to <code>lowest-price</code>. EC2 Fleet selects the cheapest Spot pools and evenly allocates your target Spot capacity across the number of Spot pools that you specify.</p>
    /// <p>Note that EC2 Fleet attempts to draw Spot Instances from the number of pools that you specify on a best effort basis. If a pool runs out of Spot capacity before fulfilling your target capacity, EC2 Fleet will continue to fulfill your request by drawing from the next cheapest pool. To ensure that your target capacity is met, you might receive Spot Instances from more than the number of pools that you specified. Similarly, if most of the pools have no Spot capacity, you might receive your full target capacity from fewer than the number of pools that you specified.</p>
    pub fn instance_pools_to_use_count(mut self, input: i32) -> Self {
        self.instance_pools_to_use_count = Some(input);
        self
    }
    /// <p>The number of Spot pools across which to allocate your target Spot capacity. Supported only when <code>AllocationStrategy</code> is set to <code>lowest-price</code>. EC2 Fleet selects the cheapest Spot pools and evenly allocates your target Spot capacity across the number of Spot pools that you specify.</p>
    /// <p>Note that EC2 Fleet attempts to draw Spot Instances from the number of pools that you specify on a best effort basis. If a pool runs out of Spot capacity before fulfilling your target capacity, EC2 Fleet will continue to fulfill your request by drawing from the next cheapest pool. To ensure that your target capacity is met, you might receive Spot Instances from more than the number of pools that you specified. Similarly, if most of the pools have no Spot capacity, you might receive your full target capacity from fewer than the number of pools that you specified.</p>
    pub fn set_instance_pools_to_use_count(mut self, input: std::option::Option<i32>) -> Self {
        self.instance_pools_to_use_count = input;
        self
    }
    /// <p>Indicates that the fleet uses a single instance type to launch all Spot Instances in the fleet.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn single_instance_type(mut self, input: bool) -> Self {
        self.single_instance_type = Some(input);
        self
    }
    /// <p>Indicates that the fleet uses a single instance type to launch all Spot Instances in the fleet.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn set_single_instance_type(mut self, input: std::option::Option<bool>) -> Self {
        self.single_instance_type = input;
        self
    }
    /// <p>Indicates that the fleet launches all Spot Instances into a single Availability Zone.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn single_availability_zone(mut self, input: bool) -> Self {
        self.single_availability_zone = Some(input);
        self
    }
    /// <p>Indicates that the fleet launches all Spot Instances into a single Availability Zone.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    pub fn set_single_availability_zone(mut self, input: std::option::Option<bool>) -> Self {
        self.single_availability_zone = input;
        self
    }
    /// <p>The minimum target capacity for Spot Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    /// <p>At least one of the following must be specified: <code>SingleAvailabilityZone</code> | <code>SingleInstanceType</code> </p>
    pub fn min_target_capacity(mut self, input: i32) -> Self {
        self.min_target_capacity = Some(input);
        self
    }
    /// <p>The minimum target capacity for Spot Instances in the fleet. If the minimum target capacity is not reached, the fleet launches no instances.</p>
    /// <p>Supported only for fleets of type <code>instant</code>.</p>
    /// <p>At least one of the following must be specified: <code>SingleAvailabilityZone</code> | <code>SingleInstanceType</code> </p>
    pub fn set_min_target_capacity(mut self, input: std::option::Option<i32>) -> Self {
        self.min_target_capacity = input;
        self
    }
    /// <p>The maximum amount per hour for Spot Instances that you're willing to pay. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn max_total_price(mut self, input: impl Into<std::string::String>) -> Self {
        self.max_total_price = Some(input.into());
        self
    }
    /// <p>The maximum amount per hour for Spot Instances that you're willing to pay. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn set_max_total_price(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.max_total_price = input;
        self
    }
    /// Consumes the builder and constructs a [`SpotOptions`](crate::types::SpotOptions).
    pub fn build(self) -> crate::types::SpotOptions {
        crate::types::SpotOptions {
            allocation_strategy: self.allocation_strategy,
            maintenance_strategies: self.maintenance_strategies,
            instance_interruption_behavior: self.instance_interruption_behavior,
            instance_pools_to_use_count: self.instance_pools_to_use_count,
            single_instance_type: self.single_instance_type,
            single_availability_zone: self.single_availability_zone,
            min_target_capacity: self.min_target_capacity,
            max_total_price: self.max_total_price,
        }
    }
}

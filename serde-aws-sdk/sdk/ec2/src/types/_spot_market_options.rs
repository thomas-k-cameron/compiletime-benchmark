// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options for Spot Instances.</p>
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
pub struct SpotMarketOptions {
    /// <p>The maximum hourly price that you're willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    #[doc(hidden)]
    pub max_price: std::option::Option<std::string::String>,
    /// <p>The Spot Instance request type. For <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances">RunInstances</a>, persistent Spot Instance requests are only supported when the instance interruption behavior is either <code>hibernate</code> or <code>stop</code>.</p>
    #[doc(hidden)]
    pub spot_instance_type: std::option::Option<crate::types::SpotInstanceType>,
    /// <p>Deprecated.</p>
    #[doc(hidden)]
    pub block_duration_minutes: std::option::Option<i32>,
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). Supported only for persistent requests.</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it.</p> </li>
    /// <li> <p>For a one-time request, <code>ValidUntil</code> is not supported. The request remains active until all instances launch or you cancel the request.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub valid_until: std::option::Option<aws_smithy_types::DateTime>,
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    #[doc(hidden)]
    pub instance_interruption_behavior:
        std::option::Option<crate::types::InstanceInterruptionBehavior>,
}
impl SpotMarketOptions {
    /// <p>The maximum hourly price that you're willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn max_price(&self) -> std::option::Option<&str> {
        self.max_price.as_deref()
    }
    /// <p>The Spot Instance request type. For <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances">RunInstances</a>, persistent Spot Instance requests are only supported when the instance interruption behavior is either <code>hibernate</code> or <code>stop</code>.</p>
    pub fn spot_instance_type(&self) -> std::option::Option<&crate::types::SpotInstanceType> {
        self.spot_instance_type.as_ref()
    }
    /// <p>Deprecated.</p>
    pub fn block_duration_minutes(&self) -> std::option::Option<i32> {
        self.block_duration_minutes
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). Supported only for persistent requests.</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it.</p> </li>
    /// <li> <p>For a one-time request, <code>ValidUntil</code> is not supported. The request remains active until all instances launch or you cancel the request.</p> </li>
    /// </ul>
    pub fn valid_until(&self) -> std::option::Option<&aws_smithy_types::DateTime> {
        self.valid_until.as_ref()
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn instance_interruption_behavior(
        &self,
    ) -> std::option::Option<&crate::types::InstanceInterruptionBehavior> {
        self.instance_interruption_behavior.as_ref()
    }
}
impl SpotMarketOptions {
    /// Creates a new builder-style object to manufacture [`SpotMarketOptions`](crate::types::SpotMarketOptions).
    pub fn builder() -> crate::types::builders::SpotMarketOptionsBuilder {
        crate::types::builders::SpotMarketOptionsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::SpotMarketOptions;
/// A builder for [`SpotMarketOptions`](crate::types::SpotMarketOptions).
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
pub struct SpotMarketOptionsBuilder {
    pub(crate) max_price: std::option::Option<std::string::String>,
    pub(crate) spot_instance_type: std::option::Option<crate::types::SpotInstanceType>,
    pub(crate) block_duration_minutes: std::option::Option<i32>,
    pub(crate) valid_until: std::option::Option<aws_smithy_types::DateTime>,
    pub(crate) instance_interruption_behavior:
        std::option::Option<crate::types::InstanceInterruptionBehavior>,
}
impl SpotMarketOptionsBuilder {
    /// <p>The maximum hourly price that you're willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn max_price(mut self, input: impl Into<std::string::String>) -> Self {
        self.max_price = Some(input.into());
        self
    }
    /// <p>The maximum hourly price that you're willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your Spot Instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn set_max_price(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.max_price = input;
        self
    }
    /// <p>The Spot Instance request type. For <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances">RunInstances</a>, persistent Spot Instance requests are only supported when the instance interruption behavior is either <code>hibernate</code> or <code>stop</code>.</p>
    pub fn spot_instance_type(mut self, input: crate::types::SpotInstanceType) -> Self {
        self.spot_instance_type = Some(input);
        self
    }
    /// <p>The Spot Instance request type. For <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances">RunInstances</a>, persistent Spot Instance requests are only supported when the instance interruption behavior is either <code>hibernate</code> or <code>stop</code>.</p>
    pub fn set_spot_instance_type(
        mut self,
        input: std::option::Option<crate::types::SpotInstanceType>,
    ) -> Self {
        self.spot_instance_type = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn block_duration_minutes(mut self, input: i32) -> Self {
        self.block_duration_minutes = Some(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_block_duration_minutes(mut self, input: std::option::Option<i32>) -> Self {
        self.block_duration_minutes = input;
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). Supported only for persistent requests.</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it.</p> </li>
    /// <li> <p>For a one-time request, <code>ValidUntil</code> is not supported. The request remains active until all instances launch or you cancel the request.</p> </li>
    /// </ul>
    pub fn valid_until(mut self, input: aws_smithy_types::DateTime) -> Self {
        self.valid_until = Some(input);
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). Supported only for persistent requests.</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it.</p> </li>
    /// <li> <p>For a one-time request, <code>ValidUntil</code> is not supported. The request remains active until all instances launch or you cancel the request.</p> </li>
    /// </ul>
    pub fn set_valid_until(
        mut self,
        input: std::option::Option<aws_smithy_types::DateTime>,
    ) -> Self {
        self.valid_until = input;
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn instance_interruption_behavior(
        mut self,
        input: crate::types::InstanceInterruptionBehavior,
    ) -> Self {
        self.instance_interruption_behavior = Some(input);
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn set_instance_interruption_behavior(
        mut self,
        input: std::option::Option<crate::types::InstanceInterruptionBehavior>,
    ) -> Self {
        self.instance_interruption_behavior = input;
        self
    }
    /// Consumes the builder and constructs a [`SpotMarketOptions`](crate::types::SpotMarketOptions).
    pub fn build(self) -> crate::types::SpotMarketOptions {
        crate::types::SpotMarketOptions {
            max_price: self.max_price,
            spot_instance_type: self.spot_instance_type,
            block_duration_minutes: self.block_duration_minutes,
            valid_until: self.valid_until,
            instance_interruption_behavior: self.instance_interruption_behavior,
        }
    }
}

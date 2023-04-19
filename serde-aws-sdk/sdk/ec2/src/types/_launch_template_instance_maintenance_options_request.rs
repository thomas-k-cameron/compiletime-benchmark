// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The maintenance options of your instance.</p>
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
pub struct LaunchTemplateInstanceMaintenanceOptionsRequest {
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html#instance-configuration-recovery">Simplified automatic recovery</a>.</p>
    #[doc(hidden)]
    pub auto_recovery: std::option::Option<crate::types::LaunchTemplateAutoRecoveryState>,
}
impl LaunchTemplateInstanceMaintenanceOptionsRequest {
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html#instance-configuration-recovery">Simplified automatic recovery</a>.</p>
    pub fn auto_recovery(
        &self,
    ) -> std::option::Option<&crate::types::LaunchTemplateAutoRecoveryState> {
        self.auto_recovery.as_ref()
    }
}
impl LaunchTemplateInstanceMaintenanceOptionsRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateInstanceMaintenanceOptionsRequest`](crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest).
    pub fn builder(
    ) -> crate::types::builders::LaunchTemplateInstanceMaintenanceOptionsRequestBuilder {
        crate::types::builders::LaunchTemplateInstanceMaintenanceOptionsRequestBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest;
/// A builder for [`LaunchTemplateInstanceMaintenanceOptionsRequest`](crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest).
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
pub struct LaunchTemplateInstanceMaintenanceOptionsRequestBuilder {
    pub(crate) auto_recovery: std::option::Option<crate::types::LaunchTemplateAutoRecoveryState>,
}
impl LaunchTemplateInstanceMaintenanceOptionsRequestBuilder {
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html#instance-configuration-recovery">Simplified automatic recovery</a>.</p>
    pub fn auto_recovery(mut self, input: crate::types::LaunchTemplateAutoRecoveryState) -> Self {
        self.auto_recovery = Some(input);
        self
    }
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html#instance-configuration-recovery">Simplified automatic recovery</a>.</p>
    pub fn set_auto_recovery(
        mut self,
        input: std::option::Option<crate::types::LaunchTemplateAutoRecoveryState>,
    ) -> Self {
        self.auto_recovery = input;
        self
    }
    /// Consumes the builder and constructs a [`LaunchTemplateInstanceMaintenanceOptionsRequest`](crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest).
    pub fn build(self) -> crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest {
        crate::types::LaunchTemplateInstanceMaintenanceOptionsRequest {
            auto_recovery: self.auto_recovery,
        }
    }
}

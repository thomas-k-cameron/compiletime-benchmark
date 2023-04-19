// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>One of the methods which provide a way for you to quickly identify when a deployment has failed, and then to optionally roll back the failure to the last working deployment.</p>
/// <p>When the alarms are generated, Amazon ECS sets the service deployment to failed. Set the rollback parameter to have Amazon ECS to roll back your service to the last completed deployment after a failure.</p>
/// <p>You can only use the <code>DeploymentAlarms</code> method to detect failures when the <code>DeploymentController</code> is set to <code>ECS</code> (rolling update).</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i> <i>Amazon Elastic Container Service Developer Guide</i> </i>.</p>
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
pub struct DeploymentAlarms {
    /// <p>One or more CloudWatch alarm names. Use a "," to separate the alarms.</p>
    #[doc(hidden)]
    pub alarm_names: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Determines whether to use the CloudWatch alarm option in the service deployment process.</p>
    #[doc(hidden)]
    pub enable: bool,
    /// <p>Determines whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is used, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p>
    #[doc(hidden)]
    pub rollback: bool,
}
impl DeploymentAlarms {
    /// <p>One or more CloudWatch alarm names. Use a "," to separate the alarms.</p>
    pub fn alarm_names(&self) -> std::option::Option<&[std::string::String]> {
        self.alarm_names.as_deref()
    }
    /// <p>Determines whether to use the CloudWatch alarm option in the service deployment process.</p>
    pub fn enable(&self) -> bool {
        self.enable
    }
    /// <p>Determines whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is used, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p>
    pub fn rollback(&self) -> bool {
        self.rollback
    }
}
impl DeploymentAlarms {
    /// Creates a new builder-style object to manufacture [`DeploymentAlarms`](crate::types::DeploymentAlarms).
    pub fn builder() -> crate::types::builders::DeploymentAlarmsBuilder {
        crate::types::builders::DeploymentAlarmsBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DeploymentAlarms;
/// A builder for [`DeploymentAlarms`](crate::types::DeploymentAlarms).
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
pub struct DeploymentAlarmsBuilder {
    pub(crate) alarm_names: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) enable: std::option::Option<bool>,
    pub(crate) rollback: std::option::Option<bool>,
}
impl DeploymentAlarmsBuilder {
    /// Appends an item to `alarm_names`.
    ///
    /// To override the contents of this collection use [`set_alarm_names`](Self::set_alarm_names).
    ///
    /// <p>One or more CloudWatch alarm names. Use a "," to separate the alarms.</p>
    pub fn alarm_names(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.alarm_names.unwrap_or_default();
        v.push(input.into());
        self.alarm_names = Some(v);
        self
    }
    /// <p>One or more CloudWatch alarm names. Use a "," to separate the alarms.</p>
    pub fn set_alarm_names(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.alarm_names = input;
        self
    }
    /// <p>Determines whether to use the CloudWatch alarm option in the service deployment process.</p>
    pub fn enable(mut self, input: bool) -> Self {
        self.enable = Some(input);
        self
    }
    /// <p>Determines whether to use the CloudWatch alarm option in the service deployment process.</p>
    pub fn set_enable(mut self, input: std::option::Option<bool>) -> Self {
        self.enable = input;
        self
    }
    /// <p>Determines whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is used, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p>
    pub fn rollback(mut self, input: bool) -> Self {
        self.rollback = Some(input);
        self
    }
    /// <p>Determines whether to configure Amazon ECS to roll back the service if a service deployment fails. If rollback is used, when a service deployment fails, the service is rolled back to the last deployment that completed successfully.</p>
    pub fn set_rollback(mut self, input: std::option::Option<bool>) -> Self {
        self.rollback = input;
        self
    }
    /// Consumes the builder and constructs a [`DeploymentAlarms`](crate::types::DeploymentAlarms).
    pub fn build(self) -> crate::types::DeploymentAlarms {
        crate::types::DeploymentAlarms {
            alarm_names: self.alarm_names,
            enable: self.enable.unwrap_or_default(),
            rollback: self.rollback.unwrap_or_default(),
        }
    }
}

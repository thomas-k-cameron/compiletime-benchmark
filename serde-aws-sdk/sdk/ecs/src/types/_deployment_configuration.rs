// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Optional deployment parameters that control how many tasks run during a deployment and the ordering of stopping and starting tasks.</p>
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
pub struct DeploymentConfiguration {
    /// <note>
    /// <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type.</p>
    /// </note>
    /// <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can't reach a steady state. If you use the deployment circuit breaker, a service deployment will transition to a failed state and stop launching new tasks. If you use the rollback option, when a service deployment fails, the service is rolled back to the last deployment that completed successfully. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
    #[doc(hidden)]
    pub deployment_circuit_breaker: std::option::Option<crate::types::DeploymentCircuitBreaker>,
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service is using the <code>REPLICA</code> service scheduler and has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default <code>maximumPercent</code> value for a service using the <code>REPLICA</code> service scheduler is 200%.</p>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    #[doc(hidden)]
    pub maximum_percent: std::option::Option<i32>,
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the service scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. </p>
    /// <p>For services that <i>do not</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>A service is considered healthy if all essential containers within the tasks in the service pass their health checks.</p> </li>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for 40 seconds after a task reaches a <code>RUNNING</code> state before the task is counted towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has one or more essential containers with a health check defined, the service scheduler will wait for the task to reach a healthy status before counting it towards the minimum healthy percent total. A task is considered healthy when all essential containers within the task have passed their health checks. The amount of time the service scheduler can wait for is determined by the container health check settings. </p> </li>
    /// </ul>
    /// <p>For services are that <i>do</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has an essential container with a health check defined, the service scheduler will wait for both the task to reach a healthy status and the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// </ul>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    #[doc(hidden)]
    pub minimum_healthy_percent: std::option::Option<i32>,
    /// <p>Information about the CloudWatch alarms.</p>
    #[doc(hidden)]
    pub alarms: std::option::Option<crate::types::DeploymentAlarms>,
}
impl DeploymentConfiguration {
    /// <note>
    /// <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type.</p>
    /// </note>
    /// <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can't reach a steady state. If you use the deployment circuit breaker, a service deployment will transition to a failed state and stop launching new tasks. If you use the rollback option, when a service deployment fails, the service is rolled back to the last deployment that completed successfully. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
    pub fn deployment_circuit_breaker(
        &self,
    ) -> std::option::Option<&crate::types::DeploymentCircuitBreaker> {
        self.deployment_circuit_breaker.as_ref()
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service is using the <code>REPLICA</code> service scheduler and has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default <code>maximumPercent</code> value for a service using the <code>REPLICA</code> service scheduler is 200%.</p>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    pub fn maximum_percent(&self) -> std::option::Option<i32> {
        self.maximum_percent
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the service scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. </p>
    /// <p>For services that <i>do not</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>A service is considered healthy if all essential containers within the tasks in the service pass their health checks.</p> </li>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for 40 seconds after a task reaches a <code>RUNNING</code> state before the task is counted towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has one or more essential containers with a health check defined, the service scheduler will wait for the task to reach a healthy status before counting it towards the minimum healthy percent total. A task is considered healthy when all essential containers within the task have passed their health checks. The amount of time the service scheduler can wait for is determined by the container health check settings. </p> </li>
    /// </ul>
    /// <p>For services are that <i>do</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has an essential container with a health check defined, the service scheduler will wait for both the task to reach a healthy status and the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// </ul>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    pub fn minimum_healthy_percent(&self) -> std::option::Option<i32> {
        self.minimum_healthy_percent
    }
    /// <p>Information about the CloudWatch alarms.</p>
    pub fn alarms(&self) -> std::option::Option<&crate::types::DeploymentAlarms> {
        self.alarms.as_ref()
    }
}
impl DeploymentConfiguration {
    /// Creates a new builder-style object to manufacture [`DeploymentConfiguration`](crate::types::DeploymentConfiguration).
    pub fn builder() -> crate::types::builders::DeploymentConfigurationBuilder {
        crate::types::builders::DeploymentConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::DeploymentConfiguration;
/// A builder for [`DeploymentConfiguration`](crate::types::DeploymentConfiguration).
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
pub struct DeploymentConfigurationBuilder {
    pub(crate) deployment_circuit_breaker:
        std::option::Option<crate::types::DeploymentCircuitBreaker>,
    pub(crate) maximum_percent: std::option::Option<i32>,
    pub(crate) minimum_healthy_percent: std::option::Option<i32>,
    pub(crate) alarms: std::option::Option<crate::types::DeploymentAlarms>,
}
impl DeploymentConfigurationBuilder {
    /// <note>
    /// <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type.</p>
    /// </note>
    /// <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can't reach a steady state. If you use the deployment circuit breaker, a service deployment will transition to a failed state and stop launching new tasks. If you use the rollback option, when a service deployment fails, the service is rolled back to the last deployment that completed successfully. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
    pub fn deployment_circuit_breaker(
        mut self,
        input: crate::types::DeploymentCircuitBreaker,
    ) -> Self {
        self.deployment_circuit_breaker = Some(input);
        self
    }
    /// <note>
    /// <p>The deployment circuit breaker can only be used for services using the rolling update (<code>ECS</code>) deployment type.</p>
    /// </note>
    /// <p>The <b>deployment circuit breaker</b> determines whether a service deployment will fail if the service can't reach a steady state. If you use the deployment circuit breaker, a service deployment will transition to a failed state and stop launching new tasks. If you use the rollback option, when a service deployment fails, the service is rolled back to the last deployment that completed successfully. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/deployment-type-ecs.html">Rolling update</a> in the <i>Amazon Elastic Container Service Developer Guide</i> </p>
    pub fn set_deployment_circuit_breaker(
        mut self,
        input: std::option::Option<crate::types::DeploymentCircuitBreaker>,
    ) -> Self {
        self.deployment_circuit_breaker = input;
        self
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service is using the <code>REPLICA</code> service scheduler and has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default <code>maximumPercent</code> value for a service using the <code>REPLICA</code> service scheduler is 200%.</p>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    pub fn maximum_percent(mut self, input: i32) -> Self {
        self.maximum_percent = Some(input);
        self
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>maximumPercent</code> parameter represents an upper limit on the number of your service's tasks that are allowed in the <code>RUNNING</code> or <code>PENDING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded down to the nearest integer). This parameter enables you to define the deployment batch size. For example, if your service is using the <code>REPLICA</code> service scheduler and has a <code>desiredCount</code> of four tasks and a <code>maximumPercent</code> value of 200%, the scheduler may start four new tasks before stopping the four older tasks (provided that the cluster resources required to do this are available). The default <code>maximumPercent</code> value for a service using the <code>REPLICA</code> service scheduler is 200%.</p>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and tasks that use the EC2 launch type, the <b>maximum percent</b> value is set to the default value and is used to define the upper limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If the tasks in the service use the Fargate launch type, the maximum percent value is not used, although it is returned when describing your service.</p>
    pub fn set_maximum_percent(mut self, input: std::option::Option<i32>) -> Self {
        self.maximum_percent = input;
        self
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the service scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. </p>
    /// <p>For services that <i>do not</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>A service is considered healthy if all essential containers within the tasks in the service pass their health checks.</p> </li>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for 40 seconds after a task reaches a <code>RUNNING</code> state before the task is counted towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has one or more essential containers with a health check defined, the service scheduler will wait for the task to reach a healthy status before counting it towards the minimum healthy percent total. A task is considered healthy when all essential containers within the task have passed their health checks. The amount of time the service scheduler can wait for is determined by the container health check settings. </p> </li>
    /// </ul>
    /// <p>For services are that <i>do</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has an essential container with a health check defined, the service scheduler will wait for both the task to reach a healthy status and the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// </ul>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    pub fn minimum_healthy_percent(mut self, input: i32) -> Self {
        self.minimum_healthy_percent = Some(input);
        self
    }
    /// <p>If a service is using the rolling update (<code>ECS</code>) deployment type, the <code>minimumHealthyPercent</code> represents a lower limit on the number of your service's tasks that must remain in the <code>RUNNING</code> state during a deployment, as a percentage of the <code>desiredCount</code> (rounded up to the nearest integer). This parameter enables you to deploy without using additional cluster capacity. For example, if your service has a <code>desiredCount</code> of four tasks and a <code>minimumHealthyPercent</code> of 50%, the service scheduler may stop two existing tasks to free up cluster capacity before starting two new tasks. </p>
    /// <p>For services that <i>do not</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>A service is considered healthy if all essential containers within the tasks in the service pass their health checks.</p> </li>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for 40 seconds after a task reaches a <code>RUNNING</code> state before the task is counted towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has one or more essential containers with a health check defined, the service scheduler will wait for the task to reach a healthy status before counting it towards the minimum healthy percent total. A task is considered healthy when all essential containers within the task have passed their health checks. The amount of time the service scheduler can wait for is determined by the container health check settings. </p> </li>
    /// </ul>
    /// <p>For services are that <i>do</i> use a load balancer, the following should be noted:</p>
    /// <ul>
    /// <li> <p>If a task has no essential containers with a health check defined, the service scheduler will wait for the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// <li> <p>If a task has an essential container with a health check defined, the service scheduler will wait for both the task to reach a healthy status and the load balancer target group health check to return a healthy status before counting the task towards the minimum healthy percent total.</p> </li>
    /// </ul>
    /// <p>If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the EC2 launch type, the <b>minimum healthy percent</b> value is set to the default value and is used to define the lower limit on the number of the tasks in the service that remain in the <code>RUNNING</code> state while the container instances are in the <code>DRAINING</code> state. If a service is using either the blue/green (<code>CODE_DEPLOY</code>) or <code>EXTERNAL</code> deployment types and is running tasks that use the Fargate launch type, the minimum healthy percent value is not used, although it is returned when describing your service.</p>
    pub fn set_minimum_healthy_percent(mut self, input: std::option::Option<i32>) -> Self {
        self.minimum_healthy_percent = input;
        self
    }
    /// <p>Information about the CloudWatch alarms.</p>
    pub fn alarms(mut self, input: crate::types::DeploymentAlarms) -> Self {
        self.alarms = Some(input);
        self
    }
    /// <p>Information about the CloudWatch alarms.</p>
    pub fn set_alarms(
        mut self,
        input: std::option::Option<crate::types::DeploymentAlarms>,
    ) -> Self {
        self.alarms = input;
        self
    }
    /// Consumes the builder and constructs a [`DeploymentConfiguration`](crate::types::DeploymentConfiguration).
    pub fn build(self) -> crate::types::DeploymentConfiguration {
        crate::types::DeploymentConfiguration {
            deployment_circuit_breaker: self.deployment_circuit_breaker,
            maximum_percent: self.maximum_percent,
            minimum_healthy_percent: self.minimum_healthy_percent,
            alarms: self.alarms,
        }
    }
}
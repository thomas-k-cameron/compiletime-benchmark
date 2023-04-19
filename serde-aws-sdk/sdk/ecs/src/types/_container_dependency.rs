// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The dependencies defined for container startup and shutdown. A container can contain multiple dependencies. When a dependency is defined for container startup, for container shutdown it is reversed.</p>
/// <p>Your Amazon ECS container instances require at least version 1.26.0 of the container agent to use container dependencies. However, we recommend using the latest container agent version. For information about checking your agent version and updating to the latest version, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-update.html">Updating the Amazon ECS Container Agent</a> in the <i>Amazon Elastic Container Service Developer Guide</i>. If you're using an Amazon ECS-optimized Linux AMI, your instance needs at least version 1.26.0-1 of the <code>ecs-init</code> package. If your container instances are launched from version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> <note>
/// <p>For tasks that use the Fargate launch type, the task or service requires the following platforms:</p>
/// <ul>
/// <li> <p>Linux platform version <code>1.3.0</code> or later.</p> </li>
/// <li> <p>Windows platform version <code>1.0.0</code> or later.</p> </li>
/// </ul>
/// </note>
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
pub struct ContainerDependency {
    /// <p>The name of a container.</p>
    #[doc(hidden)]
    pub container_name: std::option::Option<std::string::String>,
    /// <p>The dependency condition of the container. The following are the available conditions and their behavior:</p>
    /// <ul>
    /// <li> <p> <code>START</code> - This condition emulates the behavior of links and volumes today. It validates that a dependent container is started before permitting other containers to start.</p> </li>
    /// <li> <p> <code>COMPLETE</code> - This condition validates that a dependent container runs to completion (exits) before permitting other containers to start. This can be useful for nonessential containers that run a script and then exit. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>SUCCESS</code> - This condition is the same as <code>COMPLETE</code>, but it also requires that the container exits with a <code>zero</code> status. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>HEALTHY</code> - This condition validates that the dependent container passes its Docker health check before permitting other containers to start. This requires that the dependent container has health checks configured. This condition is confirmed only at task startup.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub condition: std::option::Option<crate::types::ContainerCondition>,
}
impl ContainerDependency {
    /// <p>The name of a container.</p>
    pub fn container_name(&self) -> std::option::Option<&str> {
        self.container_name.as_deref()
    }
    /// <p>The dependency condition of the container. The following are the available conditions and their behavior:</p>
    /// <ul>
    /// <li> <p> <code>START</code> - This condition emulates the behavior of links and volumes today. It validates that a dependent container is started before permitting other containers to start.</p> </li>
    /// <li> <p> <code>COMPLETE</code> - This condition validates that a dependent container runs to completion (exits) before permitting other containers to start. This can be useful for nonessential containers that run a script and then exit. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>SUCCESS</code> - This condition is the same as <code>COMPLETE</code>, but it also requires that the container exits with a <code>zero</code> status. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>HEALTHY</code> - This condition validates that the dependent container passes its Docker health check before permitting other containers to start. This requires that the dependent container has health checks configured. This condition is confirmed only at task startup.</p> </li>
    /// </ul>
    pub fn condition(&self) -> std::option::Option<&crate::types::ContainerCondition> {
        self.condition.as_ref()
    }
}
impl ContainerDependency {
    /// Creates a new builder-style object to manufacture [`ContainerDependency`](crate::types::ContainerDependency).
    pub fn builder() -> crate::types::builders::ContainerDependencyBuilder {
        crate::types::builders::ContainerDependencyBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ContainerDependency;
/// A builder for [`ContainerDependency`](crate::types::ContainerDependency).
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
pub struct ContainerDependencyBuilder {
    pub(crate) container_name: std::option::Option<std::string::String>,
    pub(crate) condition: std::option::Option<crate::types::ContainerCondition>,
}
impl ContainerDependencyBuilder {
    /// <p>The name of a container.</p>
    pub fn container_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.container_name = Some(input.into());
        self
    }
    /// <p>The name of a container.</p>
    pub fn set_container_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.container_name = input;
        self
    }
    /// <p>The dependency condition of the container. The following are the available conditions and their behavior:</p>
    /// <ul>
    /// <li> <p> <code>START</code> - This condition emulates the behavior of links and volumes today. It validates that a dependent container is started before permitting other containers to start.</p> </li>
    /// <li> <p> <code>COMPLETE</code> - This condition validates that a dependent container runs to completion (exits) before permitting other containers to start. This can be useful for nonessential containers that run a script and then exit. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>SUCCESS</code> - This condition is the same as <code>COMPLETE</code>, but it also requires that the container exits with a <code>zero</code> status. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>HEALTHY</code> - This condition validates that the dependent container passes its Docker health check before permitting other containers to start. This requires that the dependent container has health checks configured. This condition is confirmed only at task startup.</p> </li>
    /// </ul>
    pub fn condition(mut self, input: crate::types::ContainerCondition) -> Self {
        self.condition = Some(input);
        self
    }
    /// <p>The dependency condition of the container. The following are the available conditions and their behavior:</p>
    /// <ul>
    /// <li> <p> <code>START</code> - This condition emulates the behavior of links and volumes today. It validates that a dependent container is started before permitting other containers to start.</p> </li>
    /// <li> <p> <code>COMPLETE</code> - This condition validates that a dependent container runs to completion (exits) before permitting other containers to start. This can be useful for nonessential containers that run a script and then exit. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>SUCCESS</code> - This condition is the same as <code>COMPLETE</code>, but it also requires that the container exits with a <code>zero</code> status. This condition can't be set on an essential container.</p> </li>
    /// <li> <p> <code>HEALTHY</code> - This condition validates that the dependent container passes its Docker health check before permitting other containers to start. This requires that the dependent container has health checks configured. This condition is confirmed only at task startup.</p> </li>
    /// </ul>
    pub fn set_condition(
        mut self,
        input: std::option::Option<crate::types::ContainerCondition>,
    ) -> Self {
        self.condition = input;
        self
    }
    /// Consumes the builder and constructs a [`ContainerDependency`](crate::types::ContainerDependency).
    pub fn build(self) -> crate::types::ContainerDependency {
        crate::types::ContainerDependency {
            container_name: self.container_name,
            condition: self.condition,
        }
    }
}

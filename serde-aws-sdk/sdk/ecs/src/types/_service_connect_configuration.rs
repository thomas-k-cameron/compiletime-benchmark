// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Service Connect configuration of your Amazon ECS service. The configuration for this service to discover and connect to services, and be discovered by, and connected from, other services within a namespace.</p>
/// <p>Tasks that run in a namespace can use short names to connect to services in the namespace. Tasks can connect to services across all of the clusters in the namespace. Tasks connect through a managed proxy container that collects logs and metrics for increased visibility. Only the tasks that Amazon ECS services create are supported with Service Connect. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
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
pub struct ServiceConnectConfiguration {
    /// <p>Specifies whether to use Service Connect with this service.</p>
    #[doc(hidden)]
    pub enabled: bool,
    /// <p>The namespace name or full Amazon Resource Name (ARN) of the Cloud Map namespace for use with Service Connect. The namespace must be in the same Amazon Web Services Region as the Amazon ECS service and cluster. The type of namespace doesn't affect Service Connect. For more information about Cloud Map, see <a href="https://docs.aws.amazon.com/">Working with Services</a> in the <i>Cloud Map Developer Guide</i>.</p>
    #[doc(hidden)]
    pub namespace: std::option::Option<std::string::String>,
    /// <p>The list of Service Connect service objects. These are names and aliases (also known as endpoints) that are used by other Amazon ECS services to connect to this service. </p>
    /// <p>This field is not required for a "client" Amazon ECS service that's a member of a namespace only to connect to other services within the namespace. An example of this would be a frontend application that accepts incoming requests from either a load balancer that's attached to the service or by other means.</p>
    /// <p>An object selects a port from the task definition, assigns a name for the Cloud Map service, and a list of aliases (endpoints) and ports for client applications to refer to this service.</p>
    #[doc(hidden)]
    pub services: std::option::Option<std::vec::Vec<crate::types::ServiceConnectService>>,
    /// <p>The log configuration for the container. This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p>
    /// <p>By default, containers use the same logging driver that the Docker daemon uses. However, the container might use a different logging driver than the Docker daemon by specifying a log driver configuration in the container definition. For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p>
    /// <p>Understand the following when specifying a log configuration for your containers.</p>
    /// <ul>
    /// <li> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the valid values below). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </li>
    /// <li> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance.</p> </li>
    /// <li> <p>For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register the available logging drivers with the <code>ECS_AVAILABLE_LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS container agent configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li>
    /// <li> <p>For tasks that are on Fargate, because you don't have access to the underlying infrastructure your tasks are hosted on, any additional software needed must be installed outside of the task. For example, the Fluentd output aggregators or a remote host running Logstash to send Gelf logs to.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub log_configuration: std::option::Option<crate::types::LogConfiguration>,
}
impl ServiceConnectConfiguration {
    /// <p>Specifies whether to use Service Connect with this service.</p>
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    /// <p>The namespace name or full Amazon Resource Name (ARN) of the Cloud Map namespace for use with Service Connect. The namespace must be in the same Amazon Web Services Region as the Amazon ECS service and cluster. The type of namespace doesn't affect Service Connect. For more information about Cloud Map, see <a href="https://docs.aws.amazon.com/">Working with Services</a> in the <i>Cloud Map Developer Guide</i>.</p>
    pub fn namespace(&self) -> std::option::Option<&str> {
        self.namespace.as_deref()
    }
    /// <p>The list of Service Connect service objects. These are names and aliases (also known as endpoints) that are used by other Amazon ECS services to connect to this service. </p>
    /// <p>This field is not required for a "client" Amazon ECS service that's a member of a namespace only to connect to other services within the namespace. An example of this would be a frontend application that accepts incoming requests from either a load balancer that's attached to the service or by other means.</p>
    /// <p>An object selects a port from the task definition, assigns a name for the Cloud Map service, and a list of aliases (endpoints) and ports for client applications to refer to this service.</p>
    pub fn services(&self) -> std::option::Option<&[crate::types::ServiceConnectService]> {
        self.services.as_deref()
    }
    /// <p>The log configuration for the container. This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p>
    /// <p>By default, containers use the same logging driver that the Docker daemon uses. However, the container might use a different logging driver than the Docker daemon by specifying a log driver configuration in the container definition. For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p>
    /// <p>Understand the following when specifying a log configuration for your containers.</p>
    /// <ul>
    /// <li> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the valid values below). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </li>
    /// <li> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance.</p> </li>
    /// <li> <p>For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register the available logging drivers with the <code>ECS_AVAILABLE_LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS container agent configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li>
    /// <li> <p>For tasks that are on Fargate, because you don't have access to the underlying infrastructure your tasks are hosted on, any additional software needed must be installed outside of the task. For example, the Fluentd output aggregators or a remote host running Logstash to send Gelf logs to.</p> </li>
    /// </ul>
    pub fn log_configuration(&self) -> std::option::Option<&crate::types::LogConfiguration> {
        self.log_configuration.as_ref()
    }
}
impl ServiceConnectConfiguration {
    /// Creates a new builder-style object to manufacture [`ServiceConnectConfiguration`](crate::types::ServiceConnectConfiguration).
    pub fn builder() -> crate::types::builders::ServiceConnectConfigurationBuilder {
        crate::types::builders::ServiceConnectConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ServiceConnectConfiguration;
/// A builder for [`ServiceConnectConfiguration`](crate::types::ServiceConnectConfiguration).
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
pub struct ServiceConnectConfigurationBuilder {
    pub(crate) enabled: std::option::Option<bool>,
    pub(crate) namespace: std::option::Option<std::string::String>,
    pub(crate) services: std::option::Option<std::vec::Vec<crate::types::ServiceConnectService>>,
    pub(crate) log_configuration: std::option::Option<crate::types::LogConfiguration>,
}
impl ServiceConnectConfigurationBuilder {
    /// <p>Specifies whether to use Service Connect with this service.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = Some(input);
        self
    }
    /// <p>Specifies whether to use Service Connect with this service.</p>
    pub fn set_enabled(mut self, input: std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>The namespace name or full Amazon Resource Name (ARN) of the Cloud Map namespace for use with Service Connect. The namespace must be in the same Amazon Web Services Region as the Amazon ECS service and cluster. The type of namespace doesn't affect Service Connect. For more information about Cloud Map, see <a href="https://docs.aws.amazon.com/">Working with Services</a> in the <i>Cloud Map Developer Guide</i>.</p>
    pub fn namespace(mut self, input: impl Into<std::string::String>) -> Self {
        self.namespace = Some(input.into());
        self
    }
    /// <p>The namespace name or full Amazon Resource Name (ARN) of the Cloud Map namespace for use with Service Connect. The namespace must be in the same Amazon Web Services Region as the Amazon ECS service and cluster. The type of namespace doesn't affect Service Connect. For more information about Cloud Map, see <a href="https://docs.aws.amazon.com/">Working with Services</a> in the <i>Cloud Map Developer Guide</i>.</p>
    pub fn set_namespace(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.namespace = input;
        self
    }
    /// Appends an item to `services`.
    ///
    /// To override the contents of this collection use [`set_services`](Self::set_services).
    ///
    /// <p>The list of Service Connect service objects. These are names and aliases (also known as endpoints) that are used by other Amazon ECS services to connect to this service. </p>
    /// <p>This field is not required for a "client" Amazon ECS service that's a member of a namespace only to connect to other services within the namespace. An example of this would be a frontend application that accepts incoming requests from either a load balancer that's attached to the service or by other means.</p>
    /// <p>An object selects a port from the task definition, assigns a name for the Cloud Map service, and a list of aliases (endpoints) and ports for client applications to refer to this service.</p>
    pub fn services(mut self, input: crate::types::ServiceConnectService) -> Self {
        let mut v = self.services.unwrap_or_default();
        v.push(input);
        self.services = Some(v);
        self
    }
    /// <p>The list of Service Connect service objects. These are names and aliases (also known as endpoints) that are used by other Amazon ECS services to connect to this service. </p>
    /// <p>This field is not required for a "client" Amazon ECS service that's a member of a namespace only to connect to other services within the namespace. An example of this would be a frontend application that accepts incoming requests from either a load balancer that's attached to the service or by other means.</p>
    /// <p>An object selects a port from the task definition, assigns a name for the Cloud Map service, and a list of aliases (endpoints) and ports for client applications to refer to this service.</p>
    pub fn set_services(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ServiceConnectService>>,
    ) -> Self {
        self.services = input;
        self
    }
    /// <p>The log configuration for the container. This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p>
    /// <p>By default, containers use the same logging driver that the Docker daemon uses. However, the container might use a different logging driver than the Docker daemon by specifying a log driver configuration in the container definition. For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p>
    /// <p>Understand the following when specifying a log configuration for your containers.</p>
    /// <ul>
    /// <li> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the valid values below). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </li>
    /// <li> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance.</p> </li>
    /// <li> <p>For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register the available logging drivers with the <code>ECS_AVAILABLE_LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS container agent configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li>
    /// <li> <p>For tasks that are on Fargate, because you don't have access to the underlying infrastructure your tasks are hosted on, any additional software needed must be installed outside of the task. For example, the Fluentd output aggregators or a remote host running Logstash to send Gelf logs to.</p> </li>
    /// </ul>
    pub fn log_configuration(mut self, input: crate::types::LogConfiguration) -> Self {
        self.log_configuration = Some(input);
        self
    }
    /// <p>The log configuration for the container. This parameter maps to <code>LogConfig</code> in the <a href="https://docs.docker.com/engine/api/v1.35/#operation/ContainerCreate">Create a container</a> section of the <a href="https://docs.docker.com/engine/api/v1.35/">Docker Remote API</a> and the <code>--log-driver</code> option to <a href="https://docs.docker.com/engine/reference/commandline/run/"> <code>docker run</code> </a>.</p>
    /// <p>By default, containers use the same logging driver that the Docker daemon uses. However, the container might use a different logging driver than the Docker daemon by specifying a log driver configuration in the container definition. For more information about the options for different supported log drivers, see <a href="https://docs.docker.com/engine/admin/logging/overview/">Configure logging drivers</a> in the Docker documentation.</p>
    /// <p>Understand the following when specifying a log configuration for your containers.</p>
    /// <ul>
    /// <li> <p>Amazon ECS currently supports a subset of the logging drivers available to the Docker daemon (shown in the valid values below). Additional log drivers may be available in future releases of the Amazon ECS container agent.</p> </li>
    /// <li> <p>This parameter requires version 1.18 of the Docker Remote API or greater on your container instance.</p> </li>
    /// <li> <p>For tasks that are hosted on Amazon EC2 instances, the Amazon ECS container agent must register the available logging drivers with the <code>ECS_AVAILABLE_LOGGING_DRIVERS</code> environment variable before containers placed on that instance can use these log configuration options. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-agent-config.html">Amazon ECS container agent configuration</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p> </li>
    /// <li> <p>For tasks that are on Fargate, because you don't have access to the underlying infrastructure your tasks are hosted on, any additional software needed must be installed outside of the task. For example, the Fluentd output aggregators or a remote host running Logstash to send Gelf logs to.</p> </li>
    /// </ul>
    pub fn set_log_configuration(
        mut self,
        input: std::option::Option<crate::types::LogConfiguration>,
    ) -> Self {
        self.log_configuration = input;
        self
    }
    /// Consumes the builder and constructs a [`ServiceConnectConfiguration`](crate::types::ServiceConnectConfiguration).
    pub fn build(self) -> crate::types::ServiceConnectConfiguration {
        crate::types::ServiceConnectConfiguration {
            enabled: self.enabled.unwrap_or_default(),
            namespace: self.namespace,
            services: self.services,
            log_configuration: self.log_configuration,
        }
    }
}

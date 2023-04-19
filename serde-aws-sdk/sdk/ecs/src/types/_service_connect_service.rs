// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Service Connect service object configuration. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/service-connect.html">Service Connect</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
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
pub struct ServiceConnectService {
    /// <p>The <code>portName</code> must match the name of one of the <code>portMappings</code> from all the containers in the task definition of this Amazon ECS service.</p>
    #[doc(hidden)]
    pub port_name: std::option::Option<std::string::String>,
    /// <p>The <code>discoveryName</code> is the name of the new Cloud Map service that Amazon ECS creates for this Amazon ECS service. This must be unique within the Cloud Map namespace. The name can contain up to 64 characters. The name can include lowercase letters, numbers, underscores (_), and hyphens (-). The name can't start with a hyphen.</p>
    /// <p>If this parameter isn't specified, the default value of <code>discoveryName.namespace</code> is used. If the <code>discoveryName</code> isn't specified, the port mapping name from the task definition is used in <code>portName.namespace</code>.</p>
    #[doc(hidden)]
    pub discovery_name: std::option::Option<std::string::String>,
    /// <p>The list of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1.</p>
    /// <p>Each alias ("endpoint") is a fully-qualified name and port number that other Amazon ECS tasks ("clients") can use to connect to this service.</p>
    /// <p>Each name and port mapping must be unique within the namespace.</p>
    /// <p>For each <code>ServiceConnectService</code>, you must provide at least one <code>clientAlias</code> with one <code>port</code>.</p>
    #[doc(hidden)]
    pub client_aliases: std::option::Option<std::vec::Vec<crate::types::ServiceConnectClientAlias>>,
    /// <p>The port number for the Service Connect proxy to listen on.</p>
    /// <p>Use the value of this field to bypass the proxy for traffic on the port number specified in the named <code>portMapping</code> in the task definition of this application, and then use it in your VPC security groups to allow traffic into the proxy for this Amazon ECS service.</p>
    /// <p>In <code>awsvpc</code> mode and Fargate, the default value is the container port number. The container port number is in the <code>portMapping</code> in the task definition. In bridge mode, the default value is the ephemeral port of the Service Connect proxy.</p>
    #[doc(hidden)]
    pub ingress_port_override: std::option::Option<i32>,
}
impl ServiceConnectService {
    /// <p>The <code>portName</code> must match the name of one of the <code>portMappings</code> from all the containers in the task definition of this Amazon ECS service.</p>
    pub fn port_name(&self) -> std::option::Option<&str> {
        self.port_name.as_deref()
    }
    /// <p>The <code>discoveryName</code> is the name of the new Cloud Map service that Amazon ECS creates for this Amazon ECS service. This must be unique within the Cloud Map namespace. The name can contain up to 64 characters. The name can include lowercase letters, numbers, underscores (_), and hyphens (-). The name can't start with a hyphen.</p>
    /// <p>If this parameter isn't specified, the default value of <code>discoveryName.namespace</code> is used. If the <code>discoveryName</code> isn't specified, the port mapping name from the task definition is used in <code>portName.namespace</code>.</p>
    pub fn discovery_name(&self) -> std::option::Option<&str> {
        self.discovery_name.as_deref()
    }
    /// <p>The list of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1.</p>
    /// <p>Each alias ("endpoint") is a fully-qualified name and port number that other Amazon ECS tasks ("clients") can use to connect to this service.</p>
    /// <p>Each name and port mapping must be unique within the namespace.</p>
    /// <p>For each <code>ServiceConnectService</code>, you must provide at least one <code>clientAlias</code> with one <code>port</code>.</p>
    pub fn client_aliases(
        &self,
    ) -> std::option::Option<&[crate::types::ServiceConnectClientAlias]> {
        self.client_aliases.as_deref()
    }
    /// <p>The port number for the Service Connect proxy to listen on.</p>
    /// <p>Use the value of this field to bypass the proxy for traffic on the port number specified in the named <code>portMapping</code> in the task definition of this application, and then use it in your VPC security groups to allow traffic into the proxy for this Amazon ECS service.</p>
    /// <p>In <code>awsvpc</code> mode and Fargate, the default value is the container port number. The container port number is in the <code>portMapping</code> in the task definition. In bridge mode, the default value is the ephemeral port of the Service Connect proxy.</p>
    pub fn ingress_port_override(&self) -> std::option::Option<i32> {
        self.ingress_port_override
    }
}
impl ServiceConnectService {
    /// Creates a new builder-style object to manufacture [`ServiceConnectService`](crate::types::ServiceConnectService).
    pub fn builder() -> crate::types::builders::ServiceConnectServiceBuilder {
        crate::types::builders::ServiceConnectServiceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ServiceConnectService;
/// A builder for [`ServiceConnectService`](crate::types::ServiceConnectService).
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
pub struct ServiceConnectServiceBuilder {
    pub(crate) port_name: std::option::Option<std::string::String>,
    pub(crate) discovery_name: std::option::Option<std::string::String>,
    pub(crate) client_aliases:
        std::option::Option<std::vec::Vec<crate::types::ServiceConnectClientAlias>>,
    pub(crate) ingress_port_override: std::option::Option<i32>,
}
impl ServiceConnectServiceBuilder {
    /// <p>The <code>portName</code> must match the name of one of the <code>portMappings</code> from all the containers in the task definition of this Amazon ECS service.</p>
    pub fn port_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.port_name = Some(input.into());
        self
    }
    /// <p>The <code>portName</code> must match the name of one of the <code>portMappings</code> from all the containers in the task definition of this Amazon ECS service.</p>
    pub fn set_port_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.port_name = input;
        self
    }
    /// <p>The <code>discoveryName</code> is the name of the new Cloud Map service that Amazon ECS creates for this Amazon ECS service. This must be unique within the Cloud Map namespace. The name can contain up to 64 characters. The name can include lowercase letters, numbers, underscores (_), and hyphens (-). The name can't start with a hyphen.</p>
    /// <p>If this parameter isn't specified, the default value of <code>discoveryName.namespace</code> is used. If the <code>discoveryName</code> isn't specified, the port mapping name from the task definition is used in <code>portName.namespace</code>.</p>
    pub fn discovery_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.discovery_name = Some(input.into());
        self
    }
    /// <p>The <code>discoveryName</code> is the name of the new Cloud Map service that Amazon ECS creates for this Amazon ECS service. This must be unique within the Cloud Map namespace. The name can contain up to 64 characters. The name can include lowercase letters, numbers, underscores (_), and hyphens (-). The name can't start with a hyphen.</p>
    /// <p>If this parameter isn't specified, the default value of <code>discoveryName.namespace</code> is used. If the <code>discoveryName</code> isn't specified, the port mapping name from the task definition is used in <code>portName.namespace</code>.</p>
    pub fn set_discovery_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.discovery_name = input;
        self
    }
    /// Appends an item to `client_aliases`.
    ///
    /// To override the contents of this collection use [`set_client_aliases`](Self::set_client_aliases).
    ///
    /// <p>The list of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1.</p>
    /// <p>Each alias ("endpoint") is a fully-qualified name and port number that other Amazon ECS tasks ("clients") can use to connect to this service.</p>
    /// <p>Each name and port mapping must be unique within the namespace.</p>
    /// <p>For each <code>ServiceConnectService</code>, you must provide at least one <code>clientAlias</code> with one <code>port</code>.</p>
    pub fn client_aliases(mut self, input: crate::types::ServiceConnectClientAlias) -> Self {
        let mut v = self.client_aliases.unwrap_or_default();
        v.push(input);
        self.client_aliases = Some(v);
        self
    }
    /// <p>The list of client aliases for this Service Connect service. You use these to assign names that can be used by client applications. The maximum number of client aliases that you can have in this list is 1.</p>
    /// <p>Each alias ("endpoint") is a fully-qualified name and port number that other Amazon ECS tasks ("clients") can use to connect to this service.</p>
    /// <p>Each name and port mapping must be unique within the namespace.</p>
    /// <p>For each <code>ServiceConnectService</code>, you must provide at least one <code>clientAlias</code> with one <code>port</code>.</p>
    pub fn set_client_aliases(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ServiceConnectClientAlias>>,
    ) -> Self {
        self.client_aliases = input;
        self
    }
    /// <p>The port number for the Service Connect proxy to listen on.</p>
    /// <p>Use the value of this field to bypass the proxy for traffic on the port number specified in the named <code>portMapping</code> in the task definition of this application, and then use it in your VPC security groups to allow traffic into the proxy for this Amazon ECS service.</p>
    /// <p>In <code>awsvpc</code> mode and Fargate, the default value is the container port number. The container port number is in the <code>portMapping</code> in the task definition. In bridge mode, the default value is the ephemeral port of the Service Connect proxy.</p>
    pub fn ingress_port_override(mut self, input: i32) -> Self {
        self.ingress_port_override = Some(input);
        self
    }
    /// <p>The port number for the Service Connect proxy to listen on.</p>
    /// <p>Use the value of this field to bypass the proxy for traffic on the port number specified in the named <code>portMapping</code> in the task definition of this application, and then use it in your VPC security groups to allow traffic into the proxy for this Amazon ECS service.</p>
    /// <p>In <code>awsvpc</code> mode and Fargate, the default value is the container port number. The container port number is in the <code>portMapping</code> in the task definition. In bridge mode, the default value is the ephemeral port of the Service Connect proxy.</p>
    pub fn set_ingress_port_override(mut self, input: std::option::Option<i32>) -> Self {
        self.ingress_port_override = input;
        self
    }
    /// Consumes the builder and constructs a [`ServiceConnectService`](crate::types::ServiceConnectService).
    pub fn build(self) -> crate::types::ServiceConnectService {
        crate::types::ServiceConnectService {
            port_name: self.port_name,
            discovery_name: self.discovery_name,
            client_aliases: self.client_aliases,
            ingress_port_override: self.ingress_port_override,
        }
    }
}

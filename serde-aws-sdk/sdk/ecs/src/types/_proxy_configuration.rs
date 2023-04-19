// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration details for the App Mesh proxy.</p>
/// <p>For tasks that use the EC2 launch type, the container instances require at least version 1.26.0 of the container agent and at least version 1.26.0-1 of the <code>ecs-init</code> package to use a proxy configuration. If your container instances are launched from the Amazon ECS optimized AMI version <code>20190301</code> or later, then they contain the required versions of the container agent and <code>ecs-init</code>. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/ecs-optimized_AMI.html">Amazon ECS-optimized Linux AMI</a> </p>
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
pub struct ProxyConfiguration {
    /// <p>The proxy type. The only supported value is <code>APPMESH</code>.</p>
    #[doc(hidden)]
    pub r#type: std::option::Option<crate::types::ProxyConfigurationType>,
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    #[doc(hidden)]
    pub container_name: std::option::Option<std::string::String>,
    /// <p>The set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified as key-value pairs.</p>
    /// <ul>
    /// <li> <p> <code>IgnoredUID</code> - (Required) The user ID (UID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredGID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>IgnoredGID</code> - (Required) The group ID (GID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredUID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>AppPorts</code> - (Required) The list of ports that the application uses. Network traffic to these ports is forwarded to the <code>ProxyIngressPort</code> and <code>ProxyEgressPort</code>.</p> </li>
    /// <li> <p> <code>ProxyIngressPort</code> - (Required) Specifies the port that incoming traffic to the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>ProxyEgressPort</code> - (Required) Specifies the port that outgoing traffic from the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>EgressIgnoredPorts</code> - (Required) The egress traffic going to the specified ports is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// <li> <p> <code>EgressIgnoredIPs</code> - (Required) The egress traffic going to the specified IP addresses is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub properties: std::option::Option<std::vec::Vec<crate::types::KeyValuePair>>,
}
impl ProxyConfiguration {
    /// <p>The proxy type. The only supported value is <code>APPMESH</code>.</p>
    pub fn r#type(&self) -> std::option::Option<&crate::types::ProxyConfigurationType> {
        self.r#type.as_ref()
    }
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    pub fn container_name(&self) -> std::option::Option<&str> {
        self.container_name.as_deref()
    }
    /// <p>The set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified as key-value pairs.</p>
    /// <ul>
    /// <li> <p> <code>IgnoredUID</code> - (Required) The user ID (UID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredGID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>IgnoredGID</code> - (Required) The group ID (GID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredUID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>AppPorts</code> - (Required) The list of ports that the application uses. Network traffic to these ports is forwarded to the <code>ProxyIngressPort</code> and <code>ProxyEgressPort</code>.</p> </li>
    /// <li> <p> <code>ProxyIngressPort</code> - (Required) Specifies the port that incoming traffic to the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>ProxyEgressPort</code> - (Required) Specifies the port that outgoing traffic from the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>EgressIgnoredPorts</code> - (Required) The egress traffic going to the specified ports is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// <li> <p> <code>EgressIgnoredIPs</code> - (Required) The egress traffic going to the specified IP addresses is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// </ul>
    pub fn properties(&self) -> std::option::Option<&[crate::types::KeyValuePair]> {
        self.properties.as_deref()
    }
}
impl ProxyConfiguration {
    /// Creates a new builder-style object to manufacture [`ProxyConfiguration`](crate::types::ProxyConfiguration).
    pub fn builder() -> crate::types::builders::ProxyConfigurationBuilder {
        crate::types::builders::ProxyConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::ProxyConfiguration;
/// A builder for [`ProxyConfiguration`](crate::types::ProxyConfiguration).
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
pub struct ProxyConfigurationBuilder {
    pub(crate) r#type: std::option::Option<crate::types::ProxyConfigurationType>,
    pub(crate) container_name: std::option::Option<std::string::String>,
    pub(crate) properties: std::option::Option<std::vec::Vec<crate::types::KeyValuePair>>,
}
impl ProxyConfigurationBuilder {
    /// <p>The proxy type. The only supported value is <code>APPMESH</code>.</p>
    pub fn r#type(mut self, input: crate::types::ProxyConfigurationType) -> Self {
        self.r#type = Some(input);
        self
    }
    /// <p>The proxy type. The only supported value is <code>APPMESH</code>.</p>
    pub fn set_type(
        mut self,
        input: std::option::Option<crate::types::ProxyConfigurationType>,
    ) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    pub fn container_name(mut self, input: impl Into<std::string::String>) -> Self {
        self.container_name = Some(input.into());
        self
    }
    /// <p>The name of the container that will serve as the App Mesh proxy.</p>
    pub fn set_container_name(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.container_name = input;
        self
    }
    /// Appends an item to `properties`.
    ///
    /// To override the contents of this collection use [`set_properties`](Self::set_properties).
    ///
    /// <p>The set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified as key-value pairs.</p>
    /// <ul>
    /// <li> <p> <code>IgnoredUID</code> - (Required) The user ID (UID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredGID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>IgnoredGID</code> - (Required) The group ID (GID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredUID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>AppPorts</code> - (Required) The list of ports that the application uses. Network traffic to these ports is forwarded to the <code>ProxyIngressPort</code> and <code>ProxyEgressPort</code>.</p> </li>
    /// <li> <p> <code>ProxyIngressPort</code> - (Required) Specifies the port that incoming traffic to the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>ProxyEgressPort</code> - (Required) Specifies the port that outgoing traffic from the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>EgressIgnoredPorts</code> - (Required) The egress traffic going to the specified ports is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// <li> <p> <code>EgressIgnoredIPs</code> - (Required) The egress traffic going to the specified IP addresses is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// </ul>
    pub fn properties(mut self, input: crate::types::KeyValuePair) -> Self {
        let mut v = self.properties.unwrap_or_default();
        v.push(input);
        self.properties = Some(v);
        self
    }
    /// <p>The set of network configuration parameters to provide the Container Network Interface (CNI) plugin, specified as key-value pairs.</p>
    /// <ul>
    /// <li> <p> <code>IgnoredUID</code> - (Required) The user ID (UID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredGID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>IgnoredGID</code> - (Required) The group ID (GID) of the proxy container as defined by the <code>user</code> parameter in a container definition. This is used to ensure the proxy ignores its own traffic. If <code>IgnoredUID</code> is specified, this field can be empty.</p> </li>
    /// <li> <p> <code>AppPorts</code> - (Required) The list of ports that the application uses. Network traffic to these ports is forwarded to the <code>ProxyIngressPort</code> and <code>ProxyEgressPort</code>.</p> </li>
    /// <li> <p> <code>ProxyIngressPort</code> - (Required) Specifies the port that incoming traffic to the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>ProxyEgressPort</code> - (Required) Specifies the port that outgoing traffic from the <code>AppPorts</code> is directed to.</p> </li>
    /// <li> <p> <code>EgressIgnoredPorts</code> - (Required) The egress traffic going to the specified ports is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// <li> <p> <code>EgressIgnoredIPs</code> - (Required) The egress traffic going to the specified IP addresses is ignored and not redirected to the <code>ProxyEgressPort</code>. It can be an empty list.</p> </li>
    /// </ul>
    pub fn set_properties(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::KeyValuePair>>,
    ) -> Self {
        self.properties = input;
        self
    }
    /// Consumes the builder and constructs a [`ProxyConfiguration`](crate::types::ProxyConfiguration).
    pub fn build(self) -> crate::types::ProxyConfiguration {
        crate::types::ProxyConfiguration {
            r#type: self.r#type,
            container_name: self.container_name,
            properties: self.properties,
        }
    }
}
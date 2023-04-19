// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>This parameter is specified when you're using an Amazon Elastic File System file system for task storage. For more information, see <a href="https://docs.aws.amazon.com/AmazonECS/latest/developerguide/efs-volumes.html">Amazon EFS volumes</a> in the <i>Amazon Elastic Container Service Developer Guide</i>.</p>
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
pub struct EfsVolumeConfiguration {
    /// <p>The Amazon EFS file system ID to use.</p>
    #[doc(hidden)]
    pub file_system_id: std::option::Option<std::string::String>,
    /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying <code>/</code> will have the same effect as omitting this parameter.</p> <important>
    /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point.</p>
    /// </important>
    #[doc(hidden)]
    pub root_directory: std::option::Option<std::string::String>,
    /// <p>Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/encryption-in-transit.html">Encrypting data in transit</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    #[doc(hidden)]
    pub transit_encryption: std::option::Option<crate::types::EfsTransitEncryption>,
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-mount-helper.html">EFS mount helper</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    #[doc(hidden)]
    pub transit_encryption_port: std::option::Option<i32>,
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    #[doc(hidden)]
    pub authorization_config: std::option::Option<crate::types::EfsAuthorizationConfig>,
}
impl EfsVolumeConfiguration {
    /// <p>The Amazon EFS file system ID to use.</p>
    pub fn file_system_id(&self) -> std::option::Option<&str> {
        self.file_system_id.as_deref()
    }
    /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying <code>/</code> will have the same effect as omitting this parameter.</p> <important>
    /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point.</p>
    /// </important>
    pub fn root_directory(&self) -> std::option::Option<&str> {
        self.root_directory.as_deref()
    }
    /// <p>Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/encryption-in-transit.html">Encrypting data in transit</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn transit_encryption(&self) -> std::option::Option<&crate::types::EfsTransitEncryption> {
        self.transit_encryption.as_ref()
    }
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-mount-helper.html">EFS mount helper</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn transit_encryption_port(&self) -> std::option::Option<i32> {
        self.transit_encryption_port
    }
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    pub fn authorization_config(
        &self,
    ) -> std::option::Option<&crate::types::EfsAuthorizationConfig> {
        self.authorization_config.as_ref()
    }
}
impl EfsVolumeConfiguration {
    /// Creates a new builder-style object to manufacture [`EfsVolumeConfiguration`](crate::types::EfsVolumeConfiguration).
    pub fn builder() -> crate::types::builders::EfsVolumeConfigurationBuilder {
        crate::types::builders::EfsVolumeConfigurationBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EfsVolumeConfiguration;
/// A builder for [`EfsVolumeConfiguration`](crate::types::EfsVolumeConfiguration).
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
pub struct EfsVolumeConfigurationBuilder {
    pub(crate) file_system_id: std::option::Option<std::string::String>,
    pub(crate) root_directory: std::option::Option<std::string::String>,
    pub(crate) transit_encryption: std::option::Option<crate::types::EfsTransitEncryption>,
    pub(crate) transit_encryption_port: std::option::Option<i32>,
    pub(crate) authorization_config: std::option::Option<crate::types::EfsAuthorizationConfig>,
}
impl EfsVolumeConfigurationBuilder {
    /// <p>The Amazon EFS file system ID to use.</p>
    pub fn file_system_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.file_system_id = Some(input.into());
        self
    }
    /// <p>The Amazon EFS file system ID to use.</p>
    pub fn set_file_system_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.file_system_id = input;
        self
    }
    /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying <code>/</code> will have the same effect as omitting this parameter.</p> <important>
    /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point.</p>
    /// </important>
    pub fn root_directory(mut self, input: impl Into<std::string::String>) -> Self {
        self.root_directory = Some(input.into());
        self
    }
    /// <p>The directory within the Amazon EFS file system to mount as the root directory inside the host. If this parameter is omitted, the root of the Amazon EFS volume will be used. Specifying <code>/</code> will have the same effect as omitting this parameter.</p> <important>
    /// <p>If an EFS access point is specified in the <code>authorizationConfig</code>, the root directory parameter must either be omitted or set to <code>/</code> which will enforce the path set on the EFS access point.</p>
    /// </important>
    pub fn set_root_directory(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.root_directory = input;
        self
    }
    /// <p>Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/encryption-in-transit.html">Encrypting data in transit</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn transit_encryption(mut self, input: crate::types::EfsTransitEncryption) -> Self {
        self.transit_encryption = Some(input);
        self
    }
    /// <p>Determines whether to use encryption for Amazon EFS data in transit between the Amazon ECS host and the Amazon EFS server. Transit encryption must be enabled if Amazon EFS IAM authorization is used. If this parameter is omitted, the default value of <code>DISABLED</code> is used. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/encryption-in-transit.html">Encrypting data in transit</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn set_transit_encryption(
        mut self,
        input: std::option::Option<crate::types::EfsTransitEncryption>,
    ) -> Self {
        self.transit_encryption = input;
        self
    }
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-mount-helper.html">EFS mount helper</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn transit_encryption_port(mut self, input: i32) -> Self {
        self.transit_encryption_port = Some(input);
        self
    }
    /// <p>The port to use when sending encrypted data between the Amazon ECS host and the Amazon EFS server. If you do not specify a transit encryption port, it will use the port selection strategy that the Amazon EFS mount helper uses. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/efs-mount-helper.html">EFS mount helper</a> in the <i>Amazon Elastic File System User Guide</i>.</p>
    pub fn set_transit_encryption_port(mut self, input: std::option::Option<i32>) -> Self {
        self.transit_encryption_port = input;
        self
    }
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    pub fn authorization_config(mut self, input: crate::types::EfsAuthorizationConfig) -> Self {
        self.authorization_config = Some(input);
        self
    }
    /// <p>The authorization configuration details for the Amazon EFS file system.</p>
    pub fn set_authorization_config(
        mut self,
        input: std::option::Option<crate::types::EfsAuthorizationConfig>,
    ) -> Self {
        self.authorization_config = input;
        self
    }
    /// Consumes the builder and constructs a [`EfsVolumeConfiguration`](crate::types::EfsVolumeConfiguration).
    pub fn build(self) -> crate::types::EfsVolumeConfiguration {
        crate::types::EfsVolumeConfiguration {
            file_system_id: self.file_system_id,
            root_directory: self.root_directory,
            transit_encryption: self.transit_encryption,
            transit_encryption_port: self.transit_encryption_port,
            authorization_config: self.authorization_config,
        }
    }
}
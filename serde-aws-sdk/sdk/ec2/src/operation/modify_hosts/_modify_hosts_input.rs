// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
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
pub struct ModifyHostsInput {
    /// <p>Specify whether to enable or disable auto-placement.</p>
    #[doc(hidden)]
    pub auto_placement: std::option::Option<crate::types::AutoPlacement>,
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    #[doc(hidden)]
    pub host_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub host_recovery: std::option::Option<crate::types::HostRecovery>,
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    #[doc(hidden)]
    pub instance_family: std::option::Option<std::string::String>,
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub host_maintenance: std::option::Option<crate::types::HostMaintenance>,
}
impl ModifyHostsInput {
    /// <p>Specify whether to enable or disable auto-placement.</p>
    pub fn auto_placement(&self) -> std::option::Option<&crate::types::AutoPlacement> {
        self.auto_placement.as_ref()
    }
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    pub fn host_ids(&self) -> std::option::Option<&[std::string::String]> {
        self.host_ids.as_deref()
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_recovery(&self) -> std::option::Option<&crate::types::HostRecovery> {
        self.host_recovery.as_ref()
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(&self) -> std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(&self) -> std::option::Option<&str> {
        self.instance_family.as_deref()
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(&self) -> std::option::Option<&crate::types::HostMaintenance> {
        self.host_maintenance.as_ref()
    }
}
impl ModifyHostsInput {
    /// Creates a new builder-style object to manufacture [`ModifyHostsInput`](crate::operation::modify_hosts::ModifyHostsInput).
    pub fn builder() -> crate::operation::modify_hosts::builders::ModifyHostsInputBuilder {
        crate::operation::modify_hosts::builders::ModifyHostsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::modify_hosts::ModifyHostsInput;
/// A builder for [`ModifyHostsInput`](crate::operation::modify_hosts::ModifyHostsInput).
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
pub struct ModifyHostsInputBuilder {
    pub(crate) auto_placement: std::option::Option<crate::types::AutoPlacement>,
    pub(crate) host_ids: std::option::Option<std::vec::Vec<std::string::String>>,
    pub(crate) host_recovery: std::option::Option<crate::types::HostRecovery>,
    pub(crate) instance_type: std::option::Option<std::string::String>,
    pub(crate) instance_family: std::option::Option<std::string::String>,
    pub(crate) host_maintenance: std::option::Option<crate::types::HostMaintenance>,
}
impl ModifyHostsInputBuilder {
    /// <p>Specify whether to enable or disable auto-placement.</p>
    pub fn auto_placement(mut self, input: crate::types::AutoPlacement) -> Self {
        self.auto_placement = Some(input);
        self
    }
    /// <p>Specify whether to enable or disable auto-placement.</p>
    pub fn set_auto_placement(
        mut self,
        input: std::option::Option<crate::types::AutoPlacement>,
    ) -> Self {
        self.auto_placement = input;
        self
    }
    /// Appends an item to `host_ids`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    pub fn host_ids(mut self, input: impl Into<std::string::String>) -> Self {
        let mut v = self.host_ids.unwrap_or_default();
        v.push(input.into());
        self.host_ids = Some(v);
        self
    }
    /// <p>The IDs of the Dedicated Hosts to modify.</p>
    pub fn set_host_ids(
        mut self,
        input: std::option::Option<std::vec::Vec<std::string::String>>,
    ) -> Self {
        self.host_ids = input;
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_recovery(mut self, input: crate::types::HostRecovery) -> Self {
        self.host_recovery = Some(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_recovery(
        mut self,
        input: std::option::Option<crate::types::HostRecovery>,
    ) -> Self {
        self.host_recovery = input;
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_type = Some(input.into());
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support only a specific instance type.</p>
    /// <p>If you want to modify a Dedicated Host to support multiple instance types in its current instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_family = Some(input.into());
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Host. Specify this parameter to modify a Dedicated Host to support multiple instance types within its current instance family.</p>
    /// <p>If you want to modify a Dedicated Host to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn set_instance_family(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_family = input;
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(mut self, input: crate::types::HostMaintenance) -> Self {
        self.host_maintenance = Some(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html"> Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_maintenance(
        mut self,
        input: std::option::Option<crate::types::HostMaintenance>,
    ) -> Self {
        self.host_maintenance = input;
        self
    }
    /// Consumes the builder and constructs a [`ModifyHostsInput`](crate::operation::modify_hosts::ModifyHostsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::modify_hosts::ModifyHostsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::modify_hosts::ModifyHostsInput {
            auto_placement: self.auto_placement,
            host_ids: self.host_ids,
            host_recovery: self.host_recovery,
            instance_type: self.instance_type,
            instance_family: self.instance_family,
            host_maintenance: self.host_maintenance,
        })
    }
}

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
pub struct AllocateHostsInput {
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>on</code> </p>
    #[doc(hidden)]
    pub auto_placement: std::option::Option<crate::types::AutoPlacement>,
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    #[doc(hidden)]
    pub availability_zone: std::option::Option<std::string::String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    #[doc(hidden)]
    pub client_token: std::option::Option<std::string::String>,
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<std::string::String>,
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    #[doc(hidden)]
    pub instance_family: std::option::Option<std::string::String>,
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters.</p>
    #[doc(hidden)]
    pub quantity: std::option::Option<i32>,
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    #[doc(hidden)]
    pub tag_specifications: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code> </p>
    #[doc(hidden)]
    pub host_recovery: std::option::Option<crate::types::HostRecovery>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host.</p>
    #[doc(hidden)]
    pub outpost_arn: std::option::Option<std::string::String>,
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub host_maintenance: std::option::Option<crate::types::HostMaintenance>,
}
impl AllocateHostsInput {
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>on</code> </p>
    pub fn auto_placement(&self) -> std::option::Option<&crate::types::AutoPlacement> {
        self.auto_placement.as_ref()
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn availability_zone(&self) -> std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(&self) -> std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(&self) -> std::option::Option<&str> {
        self.instance_family.as_deref()
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters.</p>
    pub fn quantity(&self) -> std::option::Option<i32> {
        self.quantity
    }
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn tag_specifications(&self) -> std::option::Option<&[crate::types::TagSpecification]> {
        self.tag_specifications.as_deref()
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code> </p>
    pub fn host_recovery(&self) -> std::option::Option<&crate::types::HostRecovery> {
        self.host_recovery.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host.</p>
    pub fn outpost_arn(&self) -> std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(&self) -> std::option::Option<&crate::types::HostMaintenance> {
        self.host_maintenance.as_ref()
    }
}
impl AllocateHostsInput {
    /// Creates a new builder-style object to manufacture [`AllocateHostsInput`](crate::operation::allocate_hosts::AllocateHostsInput).
    pub fn builder() -> crate::operation::allocate_hosts::builders::AllocateHostsInputBuilder {
        crate::operation::allocate_hosts::builders::AllocateHostsInputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::operation::allocate_hosts::AllocateHostsInput;
/// A builder for [`AllocateHostsInput`](crate::operation::allocate_hosts::AllocateHostsInput).
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
pub struct AllocateHostsInputBuilder {
    pub(crate) auto_placement: std::option::Option<crate::types::AutoPlacement>,
    pub(crate) availability_zone: std::option::Option<std::string::String>,
    pub(crate) client_token: std::option::Option<std::string::String>,
    pub(crate) instance_type: std::option::Option<std::string::String>,
    pub(crate) instance_family: std::option::Option<std::string::String>,
    pub(crate) quantity: std::option::Option<i32>,
    pub(crate) tag_specifications:
        std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) host_recovery: std::option::Option<crate::types::HostRecovery>,
    pub(crate) outpost_arn: std::option::Option<std::string::String>,
    pub(crate) host_maintenance: std::option::Option<crate::types::HostMaintenance>,
}
impl AllocateHostsInputBuilder {
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>on</code> </p>
    pub fn auto_placement(mut self, input: crate::types::AutoPlacement) -> Self {
        self.auto_placement = Some(input);
        self
    }
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>on</code> </p>
    pub fn set_auto_placement(
        mut self,
        input: std::option::Option<crate::types::AutoPlacement>,
    ) -> Self {
        self.auto_placement = input;
        self
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn availability_zone(mut self, input: impl Into<std::string::String>) -> Self {
        self.availability_zone = Some(input.into());
        self
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn set_availability_zone(
        mut self,
        input: std::option::Option<std::string::String>,
    ) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl Into<std::string::String>) -> Self {
        self.client_token = Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_type = Some(input.into());
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn set_instance_type(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_family = Some(input.into());
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn set_instance_family(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_family = input;
        self
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.quantity = Some(input);
        self
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters.</p>
    pub fn set_quantity(mut self, input: std::option::Option<i32>) -> Self {
        self.quantity = input;
        self
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = Some(v);
        self
    }
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn set_tag_specifications(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::TagSpecification>>,
    ) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code> </p>
    pub fn host_recovery(mut self, input: crate::types::HostRecovery) -> Self {
        self.host_recovery = Some(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code> </p>
    pub fn set_host_recovery(
        mut self,
        input: std::option::Option<crate::types::HostRecovery>,
    ) -> Self {
        self.host_recovery = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host.</p>
    pub fn outpost_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.outpost_arn = Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host.</p>
    pub fn set_outpost_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(mut self, input: crate::types::HostMaintenance) -> Self {
        self.host_maintenance = Some(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_maintenance(
        mut self,
        input: std::option::Option<crate::types::HostMaintenance>,
    ) -> Self {
        self.host_maintenance = input;
        self
    }
    /// Consumes the builder and constructs a [`AllocateHostsInput`](crate::operation::allocate_hosts::AllocateHostsInput).
    pub fn build(
        self,
    ) -> Result<
        crate::operation::allocate_hosts::AllocateHostsInput,
        aws_smithy_http::operation::error::BuildError,
    > {
        Ok(crate::operation::allocate_hosts::AllocateHostsInput {
            auto_placement: self.auto_placement,
            availability_zone: self.availability_zone,
            client_token: self.client_token,
            instance_type: self.instance_type,
            instance_family: self.instance_family,
            quantity: self.quantity,
            tag_specifications: self.tag_specifications,
            host_recovery: self.host_recovery,
            outpost_arn: self.outpost_arn,
            host_maintenance: self.host_maintenance,
        })
    }
}
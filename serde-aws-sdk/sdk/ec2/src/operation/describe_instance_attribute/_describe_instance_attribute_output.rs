// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an instance attribute.</p>
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
pub struct DescribeInstanceAttributeOutput {
    /// <p>The security groups associated with the instance.</p>
    #[doc(hidden)]
    pub groups: std::option::Option<std::vec::Vec<crate::types::GroupIdentifier>>,
    /// <p>The block device mapping of the instance.</p>
    #[doc(hidden)]
    pub block_device_mappings:
        std::option::Option<std::vec::Vec<crate::types::InstanceBlockDeviceMapping>>,
    /// <p>If the value is <code>true</code>, you can't terminate the instance through the Amazon EC2 console, CLI, or API; otherwise, you can.</p>
    #[doc(hidden)]
    pub disable_api_termination: std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>Indicates whether enhanced networking with ENA is enabled.</p>
    #[doc(hidden)]
    pub ena_support: std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    #[doc(hidden)]
    pub enclave_options: std::option::Option<crate::types::EnclaveOptions>,
    /// <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p>
    #[doc(hidden)]
    pub ebs_optimized: std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>The ID of the instance.</p>
    #[doc(hidden)]
    pub instance_id: std::option::Option<std::string::String>,
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    #[doc(hidden)]
    pub instance_initiated_shutdown_behavior: std::option::Option<crate::types::AttributeValue>,
    /// <p>The instance type.</p>
    #[doc(hidden)]
    pub instance_type: std::option::Option<crate::types::AttributeValue>,
    /// <p>The kernel ID.</p>
    #[doc(hidden)]
    pub kernel_id: std::option::Option<crate::types::AttributeValue>,
    /// <p>A list of product codes.</p>
    #[doc(hidden)]
    pub product_codes: std::option::Option<std::vec::Vec<crate::types::ProductCode>>,
    /// <p>The RAM disk ID.</p>
    #[doc(hidden)]
    pub ramdisk_id: std::option::Option<crate::types::AttributeValue>,
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    #[doc(hidden)]
    pub root_device_name: std::option::Option<crate::types::AttributeValue>,
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    #[doc(hidden)]
    pub source_dest_check: std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is enabled.</p>
    #[doc(hidden)]
    pub sriov_net_support: std::option::Option<crate::types::AttributeValue>,
    /// <p>The user data.</p>
    #[doc(hidden)]
    pub user_data: std::option::Option<crate::types::AttributeValue>,
    /// <p>To enable the instance for Amazon Web Services Stop Protection, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    #[doc(hidden)]
    pub disable_api_stop: std::option::Option<crate::types::AttributeBooleanValue>,
    _request_id: Option<String>,
}
impl DescribeInstanceAttributeOutput {
    /// <p>The security groups associated with the instance.</p>
    pub fn groups(&self) -> std::option::Option<&[crate::types::GroupIdentifier]> {
        self.groups.as_deref()
    }
    /// <p>The block device mapping of the instance.</p>
    pub fn block_device_mappings(
        &self,
    ) -> std::option::Option<&[crate::types::InstanceBlockDeviceMapping]> {
        self.block_device_mappings.as_deref()
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance through the Amazon EC2 console, CLI, or API; otherwise, you can.</p>
    pub fn disable_api_termination(
        &self,
    ) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.disable_api_termination.as_ref()
    }
    /// <p>Indicates whether enhanced networking with ENA is enabled.</p>
    pub fn ena_support(&self) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.ena_support.as_ref()
    }
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn enclave_options(&self) -> std::option::Option<&crate::types::EnclaveOptions> {
        self.enclave_options.as_ref()
    }
    /// <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p>
    pub fn ebs_optimized(&self) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.ebs_optimized.as_ref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn instance_initiated_shutdown_behavior(
        &self,
    ) -> std::option::Option<&crate::types::AttributeValue> {
        self.instance_initiated_shutdown_behavior.as_ref()
    }
    /// <p>The instance type.</p>
    pub fn instance_type(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.instance_type.as_ref()
    }
    /// <p>The kernel ID.</p>
    pub fn kernel_id(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.kernel_id.as_ref()
    }
    /// <p>A list of product codes.</p>
    pub fn product_codes(&self) -> std::option::Option<&[crate::types::ProductCode]> {
        self.product_codes.as_deref()
    }
    /// <p>The RAM disk ID.</p>
    pub fn ramdisk_id(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.ramdisk_id.as_ref()
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn root_device_name(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.root_device_name.as_ref()
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(&self) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.source_dest_check.as_ref()
    }
    /// <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is enabled.</p>
    pub fn sriov_net_support(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.sriov_net_support.as_ref()
    }
    /// <p>The user data.</p>
    pub fn user_data(&self) -> std::option::Option<&crate::types::AttributeValue> {
        self.user_data.as_ref()
    }
    /// <p>To enable the instance for Amazon Web Services Stop Protection, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn disable_api_stop(&self) -> std::option::Option<&crate::types::AttributeBooleanValue> {
        self.disable_api_stop.as_ref()
    }
}
impl aws_http::request_id::RequestId for DescribeInstanceAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeInstanceAttributeOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInstanceAttributeOutput`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput).
    pub fn builder() -> crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeOutputBuilder{
        crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeOutputBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape =
    crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput;
/// A builder for [`DescribeInstanceAttributeOutput`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput).
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
pub struct DescribeInstanceAttributeOutputBuilder {
    pub(crate) groups: std::option::Option<std::vec::Vec<crate::types::GroupIdentifier>>,
    pub(crate) block_device_mappings:
        std::option::Option<std::vec::Vec<crate::types::InstanceBlockDeviceMapping>>,
    pub(crate) disable_api_termination: std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) ena_support: std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) enclave_options: std::option::Option<crate::types::EnclaveOptions>,
    pub(crate) ebs_optimized: std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) instance_id: std::option::Option<std::string::String>,
    pub(crate) instance_initiated_shutdown_behavior:
        std::option::Option<crate::types::AttributeValue>,
    pub(crate) instance_type: std::option::Option<crate::types::AttributeValue>,
    pub(crate) kernel_id: std::option::Option<crate::types::AttributeValue>,
    pub(crate) product_codes: std::option::Option<std::vec::Vec<crate::types::ProductCode>>,
    pub(crate) ramdisk_id: std::option::Option<crate::types::AttributeValue>,
    pub(crate) root_device_name: std::option::Option<crate::types::AttributeValue>,
    pub(crate) source_dest_check: std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) sriov_net_support: std::option::Option<crate::types::AttributeValue>,
    pub(crate) user_data: std::option::Option<crate::types::AttributeValue>,
    pub(crate) disable_api_stop: std::option::Option<crate::types::AttributeBooleanValue>,
    _request_id: Option<String>,
}
impl DescribeInstanceAttributeOutputBuilder {
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The security groups associated with the instance.</p>
    pub fn groups(mut self, input: crate::types::GroupIdentifier) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input);
        self.groups = Some(v);
        self
    }
    /// <p>The security groups associated with the instance.</p>
    pub fn set_groups(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::GroupIdentifier>>,
    ) -> Self {
        self.groups = input;
        self
    }
    /// Appends an item to `block_device_mappings`.
    ///
    /// To override the contents of this collection use [`set_block_device_mappings`](Self::set_block_device_mappings).
    ///
    /// <p>The block device mapping of the instance.</p>
    pub fn block_device_mappings(
        mut self,
        input: crate::types::InstanceBlockDeviceMapping,
    ) -> Self {
        let mut v = self.block_device_mappings.unwrap_or_default();
        v.push(input);
        self.block_device_mappings = Some(v);
        self
    }
    /// <p>The block device mapping of the instance.</p>
    pub fn set_block_device_mappings(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::InstanceBlockDeviceMapping>>,
    ) -> Self {
        self.block_device_mappings = input;
        self
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance through the Amazon EC2 console, CLI, or API; otherwise, you can.</p>
    pub fn disable_api_termination(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.disable_api_termination = Some(input);
        self
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance through the Amazon EC2 console, CLI, or API; otherwise, you can.</p>
    pub fn set_disable_api_termination(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.disable_api_termination = input;
        self
    }
    /// <p>Indicates whether enhanced networking with ENA is enabled.</p>
    pub fn ena_support(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.ena_support = Some(input);
        self
    }
    /// <p>Indicates whether enhanced networking with ENA is enabled.</p>
    pub fn set_ena_support(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.ena_support = input;
        self
    }
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn enclave_options(mut self, input: crate::types::EnclaveOptions) -> Self {
        self.enclave_options = Some(input);
        self
    }
    /// <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn set_enclave_options(
        mut self,
        input: std::option::Option<crate::types::EnclaveOptions>,
    ) -> Self {
        self.enclave_options = input;
        self
    }
    /// <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p>
    pub fn ebs_optimized(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.ebs_optimized = Some(input);
        self
    }
    /// <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p>
    pub fn set_ebs_optimized(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.ebs_optimized = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.instance_id = Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn instance_initiated_shutdown_behavior(
        mut self,
        input: crate::types::AttributeValue,
    ) -> Self {
        self.instance_initiated_shutdown_behavior = Some(input);
        self
    }
    /// <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn set_instance_initiated_shutdown_behavior(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.instance_initiated_shutdown_behavior = input;
        self
    }
    /// <p>The instance type.</p>
    pub fn instance_type(mut self, input: crate::types::AttributeValue) -> Self {
        self.instance_type = Some(input);
        self
    }
    /// <p>The instance type.</p>
    pub fn set_instance_type(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The kernel ID.</p>
    pub fn kernel_id(mut self, input: crate::types::AttributeValue) -> Self {
        self.kernel_id = Some(input);
        self
    }
    /// <p>The kernel ID.</p>
    pub fn set_kernel_id(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.kernel_id = input;
        self
    }
    /// Appends an item to `product_codes`.
    ///
    /// To override the contents of this collection use [`set_product_codes`](Self::set_product_codes).
    ///
    /// <p>A list of product codes.</p>
    pub fn product_codes(mut self, input: crate::types::ProductCode) -> Self {
        let mut v = self.product_codes.unwrap_or_default();
        v.push(input);
        self.product_codes = Some(v);
        self
    }
    /// <p>A list of product codes.</p>
    pub fn set_product_codes(
        mut self,
        input: std::option::Option<std::vec::Vec<crate::types::ProductCode>>,
    ) -> Self {
        self.product_codes = input;
        self
    }
    /// <p>The RAM disk ID.</p>
    pub fn ramdisk_id(mut self, input: crate::types::AttributeValue) -> Self {
        self.ramdisk_id = Some(input);
        self
    }
    /// <p>The RAM disk ID.</p>
    pub fn set_ramdisk_id(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.ramdisk_id = input;
        self
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn root_device_name(mut self, input: crate::types::AttributeValue) -> Self {
        self.root_device_name = Some(input);
        self
    }
    /// <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    pub fn set_root_device_name(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.root_device_name = input;
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.source_dest_check = Some(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.source_dest_check = input;
        self
    }
    /// <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is enabled.</p>
    pub fn sriov_net_support(mut self, input: crate::types::AttributeValue) -> Self {
        self.sriov_net_support = Some(input);
        self
    }
    /// <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is enabled.</p>
    pub fn set_sriov_net_support(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.sriov_net_support = input;
        self
    }
    /// <p>The user data.</p>
    pub fn user_data(mut self, input: crate::types::AttributeValue) -> Self {
        self.user_data = Some(input);
        self
    }
    /// <p>The user data.</p>
    pub fn set_user_data(
        mut self,
        input: std::option::Option<crate::types::AttributeValue>,
    ) -> Self {
        self.user_data = input;
        self
    }
    /// <p>To enable the instance for Amazon Web Services Stop Protection, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn disable_api_stop(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.disable_api_stop = Some(input);
        self
    }
    /// <p>To enable the instance for Amazon Web Services Stop Protection, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    pub fn set_disable_api_stop(
        mut self,
        input: std::option::Option<crate::types::AttributeBooleanValue>,
    ) -> Self {
        self.disable_api_stop = input;
        self
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInstanceAttributeOutput`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput {
        crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput {
            groups: self.groups,
            block_device_mappings: self.block_device_mappings,
            disable_api_termination: self.disable_api_termination,
            ena_support: self.ena_support,
            enclave_options: self.enclave_options,
            ebs_optimized: self.ebs_optimized,
            instance_id: self.instance_id,
            instance_initiated_shutdown_behavior: self.instance_initiated_shutdown_behavior,
            instance_type: self.instance_type,
            kernel_id: self.kernel_id,
            product_codes: self.product_codes,
            ramdisk_id: self.ramdisk_id,
            root_device_name: self.root_device_name,
            source_dest_check: self.source_dest_check,
            sriov_net_support: self.sriov_net_support,
            user_data: self.user_data,
            disable_api_stop: self.disable_api_stop,
            _request_id: self._request_id,
        }
    }
}
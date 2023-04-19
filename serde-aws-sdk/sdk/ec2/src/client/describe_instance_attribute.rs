// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInstanceAttribute`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`attribute(InstanceAttributeName)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::attribute) / [`set_attribute(Option<InstanceAttributeName>)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::set_attribute): <p>The instance attribute.</p>  <p>Note: The <code>enaSupport</code> attribute is not supported at this time.</p>
    ///   - [`dry_run(bool)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`instance_id(impl Into<String>)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::set_instance_id): <p>The ID of the instance.</p>
    /// - On success, responds with [`DescribeInstanceAttributeOutput`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput) with field(s):
    ///   - [`groups(Option<Vec<GroupIdentifier>>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::groups): <p>The security groups associated with the instance.</p>
    ///   - [`block_device_mappings(Option<Vec<InstanceBlockDeviceMapping>>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::block_device_mappings): <p>The block device mapping of the instance.</p>
    ///   - [`disable_api_termination(Option<AttributeBooleanValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::disable_api_termination): <p>If the value is <code>true</code>, you can't terminate the instance through the Amazon EC2 console, CLI, or API; otherwise, you can.</p>
    ///   - [`ena_support(Option<AttributeBooleanValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::ena_support): <p>Indicates whether enhanced networking with ENA is enabled.</p>
    ///   - [`enclave_options(Option<EnclaveOptions>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::enclave_options): <p>To enable the instance for Amazon Web Services Nitro Enclaves, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    ///   - [`ebs_optimized(Option<AttributeBooleanValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::ebs_optimized): <p>Indicates whether the instance is optimized for Amazon EBS I/O.</p>
    ///   - [`instance_id(Option<String>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::instance_id): <p>The ID of the instance.</p>
    ///   - [`instance_initiated_shutdown_behavior(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::instance_initiated_shutdown_behavior): <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    ///   - [`instance_type(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::instance_type): <p>The instance type.</p>
    ///   - [`kernel_id(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::kernel_id): <p>The kernel ID.</p>
    ///   - [`product_codes(Option<Vec<ProductCode>>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::product_codes): <p>A list of product codes.</p>
    ///   - [`ramdisk_id(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::ramdisk_id): <p>The RAM disk ID.</p>
    ///   - [`root_device_name(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::root_device_name): <p>The device name of the root device volume (for example, <code>/dev/sda1</code>).</p>
    ///   - [`source_dest_check(Option<AttributeBooleanValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::source_dest_check): <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    ///   - [`sriov_net_support(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::sriov_net_support): <p>Indicates whether enhanced networking with the Intel 82599 Virtual Function interface is enabled.</p>
    ///   - [`user_data(Option<AttributeValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::user_data): <p>The user data.</p>
    ///   - [`disable_api_stop(Option<AttributeBooleanValue>)`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeOutput::disable_api_stop): <p>To enable the instance for Amazon Web Services Stop Protection, set this parameter to <code>true</code>; otherwise, set it to <code>false</code>.</p>
    /// - On failure, responds with [`SdkError<DescribeInstanceAttributeError>`](crate::operation::describe_instance_attribute::DescribeInstanceAttributeError)
    pub fn describe_instance_attribute(&self) -> crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder{
        crate::operation::describe_instance_attribute::builders::DescribeInstanceAttributeFluentBuilder::new(self.handle.clone())
    }
}

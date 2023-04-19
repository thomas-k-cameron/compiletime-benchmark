// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RunInstances`](crate::operation::run_instances::builders::RunInstancesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`block_device_mappings(Vec<BlockDeviceMapping>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::block_device_mappings) / [`set_block_device_mappings(Option<Vec<BlockDeviceMapping>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_block_device_mappings): <p>The block device mapping, which defines the EBS volumes and instance store volumes to attach to the instance at launch. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html">Block device mappings</a> in the <i>Amazon EC2 User Guide</i>.</p>
    ///   - [`image_id(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::image_id) / [`set_image_id(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_image_id): <p>The ID of the AMI. An AMI ID is required to launch an instance and must be specified here or in a launch template.</p>
    ///   - [`instance_type(InstanceType)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::instance_type) / [`set_instance_type(Option<InstanceType>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_instance_type): <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>  <p>Default: <code>m1.small</code> </p>
    ///   - [`ipv6_address_count(i32)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::ipv6_address_count) / [`set_ipv6_address_count(Option<i32>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_ipv6_address_count): <p>[EC2-VPC] The number of IPv6 addresses to associate with the primary network interface. Amazon EC2 chooses the IPv6 addresses from the range of your subnet. You cannot specify this option and the option to assign specific IPv6 addresses in the same request. You can specify this option if you've specified a minimum number of instances to launch.</p>  <p>You cannot specify this option and the network interfaces option in the same request.</p>
    ///   - [`ipv6_addresses(Vec<InstanceIpv6Address>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::ipv6_addresses) / [`set_ipv6_addresses(Option<Vec<InstanceIpv6Address>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_ipv6_addresses): <p>[EC2-VPC] The IPv6 addresses from the range of the subnet to associate with the primary network interface. You cannot specify this option and the option to assign a number of IPv6 addresses in the same request. You cannot specify this option if you've specified a minimum number of instances to launch.</p>  <p>You cannot specify this option and the network interfaces option in the same request.</p>
    ///   - [`kernel_id(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::kernel_id) / [`set_kernel_id(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_kernel_id): <p>The ID of the kernel.</p> <important>   <p>We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedkernels.html">PV-GRUB</a> in the <i>Amazon EC2 User Guide</i>.</p>  </important>
    ///   - [`key_name(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::key_name) / [`set_key_name(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_key_name): <p>The name of the key pair. You can create a key pair using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateKeyPair.html">CreateKeyPair</a> or <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ImportKeyPair.html">ImportKeyPair</a>.</p> <important>   <p>If you do not specify a key pair, you can't connect to the instance unless you choose an AMI that is configured to allow users another way to log in.</p>  </important>
    ///   - [`max_count(i32)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::max_count) / [`set_max_count(Option<i32>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_max_count): <p>The maximum number of instances to launch. If you specify more instances than Amazon EC2 can launch in the target Availability Zone, Amazon EC2 launches the largest possible number of instances above <code>MinCount</code>.</p>  <p>Constraints: Between 1 and the maximum number you're allowed for the specified instance type. For more information about the default limits, and how to request an increase, see <a href="http://aws.amazon.com/ec2/faqs/#How_many_instances_can_I_run_in_Amazon_EC2">How many instances can I run in Amazon EC2</a> in the Amazon EC2 FAQ.</p>
    ///   - [`min_count(i32)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::min_count) / [`set_min_count(Option<i32>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_min_count): <p>The minimum number of instances to launch. If you specify a minimum that is more instances than Amazon EC2 can launch in the target Availability Zone, Amazon EC2 launches no instances.</p>  <p>Constraints: Between 1 and the maximum number you're allowed for the specified instance type. For more information about the default limits, and how to request an increase, see <a href="http://aws.amazon.com/ec2/faqs/#How_many_instances_can_I_run_in_Amazon_EC2">How many instances can I run in Amazon EC2</a> in the Amazon EC2 General FAQ.</p>
    ///   - [`monitoring(RunInstancesMonitoringEnabled)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::monitoring) / [`set_monitoring(Option<RunInstancesMonitoringEnabled>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_monitoring): <p>Specifies whether detailed monitoring is enabled for the instance.</p>
    ///   - [`placement(Placement)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::placement) / [`set_placement(Option<Placement>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_placement): <p>The placement for the instance.</p>
    ///   - [`ramdisk_id(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::ramdisk_id) / [`set_ramdisk_id(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_ramdisk_id): <p>The ID of the RAM disk to select. Some kernels require additional drivers at launch. Check the kernel requirements for information about whether you need to specify a RAM disk. To find kernel requirements, go to the Amazon Web Services Resource Center and search for the kernel ID.</p> <important>   <p>We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedkernels.html">PV-GRUB</a> in the <i>Amazon EC2 User Guide</i>.</p>  </important>
    ///   - [`security_group_ids(Vec<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::security_group_ids) / [`set_security_group_ids(Option<Vec<String>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_security_group_ids): <p>The IDs of the security groups. You can create a security group using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateSecurityGroup.html">CreateSecurityGroup</a>.</p>  <p>If you specify a network interface, you must specify any security groups as part of the network interface.</p>
    ///   - [`security_groups(Vec<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::security_groups) / [`set_security_groups(Option<Vec<String>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_security_groups): <p>[EC2-Classic, default VPC] The names of the security groups.</p>  <p>If you specify a network interface, you must specify any security groups as part of the network interface.</p>  <p>Default: Amazon EC2 uses the default security group.</p>
    ///   - [`subnet_id(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::subnet_id) / [`set_subnet_id(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_subnet_id): <p>[EC2-VPC] The ID of the subnet to launch the instance into.</p>  <p>If you specify a network interface, you must specify any subnets as part of the network interface.</p>
    ///   - [`user_data(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::user_data) / [`set_user_data(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_user_data): <p>The user data script to make available to the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/user-data.html">Run commands on your Linux instance at launch</a> and <a href="https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/ec2-windows-user-data.html">Run commands on your Windows instance at launch</a>. If you are using a command line tool, base64-encoding is performed for you, and you can load the text from a file. Otherwise, you must provide base64-encoded text. User data is limited to 16 KB.</p>
    ///   - [`additional_info(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::additional_info) / [`set_additional_info(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_additional_info): <p>Reserved.</p>
    ///   - [`client_token(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_client_token): <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, a randomly generated token is used for the request to ensure idempotency.</p>  <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>  <p>Constraints: Maximum 64 ASCII characters</p>
    ///   - [`disable_api_termination(bool)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::disable_api_termination) / [`set_disable_api_termination(Option<bool>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_disable_api_termination): <p>If you set this parameter to <code>true</code>, you can't terminate the instance using the Amazon EC2 console, CLI, or API; otherwise, you can. To change this attribute after launch, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyInstanceAttribute.html">ModifyInstanceAttribute</a>. Alternatively, if you set <code>InstanceInitiatedShutdownBehavior</code> to <code>terminate</code>, you can terminate the instance by running the shutdown command from the instance.</p>  <p>Default: <code>false</code> </p>
    ///   - [`dry_run(bool)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ebs_optimized(bool)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::ebs_optimized) / [`set_ebs_optimized(Option<bool>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_ebs_optimized): <p>Indicates whether the instance is optimized for Amazon EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal Amazon EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS-optimized instance.</p>  <p>Default: <code>false</code> </p>
    ///   - [`iam_instance_profile(IamInstanceProfileSpecification)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::iam_instance_profile) / [`set_iam_instance_profile(Option<IamInstanceProfileSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_iam_instance_profile): <p>The name or Amazon Resource Name (ARN) of an IAM instance profile.</p>
    ///   - [`instance_initiated_shutdown_behavior(ShutdownBehavior)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::instance_initiated_shutdown_behavior) / [`set_instance_initiated_shutdown_behavior(Option<ShutdownBehavior>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_instance_initiated_shutdown_behavior): <p>Indicates whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>  <p>Default: <code>stop</code> </p>
    ///   - [`network_interfaces(Vec<InstanceNetworkInterfaceSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::network_interfaces) / [`set_network_interfaces(Option<Vec<InstanceNetworkInterfaceSpecification>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_network_interfaces): <p>The network interfaces to associate with the instance. If you specify a network interface, you must specify any security groups and subnets as part of the network interface.</p>
    ///   - [`private_ip_address(impl Into<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::private_ip_address) / [`set_private_ip_address(Option<String>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_private_ip_address): <p>[EC2-VPC] The primary IPv4 address. You must specify a value from the IPv4 address range of the subnet.</p>  <p>Only one private IP address can be designated as primary. You can't specify this option if you've specified the option to designate a private IP address as the primary IP address in a network interface specification. You cannot specify this option if you're launching more than one instance in the request.</p>  <p>You cannot specify this option and the network interfaces option in the same request.</p>
    ///   - [`elastic_gpu_specification(Vec<ElasticGpuSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::elastic_gpu_specification) / [`set_elastic_gpu_specification(Option<Vec<ElasticGpuSpecification>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_elastic_gpu_specification): <p>An elastic GPU to associate with the instance. An Elastic GPU is a GPU resource that you can attach to your Windows instance to accelerate the graphics performance of your applications. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/WindowsGuide/elastic-graphics.html">Amazon EC2 Elastic GPUs</a> in the <i>Amazon EC2 User Guide</i>.</p>
    ///   - [`elastic_inference_accelerators(Vec<ElasticInferenceAccelerator>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::elastic_inference_accelerators) / [`set_elastic_inference_accelerators(Option<Vec<ElasticInferenceAccelerator>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_elastic_inference_accelerators): <p>An elastic inference accelerator to associate with the instance. Elastic inference accelerators are a resource you can attach to your Amazon EC2 instances to accelerate your Deep Learning (DL) inference workloads.</p>  <p>You cannot specify accelerators from different generations in the same request.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_tag_specifications): <p>The tags to apply to the resources that are created during instance launch.</p>  <p>You can specify tags for the following resources only:</p>  <ul>   <li> <p>Instances</p> </li>   <li> <p>Volumes</p> </li>   <li> <p>Elastic graphics</p> </li>   <li> <p>Spot Instance requests</p> </li>   <li> <p>Network interfaces</p> </li>  </ul>  <p>To tag a resource after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>.</p>
    ///   - [`launch_template(LaunchTemplateSpecification)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::launch_template) / [`set_launch_template(Option<LaunchTemplateSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_launch_template): <p>The launch template to use to launch the instances. Any parameters that you specify in <code>RunInstances</code> override the same parameters in the launch template. You can specify either the name or ID of a launch template, but not both.</p>
    ///   - [`instance_market_options(InstanceMarketOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::instance_market_options) / [`set_instance_market_options(Option<InstanceMarketOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_instance_market_options): <p>The market (purchasing) option for the instances.</p>  <p>For <code>RunInstances</code>, persistent Spot Instance requests are only supported when <b>InstanceInterruptionBehavior</b> is set to either <code>hibernate</code> or <code>stop</code>.</p>
    ///   - [`credit_specification(CreditSpecificationRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::credit_specification) / [`set_credit_specification(Option<CreditSpecificationRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_credit_specification): <p>The credit option for CPU usage of the burstable performance instance. Valid values are <code>standard</code> and <code>unlimited</code>. To change this attribute after launch, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyInstanceCreditSpecification.html"> ModifyInstanceCreditSpecification</a>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/burstable-performance-instances.html">Burstable performance instances</a> in the <i>Amazon EC2 User Guide</i>.</p>  <p>Default: <code>standard</code> (T2 instances) or <code>unlimited</code> (T3/T3a/T4g instances)</p>  <p>For T3 instances with <code>host</code> tenancy, only <code>standard</code> is supported.</p>
    ///   - [`cpu_options(CpuOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::cpu_options) / [`set_cpu_options(Option<CpuOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_cpu_options): <p>The CPU options for the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-optimize-cpu.html">Optimize CPU options</a> in the <i>Amazon EC2 User Guide</i>.</p>
    ///   - [`capacity_reservation_specification(CapacityReservationSpecification)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::capacity_reservation_specification) / [`set_capacity_reservation_specification(Option<CapacityReservationSpecification>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_capacity_reservation_specification): <p>Information about the Capacity Reservation targeting option. If you do not specify this parameter, the instance's Capacity Reservation preference defaults to <code>open</code>, which enables it to run in any open Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p>
    ///   - [`hibernation_options(HibernationOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::hibernation_options) / [`set_hibernation_options(Option<HibernationOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_hibernation_options): <p>Indicates whether an instance is enabled for hibernation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Hibernate.html">Hibernate your instance</a> in the <i>Amazon EC2 User Guide</i>.</p>  <p>You can't enable hibernation and Amazon Web Services Nitro Enclaves on the same instance.</p>
    ///   - [`license_specifications(Vec<LicenseConfigurationRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::license_specifications) / [`set_license_specifications(Option<Vec<LicenseConfigurationRequest>>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_license_specifications): <p>The license configurations.</p>
    ///   - [`metadata_options(InstanceMetadataOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::metadata_options) / [`set_metadata_options(Option<InstanceMetadataOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_metadata_options): <p>The metadata options for the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">Instance metadata and user data</a>.</p>
    ///   - [`enclave_options(EnclaveOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::enclave_options) / [`set_enclave_options(Option<EnclaveOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_enclave_options): <p>Indicates whether the instance is enabled for Amazon Web Services Nitro Enclaves. For more information, see <a href="https://docs.aws.amazon.com/enclaves/latest/user/nitro-enclave.html"> What is Amazon Web Services Nitro Enclaves?</a> in the <i>Amazon Web Services Nitro Enclaves User Guide</i>.</p>  <p>You can't enable Amazon Web Services Nitro Enclaves and hibernation on the same instance.</p>
    ///   - [`private_dns_name_options(PrivateDnsNameOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::private_dns_name_options) / [`set_private_dns_name_options(Option<PrivateDnsNameOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_private_dns_name_options): <p>The options for the instance hostname. The default values are inherited from the subnet.</p>
    ///   - [`maintenance_options(InstanceMaintenanceOptionsRequest)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::maintenance_options) / [`set_maintenance_options(Option<InstanceMaintenanceOptionsRequest>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_maintenance_options): <p>The maintenance and recovery options for the instance.</p>
    ///   - [`disable_api_stop(bool)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::disable_api_stop) / [`set_disable_api_stop(Option<bool>)`](crate::operation::run_instances::builders::RunInstancesFluentBuilder::set_disable_api_stop): <p>Indicates whether an instance is enabled for stop protection. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection">Stop protection</a>. </p>
    /// - On success, responds with [`RunInstancesOutput`](crate::operation::run_instances::RunInstancesOutput) with field(s):
    ///   - [`groups(Option<Vec<GroupIdentifier>>)`](crate::operation::run_instances::RunInstancesOutput::groups): <p>[EC2-Classic only] The security groups.</p>
    ///   - [`instances(Option<Vec<Instance>>)`](crate::operation::run_instances::RunInstancesOutput::instances): <p>The instances.</p>
    ///   - [`owner_id(Option<String>)`](crate::operation::run_instances::RunInstancesOutput::owner_id): <p>The ID of the Amazon Web Services account that owns the reservation.</p>
    ///   - [`requester_id(Option<String>)`](crate::operation::run_instances::RunInstancesOutput::requester_id): <p>The ID of the requester that launched the instances on your behalf (for example, Amazon Web Services Management Console or Auto Scaling).</p>
    ///   - [`reservation_id(Option<String>)`](crate::operation::run_instances::RunInstancesOutput::reservation_id): <p>The ID of the reservation.</p>
    /// - On failure, responds with [`SdkError<RunInstancesError>`](crate::operation::run_instances::RunInstancesError)
    pub fn run_instances(
        &self,
    ) -> crate::operation::run_instances::builders::RunInstancesFluentBuilder {
        crate::operation::run_instances::builders::RunInstancesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

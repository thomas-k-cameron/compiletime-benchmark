// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyIpamPool`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_pool_id(impl Into<String>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_ipam_pool_id): <p>The ID of the IPAM pool you want to modify.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_description): <p>The description of the IPAM pool you want to modify.</p>
    ///   - [`auto_import(bool)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::auto_import) / [`set_auto_import(Option<bool>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_auto_import): <p>If true, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM. The CIDRs that will be allocated for these resources must not already be allocated to other resources in order for the import to succeed. IPAM will import a CIDR regardless of its compliance with the pool's allocation rules, so a resource might be imported and subsequently marked as noncompliant. If IPAM discovers multiple CIDRs that overlap, IPAM will import the largest CIDR only. If IPAM discovers multiple CIDRs with matching CIDRs, IPAM will randomly import one of them only. </p>  <p>A locale must be set on the pool for this feature to work.</p>
    ///   - [`allocation_min_netmask_length(i32)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::allocation_min_netmask_length) / [`set_allocation_min_netmask_length(Option<i32>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_allocation_min_netmask_length): <p>The minimum netmask length required for CIDR allocations in this IPAM pool to be compliant. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128. The minimum netmask length must be less than the maximum netmask length.</p>
    ///   - [`allocation_max_netmask_length(i32)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::allocation_max_netmask_length) / [`set_allocation_max_netmask_length(Option<i32>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_allocation_max_netmask_length): <p>The maximum netmask length possible for CIDR allocations in this IPAM pool to be compliant. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.The maximum netmask length must be greater than the minimum netmask length.</p>
    ///   - [`allocation_default_netmask_length(i32)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::allocation_default_netmask_length) / [`set_allocation_default_netmask_length(Option<i32>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_allocation_default_netmask_length): <p>The default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16.</p>
    ///   - [`clear_allocation_default_netmask_length(bool)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::clear_allocation_default_netmask_length) / [`set_clear_allocation_default_netmask_length(Option<bool>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_clear_allocation_default_netmask_length): <p>Clear the default netmask length allocation rule for this pool.</p>
    ///   - [`add_allocation_resource_tags(Vec<RequestIpamResourceTag>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::add_allocation_resource_tags) / [`set_add_allocation_resource_tags(Option<Vec<RequestIpamResourceTag>>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_add_allocation_resource_tags): <p>Add tag allocation rules to a pool. For more information about allocation rules, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/create-top-ipam.html">Create a top-level pool</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    ///   - [`remove_allocation_resource_tags(Vec<RequestIpamResourceTag>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::remove_allocation_resource_tags) / [`set_remove_allocation_resource_tags(Option<Vec<RequestIpamResourceTag>>)`](crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::set_remove_allocation_resource_tags): <p>Remove tag allocation rules from a pool.</p>
    /// - On success, responds with [`ModifyIpamPoolOutput`](crate::operation::modify_ipam_pool::ModifyIpamPoolOutput) with field(s):
    ///   - [`ipam_pool(Option<IpamPool>)`](crate::operation::modify_ipam_pool::ModifyIpamPoolOutput::ipam_pool): <p>The results of the modification.</p>
    /// - On failure, responds with [`SdkError<ModifyIpamPoolError>`](crate::operation::modify_ipam_pool::ModifyIpamPoolError)
    pub fn modify_ipam_pool(
        &self,
    ) -> crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder {
        crate::operation::modify_ipam_pool::builders::ModifyIpamPoolFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
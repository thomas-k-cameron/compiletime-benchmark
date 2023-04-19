// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyIpam`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::set_dry_run): <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    ///   - [`ipam_id(impl Into<String>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::ipam_id) / [`set_ipam_id(Option<String>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::set_ipam_id): <p>The ID of the IPAM you want to modify.</p>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::set_description): <p>The description of the IPAM you want to modify.</p>
    ///   - [`add_operating_regions(Vec<AddIpamOperatingRegion>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::add_operating_regions) / [`set_add_operating_regions(Option<Vec<AddIpamOperatingRegion>>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::set_add_operating_regions): <p>Choose the operating Regions for the IPAM. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>  <p>For more information about operating Regions, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/create-ipam.html">Create an IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
    ///   - [`remove_operating_regions(Vec<RemoveIpamOperatingRegion>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::remove_operating_regions) / [`set_remove_operating_regions(Option<Vec<RemoveIpamOperatingRegion>>)`](crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::set_remove_operating_regions): <p>The operating Regions to remove.</p>
    /// - On success, responds with [`ModifyIpamOutput`](crate::operation::modify_ipam::ModifyIpamOutput) with field(s):
    ///   - [`ipam(Option<Ipam>)`](crate::operation::modify_ipam::ModifyIpamOutput::ipam): <p>The results of the modification.</p>
    /// - On failure, responds with [`SdkError<ModifyIpamError>`](crate::operation::modify_ipam::ModifyIpamError)
    pub fn modify_ipam(&self) -> crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder {
        crate::operation::modify_ipam::builders::ModifyIpamFluentBuilder::new(self.handle.clone())
    }
}

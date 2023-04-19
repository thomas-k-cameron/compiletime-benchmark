// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateDhcpOptions`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dhcp_options_id(impl Into<String>)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::dhcp_options_id) / [`set_dhcp_options_id(Option<String>)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::set_dhcp_options_id): <p>The ID of the DHCP options set, or <code>default</code> to associate no DHCP options with the VPC.</p>
    ///   - [`vpc_id(impl Into<String>)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::set_vpc_id): <p>The ID of the VPC.</p>
    ///   - [`dry_run(bool)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`AssociateDhcpOptionsOutput`](crate::operation::associate_dhcp_options::AssociateDhcpOptionsOutput)
    /// - On failure, responds with [`SdkError<AssociateDhcpOptionsError>`](crate::operation::associate_dhcp_options::AssociateDhcpOptionsError)
    pub fn associate_dhcp_options(
        &self,
    ) -> crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder {
        crate::operation::associate_dhcp_options::builders::AssociateDhcpOptionsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
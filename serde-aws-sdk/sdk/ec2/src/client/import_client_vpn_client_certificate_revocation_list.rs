// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportClientVpnClientCertificateRevocationList`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_vpn_endpoint_id(impl Into<String>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::client_vpn_endpoint_id) / [`set_client_vpn_endpoint_id(Option<String>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::set_client_vpn_endpoint_id): <p>The ID of the Client VPN endpoint to which the client certificate revocation list applies.</p>
    ///   - [`certificate_revocation_list(impl Into<String>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::certificate_revocation_list) / [`set_certificate_revocation_list(Option<String>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::set_certificate_revocation_list): <p>The client certificate revocation list file. For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/cvpn-working-certificates.html#cvpn-working-certificates-generate">Generate a Client Certificate Revocation List</a> in the <i>Client VPN Administrator Guide</i>.</p>
    ///   - [`dry_run(bool)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`ImportClientVpnClientCertificateRevocationListOutput`](crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<ImportClientVpnClientCertificateRevocationListError>`](crate::operation::import_client_vpn_client_certificate_revocation_list::ImportClientVpnClientCertificateRevocationListError)
    pub fn import_client_vpn_client_certificate_revocation_list(&self) -> crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder{
        crate::operation::import_client_vpn_client_certificate_revocation_list::builders::ImportClientVpnClientCertificateRevocationListFluentBuilder::new(self.handle.clone())
    }
}

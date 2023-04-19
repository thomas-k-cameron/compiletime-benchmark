// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_certificate_authentication_request(
    mut writer: aws_smithy_query::QueryValueWriter,
    input: &crate::types::CertificateAuthenticationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientRootCertificateChainArn");
    if let Some(var_2) = &input.client_root_certificate_chain_arn {
        scope_1.string(var_2);
    }
    Ok(())
}

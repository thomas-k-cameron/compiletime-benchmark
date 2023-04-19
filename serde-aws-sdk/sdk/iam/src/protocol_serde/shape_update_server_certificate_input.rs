// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_server_certificate_input_input(
    input: &crate::operation::update_server_certificate::UpdateServerCertificateInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "UpdateServerCertificate", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ServerCertificateName");
    if let Some(var_2) = &input.server_certificate_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NewPath");
    if let Some(var_4) = &input.new_path {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NewServerCertificateName");
    if let Some(var_6) = &input.new_server_certificate_name {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

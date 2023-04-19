// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_service_specific_credential_input_input(
    input: &crate::operation::update_service_specific_credential::UpdateServiceSpecificCredentialInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "UpdateServiceSpecificCredential",
        "2010-05-08",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UserName");
    if let Some(var_2) = &input.user_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ServiceSpecificCredentialId");
    if let Some(var_4) = &input.service_specific_credential_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Status");
    if let Some(var_6) = &input.status {
        scope_5.string(var_6.as_str());
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resync_mfa_device_input_input(
    input: &crate::operation::resync_mfa_device::ResyncMfaDeviceInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ResyncMFADevice", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("UserName");
    if let Some(var_2) = &input.user_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SerialNumber");
    if let Some(var_4) = &input.serial_number {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AuthenticationCode1");
    if let Some(var_6) = &input.authentication_code1 {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("AuthenticationCode2");
    if let Some(var_8) = &input.authentication_code2 {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}
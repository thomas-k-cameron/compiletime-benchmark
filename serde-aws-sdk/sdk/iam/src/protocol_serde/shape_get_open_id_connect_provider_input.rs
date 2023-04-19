// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_open_id_connect_provider_input_input(
    input: &crate::operation::get_open_id_connect_provider::GetOpenIdConnectProviderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "GetOpenIDConnectProvider", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("OpenIDConnectProviderArn");
    if let Some(var_2) = &input.open_id_connect_provider_arn {
        scope_1.string(var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

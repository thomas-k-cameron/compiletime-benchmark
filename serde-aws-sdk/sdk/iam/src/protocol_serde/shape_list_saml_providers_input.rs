// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_saml_providers_input_input(
    input: &crate::operation::list_saml_providers::ListSamlProvidersInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ListSAMLProviders", "2010-05-08");
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_untag_open_id_connect_provider_input_input(
    input: &crate::operation::untag_open_id_connect_provider::UntagOpenIdConnectProviderInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "UntagOpenIDConnectProvider", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("OpenIDConnectProviderArn");
    if let Some(var_2) = &input.open_id_connect_provider_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TagKeys");
    if let Some(var_4) = &input.tag_keys {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

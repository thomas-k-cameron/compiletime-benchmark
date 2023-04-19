// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_key_pair_input_input(
    input: &crate::operation::import_key_pair::ImportKeyPairInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ImportKeyPair", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("KeyName");
    if let Some(var_4) = &input.key_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PublicKeyMaterial");
    if let Some(var_6) = &input.public_key_material {
        scope_5.string(&aws_smithy_types::base64::encode(var_6));
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("TagSpecification");
    if let Some(var_8) = &input.tag_specifications {
        let mut list_10 = scope_7.start_list(true, Some("item"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_11, item_9,
            )?;
        }
        list_10.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

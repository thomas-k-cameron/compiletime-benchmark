// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_snapshots_input_input(
    input: &crate::operation::create_snapshots::CreateSnapshotsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateSnapshots", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Description");
    if let Some(var_2) = &input.description {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceSpecification");
    if let Some(var_4) = &input.instance_specification {
        crate::protocol_serde::shape_instance_specification::ser_instance_specification(
            scope_3, var_4,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("OutpostArn");
    if let Some(var_6) = &input.outpost_arn {
        scope_5.string(var_6);
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
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("DryRun");
    if let Some(var_13) = &input.dry_run {
        scope_12.boolean(*var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("CopyTagsFromSource");
    if let Some(var_15) = &input.copy_tags_from_source {
        scope_14.string(var_15.as_str());
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_snapshot_attribute_input_input(
    input: &crate::operation::modify_snapshot_attribute::ModifySnapshotAttributeInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "ModifySnapshotAttribute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Attribute");
    if let Some(var_2) = &input.attribute {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CreateVolumePermission");
    if let Some(var_4) = &input.create_volume_permission {
        crate::protocol_serde::shape_create_volume_permission_modifications::ser_create_volume_permission_modifications(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("UserGroup");
    if let Some(var_6) = &input.group_names {
        let mut list_8 = scope_5.start_list(true, Some("GroupName"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("OperationType");
    if let Some(var_11) = &input.operation_type {
        scope_10.string(var_11.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("SnapshotId");
    if let Some(var_13) = &input.snapshot_id {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("UserId");
    if let Some(var_15) = &input.user_ids {
        let mut list_17 = scope_14.start_list(true, Some("UserId"));
        for item_16 in var_15 {
            #[allow(unused_mut)]
            let mut entry_18 = list_17.entry();
            entry_18.string(item_16);
        }
        list_17.finish();
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("DryRun");
    if let Some(var_20) = &input.dry_run {
        scope_19.boolean(*var_20);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

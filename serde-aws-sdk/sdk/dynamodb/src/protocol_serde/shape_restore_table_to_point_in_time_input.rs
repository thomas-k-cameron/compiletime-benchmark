// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_table_to_point_in_time_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::restore_table_to_point_in_time::RestoreTableToPointInTimeInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_table_arn {
        object.key("SourceTableArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.source_table_name {
        object.key("SourceTableName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_table_name {
        object.key("TargetTableName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.use_latest_restorable_time {
        object.key("UseLatestRestorableTime").boolean(*var_4);
    }
    if let Some(var_5) = &input.restore_date_time {
        object
            .key("RestoreDateTime")
            .date_time(var_5, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.billing_mode_override {
        object.key("BillingModeOverride").string(var_6.as_str());
    }
    if let Some(var_7) = &input.global_secondary_index_override {
        let mut array_8 = object.key("GlobalSecondaryIndexOverride").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_global_secondary_index::ser_global_secondary_index(
                    &mut object_10,
                    item_9,
                )?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.local_secondary_index_override {
        let mut array_12 = object.key("LocalSecondaryIndexOverride").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_local_secondary_index::ser_local_secondary_index(
                    &mut object_14,
                    item_13,
                )?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.provisioned_throughput_override {
        #[allow(unused_mut)]
        let mut object_16 = object.key("ProvisionedThroughputOverride").start_object();
        crate::protocol_serde::shape_provisioned_throughput::ser_provisioned_throughput(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    if let Some(var_17) = &input.sse_specification_override {
        #[allow(unused_mut)]
        let mut object_18 = object.key("SSESpecificationOverride").start_object();
        crate::protocol_serde::shape_sse_specification::ser_sse_specification(
            &mut object_18,
            var_17,
        )?;
        object_18.finish();
    }
    Ok(())
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_flow_logs_input_input(
    input: &crate::operation::create_flow_logs::CreateFlowLogsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateFlowLogs", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClientToken");
    if let Some(var_4) = &input.client_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DeliverLogsPermissionArn");
    if let Some(var_6) = &input.deliver_logs_permission_arn {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DeliverCrossAccountRole");
    if let Some(var_8) = &input.deliver_cross_account_role {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("LogGroupName");
    if let Some(var_10) = &input.log_group_name {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("ResourceId");
    if let Some(var_12) = &input.resource_ids {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            entry_15.string(item_13);
        }
        list_14.finish();
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("ResourceType");
    if let Some(var_17) = &input.resource_type {
        scope_16.string(var_17.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("TrafficType");
    if let Some(var_19) = &input.traffic_type {
        scope_18.string(var_19.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("LogDestinationType");
    if let Some(var_21) = &input.log_destination_type {
        scope_20.string(var_21.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("LogDestination");
    if let Some(var_23) = &input.log_destination {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("LogFormat");
    if let Some(var_25) = &input.log_format {
        scope_24.string(var_25);
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("TagSpecification");
    if let Some(var_27) = &input.tag_specifications {
        let mut list_29 = scope_26.start_list(true, Some("item"));
        for item_28 in var_27 {
            #[allow(unused_mut)]
            let mut entry_30 = list_29.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(
                entry_30, item_28,
            )?;
        }
        list_29.finish();
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("MaxAggregationInterval");
    if let Some(var_32) = &input.max_aggregation_interval {
        scope_31.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("DestinationOptions");
    if let Some(var_34) = &input.destination_options {
        crate::protocol_serde::shape_destination_options_request::ser_destination_options_request(
            scope_33, var_34,
        )?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

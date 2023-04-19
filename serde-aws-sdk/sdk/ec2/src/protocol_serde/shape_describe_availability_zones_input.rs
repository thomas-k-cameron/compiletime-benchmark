// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_availability_zones_input_input(
    input: &crate::operation::describe_availability_zones::DescribeAvailabilityZonesInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DescribeAvailabilityZones", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Filter");
    if let Some(var_2) = &input.filters {
        let mut list_4 = scope_1.start_list(true, Some("Filter"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("ZoneName");
    if let Some(var_7) = &input.zone_names {
        let mut list_9 = scope_6.start_list(true, Some("ZoneName"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("ZoneId");
    if let Some(var_12) = &input.zone_ids {
        let mut list_14 = scope_11.start_list(true, Some("ZoneId"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            entry_15.string(item_13);
        }
        list_14.finish();
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("AllAvailabilityZones");
    if let Some(var_17) = &input.all_availability_zones {
        scope_16.boolean(*var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("DryRun");
    if let Some(var_19) = &input.dry_run {
        scope_18.boolean(*var_19);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

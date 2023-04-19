// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_instance_event_windows_input_input(
    input: &crate::operation::describe_instance_event_windows::DescribeInstanceEventWindowsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DescribeInstanceEventWindows", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceEventWindowId");
    if let Some(var_4) = &input.instance_event_window_ids {
        let mut list_6 = scope_3.start_list(true, Some("InstanceEventWindowId"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Filter");
    if let Some(var_9) = &input.filters {
        let mut list_11 = scope_8.start_list(true, Some("Filter"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_12, item_10)?;
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("MaxResults");
    if let Some(var_14) = &input.max_results {
        scope_13.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("NextToken");
    if let Some(var_16) = &input.next_token {
        scope_15.string(var_16);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

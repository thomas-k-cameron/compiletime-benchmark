// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_store_image_tasks_input_input(
    input: &crate::operation::describe_store_image_tasks::DescribeStoreImageTasksInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DescribeStoreImageTasks", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ImageId");
    if let Some(var_2) = &input.image_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DryRun");
    if let Some(var_7) = &input.dry_run {
        scope_6.boolean(*var_7);
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
    let mut scope_13 = writer.prefix("NextToken");
    if let Some(var_14) = &input.next_token {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("MaxResults");
    if let Some(var_16) = &input.max_results {
        scope_15.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

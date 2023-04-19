// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_ipv6_pools_input_input(
    input: &crate::operation::describe_ipv6_pools::DescribeIpv6PoolsInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer =
        aws_smithy_query::QueryWriter::new(&mut out, "DescribeIpv6Pools", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PoolId");
    if let Some(var_2) = &input.pool_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("NextToken");
    if let Some(var_7) = &input.next_token {
        scope_6.string(var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("MaxResults");
    if let Some(var_9) = &input.max_results {
        scope_8.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("DryRun");
    if let Some(var_11) = &input.dry_run {
        scope_10.boolean(*var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("Filter");
    if let Some(var_13) = &input.filters {
        let mut list_15 = scope_12.start_list(true, Some("Filter"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_16, item_14)?;
        }
        list_15.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

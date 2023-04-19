// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_egress_only_internet_gateways_input_input(
    input: &crate::operation::describe_egress_only_internet_gateways::DescribeEgressOnlyInternetGatewaysInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "DescribeEgressOnlyInternetGateways",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EgressOnlyInternetGatewayId");
    if let Some(var_4) = &input.egress_only_internet_gateway_ids {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
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
    let mut scope_10 = writer.prefix("NextToken");
    if let Some(var_11) = &input.next_token {
        scope_10.string(var_11);
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

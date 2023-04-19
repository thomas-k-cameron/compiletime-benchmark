// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_vpc_classic_link_dns_support_input_input(
    input: &crate::operation::describe_vpc_classic_link_dns_support::DescribeVpcClassicLinkDnsSupportInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(
        &mut out,
        "DescribeVpcClassicLinkDnsSupport",
        "2016-11-15",
    );
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("MaxResults");
    if let Some(var_2) = &input.max_results {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NextToken");
    if let Some(var_4) = &input.next_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("VpcIds");
    if let Some(var_6) = &input.vpc_ids {
        let mut list_8 = scope_5.start_list(true, Some("VpcId"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

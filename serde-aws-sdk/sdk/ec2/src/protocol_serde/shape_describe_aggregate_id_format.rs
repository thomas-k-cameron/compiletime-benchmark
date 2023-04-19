// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_aggregate_id_format_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatOutput,
    crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_aggregate_id_format_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatOutput,
    crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_aggregate_id_format::de_describe_aggregate_id_format(response.body().as_ref(), output).map_err(crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_aggregate_id_format(inp: &[u8], mut builder: crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatOutputBuilder) -> Result<crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeAggregateIdFormatResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeAggregateIdFormatResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("useLongIdsAggregated") /* UseLongIdsAggregated com.amazonaws.ec2.synthetic#DescribeAggregateIdFormatOutput$UseLongIdsAggregated */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_use_long_ids_aggregated(var_1);
            }
            ,
            s if s.matches("statusSet") /* Statuses com.amazonaws.ec2.synthetic#DescribeAggregateIdFormatOutput$Statuses */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_id_format_list::de_id_format_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_statuses(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_conversion_tasks_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_conversion_tasks::DescribeConversionTasksOutput,
    crate::operation::describe_conversion_tasks::DescribeConversionTasksError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_conversion_tasks::DescribeConversionTasksError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_conversion_tasks::DescribeConversionTasksError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_conversion_tasks_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_conversion_tasks::DescribeConversionTasksOutput,
    crate::operation::describe_conversion_tasks::DescribeConversionTasksError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_conversion_tasks::de_describe_conversion_tasks(response.body().as_ref(), output).map_err(crate::operation::describe_conversion_tasks::DescribeConversionTasksError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_conversion_tasks(
    inp: &[u8],
    mut builder: crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksOutputBuilder,
) -> Result<
    crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeConversionTasksResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeConversionTasksResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("conversionTasks") /* ConversionTasks com.amazonaws.ec2.synthetic#DescribeConversionTasksOutput$ConversionTasks */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_describe_conversion_task_list::de_describe_conversion_task_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_conversion_tasks(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

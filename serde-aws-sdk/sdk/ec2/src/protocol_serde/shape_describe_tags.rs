// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_tags_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_tags::DescribeTagsOutput,
    crate::operation::describe_tags::DescribeTagsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::describe_tags::DescribeTagsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_tags::DescribeTagsError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_tags_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_tags::DescribeTagsOutput,
    crate::operation::describe_tags::DescribeTagsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::describe_tags::builders::DescribeTagsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_tags::de_describe_tags(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::describe_tags::DescribeTagsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_tags(
    inp: &[u8],
    mut builder: crate::operation::describe_tags::builders::DescribeTagsOutputBuilder,
) -> Result<
    crate::operation::describe_tags::builders::DescribeTagsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeTagsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeTagsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeTagsOutput$NextToken */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_1);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2.synthetic#DescribeTagsOutput$Tags */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_tag_description_list::de_tag_description_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

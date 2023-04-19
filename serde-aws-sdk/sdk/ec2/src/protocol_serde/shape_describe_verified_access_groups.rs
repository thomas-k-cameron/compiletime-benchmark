// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_verified_access_groups_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput,
    crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_verified_access_groups_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsOutput,
    crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_verified_access_groups::builders::DescribeVerifiedAccessGroupsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_verified_access_groups::de_describe_verified_access_groups(response.body().as_ref(), output).map_err(crate::operation::describe_verified_access_groups::DescribeVerifiedAccessGroupsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_verified_access_groups(inp: &[u8], mut builder: crate::operation::describe_verified_access_groups::builders::DescribeVerifiedAccessGroupsOutputBuilder) -> Result<crate::operation::describe_verified_access_groups::builders::DescribeVerifiedAccessGroupsOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVerifiedAccessGroupsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVerifiedAccessGroupsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("verifiedAccessGroupSet") /* VerifiedAccessGroups com.amazonaws.ec2.synthetic#DescribeVerifiedAccessGroupsOutput$VerifiedAccessGroups */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_verified_access_group_list::de_verified_access_group_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_verified_access_groups(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeVerifiedAccessGroupsOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

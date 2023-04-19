// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpc_classic_link_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
    crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpc_classic_link_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkOutput,
    crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_vpc_classic_link::builders::DescribeVpcClassicLinkOutputBuilder::default();
        let _ = response;
        output =
            crate::protocol_serde::shape_describe_vpc_classic_link::de_describe_vpc_classic_link(
                response.body().as_ref(),
                output,
            )
            .map_err(
                crate::operation::describe_vpc_classic_link::DescribeVpcClassicLinkError::unhandled,
            )?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_vpc_classic_link(
    inp: &[u8],
    mut builder: crate::operation::describe_vpc_classic_link::builders::DescribeVpcClassicLinkOutputBuilder,
) -> Result<
    crate::operation::describe_vpc_classic_link::builders::DescribeVpcClassicLinkOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVpcClassicLinkResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVpcClassicLinkResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("vpcSet") /* Vpcs com.amazonaws.ec2.synthetic#DescribeVpcClassicLinkOutput$Vpcs */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_vpc_classic_link_list::de_vpc_classic_link_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpcs(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
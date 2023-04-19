// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_checker_ip_ranges_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput,
    crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_checker_ip_ranges_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_checker_ip_ranges::GetCheckerIpRangesOutput,
    crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_checker_ip_ranges::de_get_checker_ip_ranges(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_checker_ip_ranges::GetCheckerIpRangesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_checker_ip_ranges(
    inp: &[u8],
    mut builder: crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesOutputBuilder,
) -> Result<
    crate::operation::get_checker_ip_ranges::builders::GetCheckerIpRangesOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetCheckerIpRangesResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetCheckerIpRangesResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CheckerIpRanges") /* CheckerIpRanges com.amazonaws.route53.synthetic#GetCheckerIpRangesOutput$CheckerIpRanges */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_checker_ip_ranges::de_checker_ip_ranges(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_checker_ip_ranges(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_multi_region_access_point_operation_headers(
    input: &crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationInput,
    mut builder: http::request::Builder,
) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        if !formatted_2.is_empty() {
            let header_value = formatted_2;
            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                aws_smithy_http::operation::error::BuildError::invalid_field(
                    "account_id",
                    format!(
                        "`{}` cannot be used as a header value: {}",
                        &header_value, err
                    ),
                )
            })?;
            builder = builder.header("x-amz-account-id", header_value);
        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_multi_region_access_point_operation_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationOutput, crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_multi_region_access_point_operation_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationOutput, crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_multi_region_access_point_operation::builders::DescribeMultiRegionAccessPointOperationOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_multi_region_access_point_operation::de_describe_multi_region_access_point_operation(response.body().as_ref(), output).map_err(crate::operation::describe_multi_region_access_point_operation::DescribeMultiRegionAccessPointOperationError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_multi_region_access_point_operation(inp: &[u8], mut builder: crate::operation::describe_multi_region_access_point_operation::builders::DescribeMultiRegionAccessPointOperationOutputBuilder) -> Result<crate::operation::describe_multi_region_access_point_operation::builders::DescribeMultiRegionAccessPointOperationOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("DescribeMultiRegionAccessPointOperationResult") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected DescribeMultiRegionAccessPointOperationResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AsyncOperation") /* AsyncOperation com.amazonaws.s3control.synthetic#DescribeMultiRegionAccessPointOperationOutput$AsyncOperation */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_async_operation::de_async_operation(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_async_operation(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
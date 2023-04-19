// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_volume_attribute_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_volume_attribute::DescribeVolumeAttributeOutput,
    crate::operation::describe_volume_attribute::DescribeVolumeAttributeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(
        crate::operation::describe_volume_attribute::DescribeVolumeAttributeError::unhandled,
    )?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::describe_volume_attribute::DescribeVolumeAttributeError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_volume_attribute_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::describe_volume_attribute::DescribeVolumeAttributeOutput,
    crate::operation::describe_volume_attribute::DescribeVolumeAttributeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_volume_attribute::builders::DescribeVolumeAttributeOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_volume_attribute::de_describe_volume_attribute(response.body().as_ref(), output).map_err(crate::operation::describe_volume_attribute::DescribeVolumeAttributeError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_volume_attribute(
    inp: &[u8],
    mut builder: crate::operation::describe_volume_attribute::builders::DescribeVolumeAttributeOutputBuilder,
) -> Result<
    crate::operation::describe_volume_attribute::builders::DescribeVolumeAttributeOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVolumeAttributeResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVolumeAttributeResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("autoEnableIO") /* AutoEnableIO com.amazonaws.ec2.synthetic#DescribeVolumeAttributeOutput$AutoEnableIO */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_attribute_boolean_value::de_attribute_boolean_value(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_auto_enable_io(var_1);
            }
            ,
            s if s.matches("productCodes") /* ProductCodes com.amazonaws.ec2.synthetic#DescribeVolumeAttributeOutput$ProductCodes */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_product_code_list::de_product_code_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_product_codes(var_2);
            }
            ,
            s if s.matches("volumeId") /* VolumeId com.amazonaws.ec2.synthetic#DescribeVolumeAttributeOutput$VolumeId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_id(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

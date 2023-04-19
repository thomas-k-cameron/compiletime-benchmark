// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_instance_uefi_data_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_instance_uefi_data::GetInstanceUefiDataOutput,
    crate::operation::get_instance_uefi_data::GetInstanceUefiDataError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_instance_uefi_data::GetInstanceUefiDataError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::get_instance_uefi_data::GetInstanceUefiDataError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_instance_uefi_data_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_instance_uefi_data::GetInstanceUefiDataOutput,
    crate::operation::get_instance_uefi_data::GetInstanceUefiDataError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_instance_uefi_data::de_get_instance_uefi_data(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_instance_uefi_data::GetInstanceUefiDataError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_instance_uefi_data(
    inp: &[u8],
    mut builder: crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataOutputBuilder,
) -> Result<
    crate::operation::get_instance_uefi_data::builders::GetInstanceUefiDataOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetInstanceUefiDataResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetInstanceUefiDataResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2.synthetic#GetInstanceUefiDataOutput$InstanceId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_1);
            }
            ,
            s if s.matches("uefiData") /* UefiData com.amazonaws.ec2.synthetic#GetInstanceUefiDataOutput$UefiData */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_uefi_data(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

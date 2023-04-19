// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_instance_event_window_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
    crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_instance_event_window_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
    crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disassociate_instance_event_window::builders::DisassociateInstanceEventWindowOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_disassociate_instance_event_window::de_disassociate_instance_event_window(response.body().as_ref(), output).map_err(crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_disassociate_instance_event_window(inp: &[u8], mut builder: crate::operation::disassociate_instance_event_window::builders::DisassociateInstanceEventWindowOutputBuilder) -> Result<crate::operation::disassociate_instance_event_window::builders::DisassociateInstanceEventWindowOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DisassociateInstanceEventWindowResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DisassociateInstanceEventWindowResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceEventWindow") /* InstanceEventWindow com.amazonaws.ec2.synthetic#DisassociateInstanceEventWindowOutput$InstanceEventWindow */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_event_window::de_instance_event_window(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_event_window(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
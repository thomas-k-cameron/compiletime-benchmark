// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_change_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_change::GetChangeOutput,
    crate::operation::get_change::GetChangeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::get_change::GetChangeError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_change::GetChangeError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::get_change::GetChangeError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::operation::get_change::GetChangeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchChange" => crate::operation::get_change::GetChangeError::NoSuchChange({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchChangeBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_no_such_change::de_no_such_change_xml_err(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::operation::get_change::GetChangeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_change::GetChangeError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_change_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::get_change::GetChangeOutput,
    crate::operation::get_change::GetChangeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_change::builders::GetChangeOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_change::de_get_change(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::get_change::GetChangeError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_change(
    inp: &[u8],
    mut builder: crate::operation::get_change::builders::GetChangeOutputBuilder,
) -> Result<
    crate::operation::get_change::builders::GetChangeOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetChangeResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetChangeResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ChangeInfo") /* ChangeInfo com.amazonaws.route53.synthetic#GetChangeOutput$ChangeInfo */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_change_info::de_change_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_change_info(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

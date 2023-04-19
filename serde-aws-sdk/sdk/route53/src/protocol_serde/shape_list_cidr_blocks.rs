// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_cidr_blocks_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_cidr_blocks::ListCidrBlocksOutput,
    crate::operation::list_cidr_blocks::ListCidrBlocksError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::list_cidr_blocks::ListCidrBlocksError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                let _ = response;
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(
                    response.body().as_ref(),
                    output,
                )
                .map_err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchCidrCollectionException" => {
            crate::operation::list_cidr_blocks::ListCidrBlocksError::NoSuchCidrCollectionException(
                {
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchCidrCollectionExceptionBuilder::default();
                        let _ = response;
                        output = crate::protocol_serde::shape_no_such_cidr_collection_exception::de_no_such_cidr_collection_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled)?;
                        let output = output.meta(generic);
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                },
            )
        }
        "NoSuchCidrLocationException" => {
            crate::operation::list_cidr_blocks::ListCidrBlocksError::NoSuchCidrLocationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::types::error::builders::NoSuchCidrLocationExceptionBuilder::default(
                        );
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_cidr_location_exception::de_no_such_cidr_location_exception_xml_err(response.body().as_ref(), output).map_err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::list_cidr_blocks::ListCidrBlocksError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_cidr_blocks_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::list_cidr_blocks::ListCidrBlocksOutput,
    crate::operation::list_cidr_blocks::ListCidrBlocksError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::list_cidr_blocks::builders::ListCidrBlocksOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_cidr_blocks::de_list_cidr_blocks(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::list_cidr_blocks::ListCidrBlocksError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_list_cidr_blocks(
    inp: &[u8],
    mut builder: crate::operation::list_cidr_blocks::builders::ListCidrBlocksOutputBuilder,
) -> Result<
    crate::operation::list_cidr_blocks::builders::ListCidrBlocksOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListCidrBlocksResponse") {
        return Err(
                                aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected ListCidrBlocksResponse but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            );
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CidrBlocks") /* CidrBlocks com.amazonaws.route53.synthetic#ListCidrBlocksOutput$CidrBlocks */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cidr_block_summaries::de_cidr_block_summaries(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cidr_blocks(var_1);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.route53.synthetic#ListCidrBlocksOutput$NextToken */ =>  {
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

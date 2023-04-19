// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_hosts_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_hosts::ModifyHostsOutput,
    crate::operation::modify_hosts::ModifyHostsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response)
        .map_err(crate::operation::modify_hosts::ModifyHostsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::modify_hosts::ModifyHostsError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_hosts_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::modify_hosts::ModifyHostsOutput,
    crate::operation::modify_hosts::ModifyHostsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::modify_hosts::builders::ModifyHostsOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_hosts::de_modify_hosts(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::operation::modify_hosts::ModifyHostsError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_hosts(
    inp: &[u8],
    mut builder: crate::operation::modify_hosts::builders::ModifyHostsOutputBuilder,
) -> Result<
    crate::operation::modify_hosts::builders::ModifyHostsOutputBuilder,
    aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyHostsResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyHostsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("successful") /* Successful com.amazonaws.ec2.synthetic#ModifyHostsOutput$Successful */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_response_host_id_list::de_response_host_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_successful(var_1);
            }
            ,
            s if s.matches("unsuccessful") /* Unsuccessful com.amazonaws.ec2.synthetic#ModifyHostsOutput$Unsuccessful */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_unsuccessful_item_list::de_unsuccessful_item_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unsuccessful(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

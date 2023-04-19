// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_ipam_resource_discovery_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryOutput,
    crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(
        crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError::generic(
            generic,
        ),
    )
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_ipam_resource_discovery_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryOutput,
    crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_ipam_resource_discovery::de_delete_ipam_resource_discovery(response.body().as_ref(), output).map_err(crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_ipam_resource_discovery(inp: &[u8], mut builder: crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryOutputBuilder) -> Result<crate::operation::delete_ipam_resource_discovery::builders::DeleteIpamResourceDiscoveryOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteIpamResourceDiscoveryResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteIpamResourceDiscoveryResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamResourceDiscovery") /* IpamResourceDiscovery com.amazonaws.ec2.synthetic#DeleteIpamResourceDiscoveryOutput$IpamResourceDiscovery */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ipam_resource_discovery::de_ipam_resource_discovery(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_traffic_mirror_filter_network_services_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput, crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError>{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_traffic_mirror_filter_network_services_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput, crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError>{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_traffic_mirror_filter_network_services::de_modify_traffic_mirror_filter_network_services(response.body().as_ref(), output).map_err(crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_traffic_mirror_filter_network_services(inp: &[u8], mut builder: crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesOutputBuilder) -> Result<crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyTrafficMirrorFilterNetworkServicesResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyTrafficMirrorFilterNetworkServicesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("trafficMirrorFilter") /* TrafficMirrorFilter com.amazonaws.ec2.synthetic#ModifyTrafficMirrorFilterNetworkServicesOutput$TrafficMirrorFilter */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_traffic_mirror_filter::de_traffic_mirror_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_traffic_mirror_filter(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
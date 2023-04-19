// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deprovision_public_ipv4_pool_cidr_http_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrOutput,
    crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deprovision_public_ipv4_pool_cidr_http_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrOutput,
    crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrOutputBuilder::default();
        let _ = response;
        output = crate::protocol_serde::shape_deprovision_public_ipv4_pool_cidr::de_deprovision_public_ipv4_pool_cidr(response.body().as_ref(), output).map_err(crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrError::unhandled)?;
        output._set_request_id(
            aws_http::request_id::RequestId::request_id(response).map(str::to_string),
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_deprovision_public_ipv4_pool_cidr(inp: &[u8], mut builder: crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrOutputBuilder) -> Result<crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrOutputBuilder, aws_smithy_xml::decode::XmlDecodeError>{
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeprovisionPublicIpv4PoolCidrResponse")) {
        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeprovisionPublicIpv4PoolCidrResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("poolId") /* PoolId com.amazonaws.ec2.synthetic#DeprovisionPublicIpv4PoolCidrOutput$PoolId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_id(var_1);
            }
            ,
            s if s.matches("deprovisionedAddressSet") /* DeprovisionedAddresses com.amazonaws.ec2.synthetic#DeprovisionPublicIpv4PoolCidrOutput$DeprovisionedAddresses */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_deprovisioned_address_set::de_deprovisioned_address_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_deprovisioned_addresses(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

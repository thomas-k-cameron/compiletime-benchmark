// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_inventory_configuration(
    decoder: &mut aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InventoryConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InventoryConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Destination") /* Destination com.amazonaws.s3#InventoryConfiguration$Destination */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_inventory_destination::de_inventory_destination(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination(var_1);
            }
            ,
            s if s.matches("IsEnabled") /* IsEnabled com.amazonaws.s3#InventoryConfiguration$IsEnabled */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.s3#IsEnabled`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_enabled(var_2);
            }
            ,
            s if s.matches("Filter") /* Filter com.amazonaws.s3#InventoryConfiguration$Filter */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_inventory_filter::de_inventory_filter(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_filter(var_3);
            }
            ,
            s if s.matches("Id") /* Id com.amazonaws.s3#InventoryConfiguration$Id */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_4);
            }
            ,
            s if s.matches("IncludedObjectVersions") /* IncludedObjectVersions com.amazonaws.s3#InventoryConfiguration$IncludedObjectVersions */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::InventoryIncludedObjectVersions, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InventoryIncludedObjectVersions::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_included_object_versions(var_5);
            }
            ,
            s if s.matches("OptionalFields") /* OptionalFields com.amazonaws.s3#InventoryConfiguration$OptionalFields */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_inventory_optional_fields::de_inventory_optional_fields(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_optional_fields(var_6);
            }
            ,
            s if s.matches("Schedule") /* Schedule com.amazonaws.s3#InventoryConfiguration$Schedule */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_inventory_schedule::de_inventory_schedule(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_schedule(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

pub fn ser_inventory_configuration(
    input: &crate::types::InventoryConfiguration,
    writer: aws_smithy_xml::encode::ElWriter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_8) = &input.destination {
        let inner_writer = scope.start_el("Destination");
        crate::protocol_serde::shape_inventory_destination::ser_inventory_destination(
            var_8,
            inner_writer,
        )?
    }
    {
        let mut inner_writer = scope.start_el("IsEnabled").finish();
        inner_writer.data(aws_smithy_types::primitive::Encoder::from(input.is_enabled).encode());
    }
    if let Some(var_9) = &input.filter {
        let inner_writer = scope.start_el("Filter");
        crate::protocol_serde::shape_inventory_filter::ser_inventory_filter(var_9, inner_writer)?
    }
    if let Some(var_10) = &input.id {
        let mut inner_writer = scope.start_el("Id").finish();
        inner_writer.data(var_10.as_str());
    }
    if let Some(var_11) = &input.included_object_versions {
        let mut inner_writer = scope.start_el("IncludedObjectVersions").finish();
        inner_writer.data(var_11.as_str());
    }
    if let Some(var_12) = &input.optional_fields {
        let mut inner_writer = scope.start_el("OptionalFields").finish();
        for list_item_13 in var_12 {
            {
                let mut inner_writer = inner_writer.start_el("Field").finish();
                inner_writer.data(list_item_13.as_str());
            }
        }
    }
    if let Some(var_14) = &input.schedule {
        let inner_writer = scope.start_el("Schedule");
        crate::protocol_serde::shape_inventory_schedule::ser_inventory_schedule(
            var_14,
            inner_writer,
        )?
    }
    scope.finish();
    Ok(())
}
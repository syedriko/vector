// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_phone_number_information(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::PhoneNumberInformation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PhoneNumberInformation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CreatedAt") /* CreatedAt com.amazonaws.sns#PhoneNumberInformation$CreatedAt */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.sns#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_created_at(var_1);
            }
            ,
            s if s.matches("PhoneNumber") /* PhoneNumber com.amazonaws.sns#PhoneNumberInformation$PhoneNumber */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_phone_number(var_2);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.sns#PhoneNumberInformation$Status */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            s if s.matches("Iso2CountryCode") /* Iso2CountryCode com.amazonaws.sns#PhoneNumberInformation$Iso2CountryCode */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_iso2_country_code(var_4);
            }
            ,
            s if s.matches("RouteType") /* RouteType com.amazonaws.sns#PhoneNumberInformation$RouteType */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::RouteType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RouteType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_route_type(var_5);
            }
            ,
            s if s.matches("NumberCapabilities") /* NumberCapabilities com.amazonaws.sns#PhoneNumberInformation$NumberCapabilities */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_number_capability_list::de_number_capability_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_number_capabilities(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

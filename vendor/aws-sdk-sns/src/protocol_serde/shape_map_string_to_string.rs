// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_map_string_to_string(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<::std::collections::HashMap<::std::string::String, ::std::string::String>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = ::std::collections::HashMap::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("entry") => {
                crate::protocol_serde::shape_map_string_to_string::de_map_string_to_string_entry(&mut tag, &mut out)?;
            }
            _ => {}
        }
    }
    Ok(out)
}

pub fn de_map_string_to_string_entry(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
    out: &mut ::std::collections::HashMap<::std::string::String, ::std::string::String>,
) -> Result<(), ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut k: Option<::std::string::String> = None;
    let mut v: Option<::std::string::String> = None;
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("key") /* key com.amazonaws.sns#MapStringToString$key */ =>  {
                k = Some(
                    Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                )
            }
            ,
            s if s.matches("value") /* value com.amazonaws.sns#MapStringToString$value */ =>  {
                v = Some(
                    Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        .into()
                    )
                    ?
                )
            }
            ,
            _ => {}
        }
    }
    let k = k.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing key map entry"))?;
    let v = v.ok_or_else(|| ::aws_smithy_xml::decode::XmlDecodeError::custom("missing value map entry"))?;
    out.insert(k, v);
    Ok(())
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_open_x_json_ser_de(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OpenXJsonSerDe,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.convert_dots_in_json_keys_to_underscores {
        object.key("ConvertDotsInJsonKeysToUnderscores").boolean(*var_1);
    }
    if let Some(var_2) = &input.case_insensitive {
        object.key("CaseInsensitive").boolean(*var_2);
    }
    if let Some(var_3) = &input.column_to_json_key_mappings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ColumnToJsonKeyMappings").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_open_x_json_ser_de<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::OpenXJsonSerDe>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::OpenXJsonSerDeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ConvertDotsInJsonKeysToUnderscores" => {
                            builder = builder.set_convert_dots_in_json_keys_to_underscores(
                                ::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?,
                            );
                        }
                        "CaseInsensitive" => {
                            builder = builder.set_case_insensitive(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ColumnToJsonKeyMappings" => {
                            builder = builder.set_column_to_json_key_mappings(
                                crate::protocol_serde::shape_column_to_json_key_mappings::de_column_to_json_key_mappings(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

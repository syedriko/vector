// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_orc_ser_de(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OrcSerDe,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stripe_size_bytes {
        object.key("StripeSizeBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.block_size_bytes {
        object.key("BlockSizeBytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.row_index_stride {
        object.key("RowIndexStride").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.enable_padding {
        object.key("EnablePadding").boolean(*var_4);
    }
    if let Some(var_5) = &input.padding_tolerance {
        object.key("PaddingTolerance").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.compression {
        object.key("Compression").string(var_6.as_str());
    }
    if let Some(var_7) = &input.bloom_filter_columns {
        let mut array_8 = object.key("BloomFilterColumns").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.bloom_filter_false_positive_probability {
        object.key("BloomFilterFalsePositiveProbability").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_10).into()),
        );
    }
    if let Some(var_11) = &input.dictionary_key_threshold {
        object.key("DictionaryKeyThreshold").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.format_version {
        object.key("FormatVersion").string(var_12.as_str());
    }
    Ok(())
}

pub(crate) fn de_orc_ser_de<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::OrcSerDe>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::OrcSerDeBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "StripeSizeBytes" => {
                            builder = builder.set_stripe_size_bytes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "BlockSizeBytes" => {
                            builder = builder.set_block_size_bytes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "RowIndexStride" => {
                            builder = builder.set_row_index_stride(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "EnablePadding" => {
                            builder = builder.set_enable_padding(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "PaddingTolerance" => {
                            builder = builder.set_padding_tolerance(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "Compression" => {
                            builder = builder.set_compression(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::OrcCompression::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "BloomFilterColumns" => {
                            builder = builder.set_bloom_filter_columns(
                                    crate::protocol_serde::shape_list_of_non_empty_strings_without_whitespace::de_list_of_non_empty_strings_without_whitespace(tokens)?
                                );
                        }
                        "BloomFilterFalsePositiveProbability" => {
                            builder = builder.set_bloom_filter_false_positive_probability(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "DictionaryKeyThreshold" => {
                            builder = builder.set_dictionary_key_threshold(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "FormatVersion" => {
                            builder = builder.set_format_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::OrcFormatVersion::from(u.as_ref())))
                                    .transpose()?,
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

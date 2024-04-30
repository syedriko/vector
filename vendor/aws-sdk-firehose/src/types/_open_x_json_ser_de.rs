// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the native Hive / HCatalog JsonSerDe.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OpenXJsonSerDe {
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p>
    /// <p>The default is <code>false</code>.</p>
    pub convert_dots_in_json_keys_to_underscores: ::std::option::Option<bool>,
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    pub case_insensitive: ::std::option::Option<bool>,
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    pub column_to_json_key_mappings: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl OpenXJsonSerDe {
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn convert_dots_in_json_keys_to_underscores(&self) -> ::std::option::Option<bool> {
        self.convert_dots_in_json_keys_to_underscores
    }
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    pub fn case_insensitive(&self) -> ::std::option::Option<bool> {
        self.case_insensitive
    }
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    pub fn column_to_json_key_mappings(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.column_to_json_key_mappings.as_ref()
    }
}
impl OpenXJsonSerDe {
    /// Creates a new builder-style object to manufacture [`OpenXJsonSerDe`](crate::types::OpenXJsonSerDe).
    pub fn builder() -> crate::types::builders::OpenXJsonSerDeBuilder {
        crate::types::builders::OpenXJsonSerDeBuilder::default()
    }
}

/// A builder for [`OpenXJsonSerDe`](crate::types::OpenXJsonSerDe).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct OpenXJsonSerDeBuilder {
    pub(crate) convert_dots_in_json_keys_to_underscores: ::std::option::Option<bool>,
    pub(crate) case_insensitive: ::std::option::Option<bool>,
    pub(crate) column_to_json_key_mappings: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl OpenXJsonSerDeBuilder {
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn convert_dots_in_json_keys_to_underscores(mut self, input: bool) -> Self {
        self.convert_dots_in_json_keys_to_underscores = ::std::option::Option::Some(input);
        self
    }
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn set_convert_dots_in_json_keys_to_underscores(mut self, input: ::std::option::Option<bool>) -> Self {
        self.convert_dots_in_json_keys_to_underscores = input;
        self
    }
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p>
    /// <p>The default is <code>false</code>.</p>
    pub fn get_convert_dots_in_json_keys_to_underscores(&self) -> &::std::option::Option<bool> {
        &self.convert_dots_in_json_keys_to_underscores
    }
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    pub fn case_insensitive(mut self, input: bool) -> Self {
        self.case_insensitive = ::std::option::Option::Some(input);
        self
    }
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    pub fn set_case_insensitive(mut self, input: ::std::option::Option<bool>) -> Self {
        self.case_insensitive = input;
        self
    }
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    pub fn get_case_insensitive(&self) -> &::std::option::Option<bool> {
        &self.case_insensitive
    }
    /// Adds a key-value pair to `column_to_json_key_mappings`.
    ///
    /// To override the contents of this collection use [`set_column_to_json_key_mappings`](Self::set_column_to_json_key_mappings).
    ///
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    pub fn column_to_json_key_mappings(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.column_to_json_key_mappings.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.column_to_json_key_mappings = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    pub fn set_column_to_json_key_mappings(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.column_to_json_key_mappings = input;
        self
    }
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    pub fn get_column_to_json_key_mappings(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.column_to_json_key_mappings
    }
    /// Consumes the builder and constructs a [`OpenXJsonSerDe`](crate::types::OpenXJsonSerDe).
    pub fn build(self) -> crate::types::OpenXJsonSerDe {
        crate::types::OpenXJsonSerDe {
            convert_dots_in_json_keys_to_underscores: self.convert_dots_in_json_keys_to_underscores,
            case_insensitive: self.case_insensitive,
            column_to_json_key_mappings: self.column_to_json_key_mappings,
        }
    }
}

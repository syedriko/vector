// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output for <code>PutRecord</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutRecordOutput {
    /// <p>The shard ID of the shard where the data record was placed.</p>
    pub shard_id: ::std::string::String,
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    pub sequence_number: ::std::string::String,
    /// <p>The encryption type to use on the record. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    _request_id: Option<String>,
}
impl PutRecordOutput {
    /// <p>The shard ID of the shard where the data record was placed.</p>
    pub fn shard_id(&self) -> &str {
        use std::ops::Deref;
        self.shard_id.deref()
    }
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    pub fn sequence_number(&self) -> &str {
        use std::ops::Deref;
        self.sequence_number.deref()
    }
    /// <p>The encryption type to use on the record. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn encryption_type(&self) -> ::std::option::Option<&crate::types::EncryptionType> {
        self.encryption_type.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for PutRecordOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl PutRecordOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
    pub fn builder() -> crate::operation::put_record::builders::PutRecordOutputBuilder {
        crate::operation::put_record::builders::PutRecordOutputBuilder::default()
    }
}

/// A builder for [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutRecordOutputBuilder {
    pub(crate) shard_id: ::std::option::Option<::std::string::String>,
    pub(crate) sequence_number: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    _request_id: Option<String>,
}
impl PutRecordOutputBuilder {
    /// <p>The shard ID of the shard where the data record was placed.</p>
    /// This field is required.
    pub fn shard_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.shard_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The shard ID of the shard where the data record was placed.</p>
    pub fn set_shard_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.shard_id = input;
        self
    }
    /// <p>The shard ID of the shard where the data record was placed.</p>
    pub fn get_shard_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.shard_id
    }
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    /// This field is required.
    pub fn sequence_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.sequence_number = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    pub fn set_sequence_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.sequence_number = input;
        self
    }
    /// <p>The sequence number identifier that was assigned to the put data record. The sequence number for the record is unique across all records in the stream. A sequence number is the identifier associated with every record put into the stream.</p>
    pub fn get_sequence_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.sequence_number
    }
    /// <p>The encryption type to use on the record. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn encryption_type(mut self, input: crate::types::EncryptionType) -> Self {
        self.encryption_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The encryption type to use on the record. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn set_encryption_type(mut self, input: ::std::option::Option<crate::types::EncryptionType>) -> Self {
        self.encryption_type = input;
        self
    }
    /// <p>The encryption type to use on the record. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn get_encryption_type(&self) -> &::std::option::Option<crate::types::EncryptionType> {
        &self.encryption_type
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`PutRecordOutput`](crate::operation::put_record::PutRecordOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`shard_id`](crate::operation::put_record::builders::PutRecordOutputBuilder::shard_id)
    /// - [`sequence_number`](crate::operation::put_record::builders::PutRecordOutputBuilder::sequence_number)
    pub fn build(self) -> ::std::result::Result<crate::operation::put_record::PutRecordOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::put_record::PutRecordOutput {
            shard_id: self.shard_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "shard_id",
                    "shard_id was not specified but it is required when building PutRecordOutput",
                )
            })?,
            sequence_number: self.sequence_number.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "sequence_number",
                    "sequence_number was not specified but it is required when building PutRecordOutput",
                )
            })?,
            encryption_type: self.encryption_type,
            _request_id: self._request_id,
        })
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents the output for <code>DescribeStream</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StreamDescription {
    /// <p>The name of the stream being described.</p>
    pub stream_name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    pub stream_arn: ::std::string::String,
    /// <p>The current status of the stream being described. The stream status is one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li>
    /// <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li>
    /// <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li>
    /// </ul>
    pub stream_status: crate::types::StreamStatus,
    /// <p> Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p>
    pub stream_mode_details: ::std::option::Option<crate::types::StreamModeDetails>,
    /// <p>The shards that comprise the stream.</p>
    pub shards: ::std::vec::Vec<crate::types::Shard>,
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    pub has_more_shards: bool,
    /// <p>The current retention period, in hours. Minimum value of 24. Maximum value of 168.</p>
    pub retention_period_hours: i32,
    /// <p>The approximate time that the stream was created.</p>
    pub stream_creation_timestamp: ::aws_smithy_types::DateTime,
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    pub enhanced_monitoring: ::std::vec::Vec<crate::types::EnhancedMetrics>,
    /// <p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub key_id: ::std::option::Option<::std::string::String>,
}
impl StreamDescription {
    /// <p>The name of the stream being described.</p>
    pub fn stream_name(&self) -> &str {
        use std::ops::Deref;
        self.stream_name.deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    pub fn stream_arn(&self) -> &str {
        use std::ops::Deref;
        self.stream_arn.deref()
    }
    /// <p>The current status of the stream being described. The stream status is one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li>
    /// <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li>
    /// <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li>
    /// </ul>
    pub fn stream_status(&self) -> &crate::types::StreamStatus {
        &self.stream_status
    }
    /// <p> Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p>
    pub fn stream_mode_details(&self) -> ::std::option::Option<&crate::types::StreamModeDetails> {
        self.stream_mode_details.as_ref()
    }
    /// <p>The shards that comprise the stream.</p>
    pub fn shards(&self) -> &[crate::types::Shard] {
        use std::ops::Deref;
        self.shards.deref()
    }
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    pub fn has_more_shards(&self) -> bool {
        self.has_more_shards
    }
    /// <p>The current retention period, in hours. Minimum value of 24. Maximum value of 168.</p>
    pub fn retention_period_hours(&self) -> i32 {
        self.retention_period_hours
    }
    /// <p>The approximate time that the stream was created.</p>
    pub fn stream_creation_timestamp(&self) -> &::aws_smithy_types::DateTime {
        &self.stream_creation_timestamp
    }
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    pub fn enhanced_monitoring(&self) -> &[crate::types::EnhancedMetrics] {
        use std::ops::Deref;
        self.enhanced_monitoring.deref()
    }
    /// <p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn encryption_type(&self) -> ::std::option::Option<&crate::types::EncryptionType> {
        self.encryption_type.as_ref()
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn key_id(&self) -> ::std::option::Option<&str> {
        self.key_id.as_deref()
    }
}
impl StreamDescription {
    /// Creates a new builder-style object to manufacture [`StreamDescription`](crate::types::StreamDescription).
    pub fn builder() -> crate::types::builders::StreamDescriptionBuilder {
        crate::types::builders::StreamDescriptionBuilder::default()
    }
}

/// A builder for [`StreamDescription`](crate::types::StreamDescription).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StreamDescriptionBuilder {
    pub(crate) stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) stream_arn: ::std::option::Option<::std::string::String>,
    pub(crate) stream_status: ::std::option::Option<crate::types::StreamStatus>,
    pub(crate) stream_mode_details: ::std::option::Option<crate::types::StreamModeDetails>,
    pub(crate) shards: ::std::option::Option<::std::vec::Vec<crate::types::Shard>>,
    pub(crate) has_more_shards: ::std::option::Option<bool>,
    pub(crate) retention_period_hours: ::std::option::Option<i32>,
    pub(crate) stream_creation_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) enhanced_monitoring: ::std::option::Option<::std::vec::Vec<crate::types::EnhancedMetrics>>,
    pub(crate) encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    pub(crate) key_id: ::std::option::Option<::std::string::String>,
}
impl StreamDescriptionBuilder {
    /// <p>The name of the stream being described.</p>
    /// This field is required.
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the stream being described.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_name = input;
        self
    }
    /// <p>The name of the stream being described.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_name
    }
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    /// This field is required.
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.stream_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.stream_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the stream being described.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.stream_arn
    }
    /// <p>The current status of the stream being described. The stream status is one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li>
    /// <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li>
    /// <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li>
    /// </ul>
    /// This field is required.
    pub fn stream_status(mut self, input: crate::types::StreamStatus) -> Self {
        self.stream_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the stream being described. The stream status is one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li>
    /// <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li>
    /// <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li>
    /// </ul>
    pub fn set_stream_status(mut self, input: ::std::option::Option<crate::types::StreamStatus>) -> Self {
        self.stream_status = input;
        self
    }
    /// <p>The current status of the stream being described. The stream status is one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>CREATING</code> - The stream is being created. Kinesis Data Streams immediately returns and sets <code>StreamStatus</code> to <code>CREATING</code>.</p> </li>
    /// <li> <p> <code>DELETING</code> - The stream is being deleted. The specified stream is in the <code>DELETING</code> state until Kinesis Data Streams completes the deletion.</p> </li>
    /// <li> <p> <code>ACTIVE</code> - The stream exists and is ready for read and write operations or deletion. You should perform read and write operations only on an <code>ACTIVE</code> stream.</p> </li>
    /// <li> <p> <code>UPDATING</code> - Shards in the stream are being merged or split. Read and write operations continue to work while the stream is in the <code>UPDATING</code> state.</p> </li>
    /// </ul>
    pub fn get_stream_status(&self) -> &::std::option::Option<crate::types::StreamStatus> {
        &self.stream_status
    }
    /// <p> Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p>
    pub fn stream_mode_details(mut self, input: crate::types::StreamModeDetails) -> Self {
        self.stream_mode_details = ::std::option::Option::Some(input);
        self
    }
    /// <p> Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p>
    pub fn set_stream_mode_details(mut self, input: ::std::option::Option<crate::types::StreamModeDetails>) -> Self {
        self.stream_mode_details = input;
        self
    }
    /// <p> Specifies the capacity mode to which you want to set your data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams. </p>
    pub fn get_stream_mode_details(&self) -> &::std::option::Option<crate::types::StreamModeDetails> {
        &self.stream_mode_details
    }
    /// Appends an item to `shards`.
    ///
    /// To override the contents of this collection use [`set_shards`](Self::set_shards).
    ///
    /// <p>The shards that comprise the stream.</p>
    pub fn shards(mut self, input: crate::types::Shard) -> Self {
        let mut v = self.shards.unwrap_or_default();
        v.push(input);
        self.shards = ::std::option::Option::Some(v);
        self
    }
    /// <p>The shards that comprise the stream.</p>
    pub fn set_shards(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Shard>>) -> Self {
        self.shards = input;
        self
    }
    /// <p>The shards that comprise the stream.</p>
    pub fn get_shards(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Shard>> {
        &self.shards
    }
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    /// This field is required.
    pub fn has_more_shards(mut self, input: bool) -> Self {
        self.has_more_shards = ::std::option::Option::Some(input);
        self
    }
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    pub fn set_has_more_shards(mut self, input: ::std::option::Option<bool>) -> Self {
        self.has_more_shards = input;
        self
    }
    /// <p>If set to <code>true</code>, more shards in the stream are available to describe.</p>
    pub fn get_has_more_shards(&self) -> &::std::option::Option<bool> {
        &self.has_more_shards
    }
    /// <p>The current retention period, in hours. Minimum value of 24. Maximum value of 168.</p>
    /// This field is required.
    pub fn retention_period_hours(mut self, input: i32) -> Self {
        self.retention_period_hours = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current retention period, in hours. Minimum value of 24. Maximum value of 168.</p>
    pub fn set_retention_period_hours(mut self, input: ::std::option::Option<i32>) -> Self {
        self.retention_period_hours = input;
        self
    }
    /// <p>The current retention period, in hours. Minimum value of 24. Maximum value of 168.</p>
    pub fn get_retention_period_hours(&self) -> &::std::option::Option<i32> {
        &self.retention_period_hours
    }
    /// <p>The approximate time that the stream was created.</p>
    /// This field is required.
    pub fn stream_creation_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.stream_creation_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The approximate time that the stream was created.</p>
    pub fn set_stream_creation_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.stream_creation_timestamp = input;
        self
    }
    /// <p>The approximate time that the stream was created.</p>
    pub fn get_stream_creation_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.stream_creation_timestamp
    }
    /// Appends an item to `enhanced_monitoring`.
    ///
    /// To override the contents of this collection use [`set_enhanced_monitoring`](Self::set_enhanced_monitoring).
    ///
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    pub fn enhanced_monitoring(mut self, input: crate::types::EnhancedMetrics) -> Self {
        let mut v = self.enhanced_monitoring.unwrap_or_default();
        v.push(input);
        self.enhanced_monitoring = ::std::option::Option::Some(v);
        self
    }
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    pub fn set_enhanced_monitoring(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EnhancedMetrics>>) -> Self {
        self.enhanced_monitoring = input;
        self
    }
    /// <p>Represents the current enhanced monitoring settings of the stream.</p>
    pub fn get_enhanced_monitoring(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EnhancedMetrics>> {
        &self.enhanced_monitoring
    }
    /// <p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn encryption_type(mut self, input: crate::types::EncryptionType) -> Self {
        self.encryption_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn set_encryption_type(mut self, input: ::std::option::Option<crate::types::EncryptionType>) -> Self {
        self.encryption_type = input;
        self
    }
    /// <p>The server-side encryption type used on the stream. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>NONE</code>: Do not encrypt the records in the stream.</p> </li>
    /// <li> <p> <code>KMS</code>: Use server-side encryption on the records in the stream using a customer-managed Amazon Web Services KMS key.</p> </li>
    /// </ul>
    pub fn get_encryption_type(&self) -> &::std::option::Option<crate::types::EncryptionType> {
        &self.encryption_type
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_id = input;
        self
    }
    /// <p>The GUID for the customer-managed Amazon Web Services KMS key to use for encryption. This value can be a globally unique identifier, a fully specified ARN to either an alias or a key, or an alias name prefixed by "alias/".You can also use a master key owned by Kinesis Data Streams by specifying the alias <code>aws/kinesis</code>.</p>
    /// <ul>
    /// <li> <p>Key ARN example: <code>arn:aws:kms:us-east-1:123456789012:key/12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias ARN example: <code>arn:aws:kms:us-east-1:123456789012:alias/MyAliasName</code> </p> </li>
    /// <li> <p>Globally unique key ID example: <code>12345678-1234-1234-1234-123456789012</code> </p> </li>
    /// <li> <p>Alias name example: <code>alias/MyAliasName</code> </p> </li>
    /// <li> <p>Master key owned by Kinesis Data Streams: <code>alias/aws/kinesis</code> </p> </li>
    /// </ul>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_id
    }
    /// Consumes the builder and constructs a [`StreamDescription`](crate::types::StreamDescription).
    /// This method will fail if any of the following fields are not set:
    /// - [`stream_name`](crate::types::builders::StreamDescriptionBuilder::stream_name)
    /// - [`stream_arn`](crate::types::builders::StreamDescriptionBuilder::stream_arn)
    /// - [`stream_status`](crate::types::builders::StreamDescriptionBuilder::stream_status)
    /// - [`shards`](crate::types::builders::StreamDescriptionBuilder::shards)
    /// - [`has_more_shards`](crate::types::builders::StreamDescriptionBuilder::has_more_shards)
    /// - [`retention_period_hours`](crate::types::builders::StreamDescriptionBuilder::retention_period_hours)
    /// - [`stream_creation_timestamp`](crate::types::builders::StreamDescriptionBuilder::stream_creation_timestamp)
    /// - [`enhanced_monitoring`](crate::types::builders::StreamDescriptionBuilder::enhanced_monitoring)
    pub fn build(self) -> ::std::result::Result<crate::types::StreamDescription, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::StreamDescription {
            stream_name: self.stream_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "stream_name",
                    "stream_name was not specified but it is required when building StreamDescription",
                )
            })?,
            stream_arn: self.stream_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "stream_arn",
                    "stream_arn was not specified but it is required when building StreamDescription",
                )
            })?,
            stream_status: self.stream_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "stream_status",
                    "stream_status was not specified but it is required when building StreamDescription",
                )
            })?,
            stream_mode_details: self.stream_mode_details,
            shards: self.shards.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "shards",
                    "shards was not specified but it is required when building StreamDescription",
                )
            })?,
            has_more_shards: self.has_more_shards.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "has_more_shards",
                    "has_more_shards was not specified but it is required when building StreamDescription",
                )
            })?,
            retention_period_hours: self.retention_period_hours.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "retention_period_hours",
                    "retention_period_hours was not specified but it is required when building StreamDescription",
                )
            })?,
            stream_creation_timestamp: self.stream_creation_timestamp.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "stream_creation_timestamp",
                    "stream_creation_timestamp was not specified but it is required when building StreamDescription",
                )
            })?,
            enhanced_monitoring: self.enhanced_monitoring.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "enhanced_monitoring",
                    "enhanced_monitoring was not specified but it is required when building StreamDescription",
                )
            })?,
            encryption_type: self.encryption_type,
            key_id: self.key_id,
        })
    }
}

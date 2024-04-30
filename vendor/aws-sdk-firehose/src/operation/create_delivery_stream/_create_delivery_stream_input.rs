// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateDeliveryStreamInput {
    /// <p>The name of the delivery stream. This name must be unique per Amazon Web Services account in the same Amazon Web Services Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    pub delivery_stream_name: ::std::option::Option<::std::string::String>,
    /// <p>The delivery stream type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>
    /// <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>
    /// </ul>
    pub delivery_stream_type: ::std::option::Option<crate::types::DeliveryStreamType>,
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <code>KinesisStreamSourceConfiguration</code> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    pub kinesis_stream_source_configuration: ::std::option::Option<crate::types::KinesisStreamSourceConfiguration>,
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    pub delivery_stream_encryption_configuration_input: ::std::option::Option<crate::types::DeliveryStreamEncryptionConfigurationInput>,
    /// <p>[Deprecated] The destination in Amazon S3. You can specify only one destination.</p>
    #[deprecated]
    pub s3_destination_configuration: ::std::option::Option<crate::types::S3DestinationConfiguration>,
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    pub extended_s3_destination_configuration: ::std::option::Option<crate::types::ExtendedS3DestinationConfiguration>,
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    pub redshift_destination_configuration: ::std::option::Option<crate::types::RedshiftDestinationConfiguration>,
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    pub elasticsearch_destination_configuration: ::std::option::Option<crate::types::ElasticsearchDestinationConfiguration>,
    /// <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p>
    pub amazonopensearchservice_destination_configuration: ::std::option::Option<crate::types::AmazonopensearchserviceDestinationConfiguration>,
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    pub splunk_destination_configuration: ::std::option::Option<crate::types::SplunkDestinationConfiguration>,
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    pub http_endpoint_destination_configuration: ::std::option::Option<crate::types::HttpEndpointDestinationConfiguration>,
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User Guide.</p>
    /// <p>You can specify up to 50 tags when creating a delivery stream.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can specify only one destination.</p>
    pub amazon_open_search_serverless_destination_configuration:
        ::std::option::Option<crate::types::AmazonOpenSearchServerlessDestinationConfiguration>,
    /// <p>The configuration for the Amazon MSK cluster to be used as the source for a delivery stream.</p>
    pub msk_source_configuration: ::std::option::Option<crate::types::MskSourceConfiguration>,
}
impl CreateDeliveryStreamInput {
    /// <p>The name of the delivery stream. This name must be unique per Amazon Web Services account in the same Amazon Web Services Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    pub fn delivery_stream_name(&self) -> ::std::option::Option<&str> {
        self.delivery_stream_name.as_deref()
    }
    /// <p>The delivery stream type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>
    /// <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>
    /// </ul>
    pub fn delivery_stream_type(&self) -> ::std::option::Option<&crate::types::DeliveryStreamType> {
        self.delivery_stream_type.as_ref()
    }
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <code>KinesisStreamSourceConfiguration</code> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    pub fn kinesis_stream_source_configuration(&self) -> ::std::option::Option<&crate::types::KinesisStreamSourceConfiguration> {
        self.kinesis_stream_source_configuration.as_ref()
    }
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    pub fn delivery_stream_encryption_configuration_input(&self) -> ::std::option::Option<&crate::types::DeliveryStreamEncryptionConfigurationInput> {
        self.delivery_stream_encryption_configuration_input.as_ref()
    }
    /// <p>[Deprecated] The destination in Amazon S3. You can specify only one destination.</p>
    #[deprecated]
    pub fn s3_destination_configuration(&self) -> ::std::option::Option<&crate::types::S3DestinationConfiguration> {
        self.s3_destination_configuration.as_ref()
    }
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    pub fn extended_s3_destination_configuration(&self) -> ::std::option::Option<&crate::types::ExtendedS3DestinationConfiguration> {
        self.extended_s3_destination_configuration.as_ref()
    }
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    pub fn redshift_destination_configuration(&self) -> ::std::option::Option<&crate::types::RedshiftDestinationConfiguration> {
        self.redshift_destination_configuration.as_ref()
    }
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    pub fn elasticsearch_destination_configuration(&self) -> ::std::option::Option<&crate::types::ElasticsearchDestinationConfiguration> {
        self.elasticsearch_destination_configuration.as_ref()
    }
    /// <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn amazonopensearchservice_destination_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::AmazonopensearchserviceDestinationConfiguration> {
        self.amazonopensearchservice_destination_configuration.as_ref()
    }
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    pub fn splunk_destination_configuration(&self) -> ::std::option::Option<&crate::types::SplunkDestinationConfiguration> {
        self.splunk_destination_configuration.as_ref()
    }
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    pub fn http_endpoint_destination_configuration(&self) -> ::std::option::Option<&crate::types::HttpEndpointDestinationConfiguration> {
        self.http_endpoint_destination_configuration.as_ref()
    }
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User Guide.</p>
    /// <p>You can specify up to 50 tags when creating a delivery stream.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn amazon_open_search_serverless_destination_configuration(
        &self,
    ) -> ::std::option::Option<&crate::types::AmazonOpenSearchServerlessDestinationConfiguration> {
        self.amazon_open_search_serverless_destination_configuration.as_ref()
    }
    /// <p>The configuration for the Amazon MSK cluster to be used as the source for a delivery stream.</p>
    pub fn msk_source_configuration(&self) -> ::std::option::Option<&crate::types::MskSourceConfiguration> {
        self.msk_source_configuration.as_ref()
    }
}
impl CreateDeliveryStreamInput {
    /// Creates a new builder-style object to manufacture [`CreateDeliveryStreamInput`](crate::operation::create_delivery_stream::CreateDeliveryStreamInput).
    pub fn builder() -> crate::operation::create_delivery_stream::builders::CreateDeliveryStreamInputBuilder {
        crate::operation::create_delivery_stream::builders::CreateDeliveryStreamInputBuilder::default()
    }
}

/// A builder for [`CreateDeliveryStreamInput`](crate::operation::create_delivery_stream::CreateDeliveryStreamInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateDeliveryStreamInputBuilder {
    pub(crate) delivery_stream_name: ::std::option::Option<::std::string::String>,
    pub(crate) delivery_stream_type: ::std::option::Option<crate::types::DeliveryStreamType>,
    pub(crate) kinesis_stream_source_configuration: ::std::option::Option<crate::types::KinesisStreamSourceConfiguration>,
    pub(crate) delivery_stream_encryption_configuration_input: ::std::option::Option<crate::types::DeliveryStreamEncryptionConfigurationInput>,
    pub(crate) s3_destination_configuration: ::std::option::Option<crate::types::S3DestinationConfiguration>,
    pub(crate) extended_s3_destination_configuration: ::std::option::Option<crate::types::ExtendedS3DestinationConfiguration>,
    pub(crate) redshift_destination_configuration: ::std::option::Option<crate::types::RedshiftDestinationConfiguration>,
    pub(crate) elasticsearch_destination_configuration: ::std::option::Option<crate::types::ElasticsearchDestinationConfiguration>,
    pub(crate) amazonopensearchservice_destination_configuration:
        ::std::option::Option<crate::types::AmazonopensearchserviceDestinationConfiguration>,
    pub(crate) splunk_destination_configuration: ::std::option::Option<crate::types::SplunkDestinationConfiguration>,
    pub(crate) http_endpoint_destination_configuration: ::std::option::Option<crate::types::HttpEndpointDestinationConfiguration>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) amazon_open_search_serverless_destination_configuration:
        ::std::option::Option<crate::types::AmazonOpenSearchServerlessDestinationConfiguration>,
    pub(crate) msk_source_configuration: ::std::option::Option<crate::types::MskSourceConfiguration>,
}
impl CreateDeliveryStreamInputBuilder {
    /// <p>The name of the delivery stream. This name must be unique per Amazon Web Services account in the same Amazon Web Services Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    /// This field is required.
    pub fn delivery_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.delivery_stream_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the delivery stream. This name must be unique per Amazon Web Services account in the same Amazon Web Services Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    pub fn set_delivery_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.delivery_stream_name = input;
        self
    }
    /// <p>The name of the delivery stream. This name must be unique per Amazon Web Services account in the same Amazon Web Services Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    pub fn get_delivery_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.delivery_stream_name
    }
    /// <p>The delivery stream type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>
    /// <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>
    /// </ul>
    pub fn delivery_stream_type(mut self, input: crate::types::DeliveryStreamType) -> Self {
        self.delivery_stream_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The delivery stream type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>
    /// <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>
    /// </ul>
    pub fn set_delivery_stream_type(mut self, input: ::std::option::Option<crate::types::DeliveryStreamType>) -> Self {
        self.delivery_stream_type = input;
        self
    }
    /// <p>The delivery stream type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li>
    /// <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li>
    /// </ul>
    pub fn get_delivery_stream_type(&self) -> &::std::option::Option<crate::types::DeliveryStreamType> {
        &self.delivery_stream_type
    }
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <code>KinesisStreamSourceConfiguration</code> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    pub fn kinesis_stream_source_configuration(mut self, input: crate::types::KinesisStreamSourceConfiguration) -> Self {
        self.kinesis_stream_source_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <code>KinesisStreamSourceConfiguration</code> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    pub fn set_kinesis_stream_source_configuration(mut self, input: ::std::option::Option<crate::types::KinesisStreamSourceConfiguration>) -> Self {
        self.kinesis_stream_source_configuration = input;
        self
    }
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <code>KinesisStreamSourceConfiguration</code> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    pub fn get_kinesis_stream_source_configuration(&self) -> &::std::option::Option<crate::types::KinesisStreamSourceConfiguration> {
        &self.kinesis_stream_source_configuration
    }
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    pub fn delivery_stream_encryption_configuration_input(mut self, input: crate::types::DeliveryStreamEncryptionConfigurationInput) -> Self {
        self.delivery_stream_encryption_configuration_input = ::std::option::Option::Some(input);
        self
    }
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    pub fn set_delivery_stream_encryption_configuration_input(
        mut self,
        input: ::std::option::Option<crate::types::DeliveryStreamEncryptionConfigurationInput>,
    ) -> Self {
        self.delivery_stream_encryption_configuration_input = input;
        self
    }
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    pub fn get_delivery_stream_encryption_configuration_input(
        &self,
    ) -> &::std::option::Option<crate::types::DeliveryStreamEncryptionConfigurationInput> {
        &self.delivery_stream_encryption_configuration_input
    }
    /// <p>[Deprecated] The destination in Amazon S3. You can specify only one destination.</p>
    #[deprecated]
    pub fn s3_destination_configuration(mut self, input: crate::types::S3DestinationConfiguration) -> Self {
        self.s3_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>[Deprecated] The destination in Amazon S3. You can specify only one destination.</p>
    #[deprecated]
    pub fn set_s3_destination_configuration(mut self, input: ::std::option::Option<crate::types::S3DestinationConfiguration>) -> Self {
        self.s3_destination_configuration = input;
        self
    }
    /// <p>[Deprecated] The destination in Amazon S3. You can specify only one destination.</p>
    #[deprecated]
    pub fn get_s3_destination_configuration(&self) -> &::std::option::Option<crate::types::S3DestinationConfiguration> {
        &self.s3_destination_configuration
    }
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    pub fn extended_s3_destination_configuration(mut self, input: crate::types::ExtendedS3DestinationConfiguration) -> Self {
        self.extended_s3_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    pub fn set_extended_s3_destination_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ExtendedS3DestinationConfiguration>,
    ) -> Self {
        self.extended_s3_destination_configuration = input;
        self
    }
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    pub fn get_extended_s3_destination_configuration(&self) -> &::std::option::Option<crate::types::ExtendedS3DestinationConfiguration> {
        &self.extended_s3_destination_configuration
    }
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    pub fn redshift_destination_configuration(mut self, input: crate::types::RedshiftDestinationConfiguration) -> Self {
        self.redshift_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    pub fn set_redshift_destination_configuration(mut self, input: ::std::option::Option<crate::types::RedshiftDestinationConfiguration>) -> Self {
        self.redshift_destination_configuration = input;
        self
    }
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    pub fn get_redshift_destination_configuration(&self) -> &::std::option::Option<crate::types::RedshiftDestinationConfiguration> {
        &self.redshift_destination_configuration
    }
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    pub fn elasticsearch_destination_configuration(mut self, input: crate::types::ElasticsearchDestinationConfiguration) -> Self {
        self.elasticsearch_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    pub fn set_elasticsearch_destination_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ElasticsearchDestinationConfiguration>,
    ) -> Self {
        self.elasticsearch_destination_configuration = input;
        self
    }
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    pub fn get_elasticsearch_destination_configuration(&self) -> &::std::option::Option<crate::types::ElasticsearchDestinationConfiguration> {
        &self.elasticsearch_destination_configuration
    }
    /// <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn amazonopensearchservice_destination_configuration(mut self, input: crate::types::AmazonopensearchserviceDestinationConfiguration) -> Self {
        self.amazonopensearchservice_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn set_amazonopensearchservice_destination_configuration(
        mut self,
        input: ::std::option::Option<crate::types::AmazonopensearchserviceDestinationConfiguration>,
    ) -> Self {
        self.amazonopensearchservice_destination_configuration = input;
        self
    }
    /// <p>The destination in Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn get_amazonopensearchservice_destination_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::AmazonopensearchserviceDestinationConfiguration> {
        &self.amazonopensearchservice_destination_configuration
    }
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    pub fn splunk_destination_configuration(mut self, input: crate::types::SplunkDestinationConfiguration) -> Self {
        self.splunk_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    pub fn set_splunk_destination_configuration(mut self, input: ::std::option::Option<crate::types::SplunkDestinationConfiguration>) -> Self {
        self.splunk_destination_configuration = input;
        self
    }
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    pub fn get_splunk_destination_configuration(&self) -> &::std::option::Option<crate::types::SplunkDestinationConfiguration> {
        &self.splunk_destination_configuration
    }
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    pub fn http_endpoint_destination_configuration(mut self, input: crate::types::HttpEndpointDestinationConfiguration) -> Self {
        self.http_endpoint_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    pub fn set_http_endpoint_destination_configuration(
        mut self,
        input: ::std::option::Option<crate::types::HttpEndpointDestinationConfiguration>,
    ) -> Self {
        self.http_endpoint_destination_configuration = input;
        self
    }
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    pub fn get_http_endpoint_destination_configuration(&self) -> &::std::option::Option<crate::types::HttpEndpointDestinationConfiguration> {
        &self.http_endpoint_destination_configuration
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User Guide.</p>
    /// <p>You can specify up to 50 tags when creating a delivery stream.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User Guide.</p>
    /// <p>You can specify up to 50 tags when creating a delivery stream.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to Amazon Web Services resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the Amazon Web Services Billing and Cost Management User Guide.</p>
    /// <p>You can specify up to 50 tags when creating a delivery stream.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn amazon_open_search_serverless_destination_configuration(
        mut self,
        input: crate::types::AmazonOpenSearchServerlessDestinationConfiguration,
    ) -> Self {
        self.amazon_open_search_serverless_destination_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn set_amazon_open_search_serverless_destination_configuration(
        mut self,
        input: ::std::option::Option<crate::types::AmazonOpenSearchServerlessDestinationConfiguration>,
    ) -> Self {
        self.amazon_open_search_serverless_destination_configuration = input;
        self
    }
    /// <p>The destination in the Serverless offering for Amazon OpenSearch Service. You can specify only one destination.</p>
    pub fn get_amazon_open_search_serverless_destination_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::AmazonOpenSearchServerlessDestinationConfiguration> {
        &self.amazon_open_search_serverless_destination_configuration
    }
    /// <p>The configuration for the Amazon MSK cluster to be used as the source for a delivery stream.</p>
    pub fn msk_source_configuration(mut self, input: crate::types::MskSourceConfiguration) -> Self {
        self.msk_source_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The configuration for the Amazon MSK cluster to be used as the source for a delivery stream.</p>
    pub fn set_msk_source_configuration(mut self, input: ::std::option::Option<crate::types::MskSourceConfiguration>) -> Self {
        self.msk_source_configuration = input;
        self
    }
    /// <p>The configuration for the Amazon MSK cluster to be used as the source for a delivery stream.</p>
    pub fn get_msk_source_configuration(&self) -> &::std::option::Option<crate::types::MskSourceConfiguration> {
        &self.msk_source_configuration
    }
    /// Consumes the builder and constructs a [`CreateDeliveryStreamInput`](crate::operation::create_delivery_stream::CreateDeliveryStreamInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_delivery_stream::CreateDeliveryStreamInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::create_delivery_stream::CreateDeliveryStreamInput {
            delivery_stream_name: self.delivery_stream_name,
            delivery_stream_type: self.delivery_stream_type,
            kinesis_stream_source_configuration: self.kinesis_stream_source_configuration,
            delivery_stream_encryption_configuration_input: self.delivery_stream_encryption_configuration_input,
            s3_destination_configuration: self.s3_destination_configuration,
            extended_s3_destination_configuration: self.extended_s3_destination_configuration,
            redshift_destination_configuration: self.redshift_destination_configuration,
            elasticsearch_destination_configuration: self.elasticsearch_destination_configuration,
            amazonopensearchservice_destination_configuration: self.amazonopensearchservice_destination_configuration,
            splunk_destination_configuration: self.splunk_destination_configuration,
            http_endpoint_destination_configuration: self.http_endpoint_destination_configuration,
            tags: self.tags,
            amazon_open_search_serverless_destination_configuration: self.amazon_open_search_serverless_destination_configuration,
            msk_source_configuration: self.msk_source_configuration,
        })
    }
}

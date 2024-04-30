// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetRecords`](crate::operation::get_records::builders::GetRecordsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`shard_iterator(impl Into<String>)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::shard_iterator) / [`set_shard_iterator(Option<String>)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::set_shard_iterator):<br>required: **true**<br><p>The position in the shard from which you want to start sequentially reading data records. A shard iterator specifies this position using the sequence number of a data record in the shard.</p><br>
    ///   - [`limit(i32)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of records to return. Specify a value of up to 10,000. If you specify a value that is greater than 10,000, <code>GetRecords</code> throws <code>InvalidArgumentException</code>. The default value is 10,000.</p><br>
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::get_records::builders::GetRecordsFluentBuilder::set_stream_arn):<br>required: **false**<br><p>The ARN of the stream.</p><br>
    /// - On success, responds with [`GetRecordsOutput`](crate::operation::get_records::GetRecordsOutput) with field(s):
    ///   - [`records(Vec::<Record>)`](crate::operation::get_records::GetRecordsOutput::records): <p>The data records retrieved from the shard.</p>
    ///   - [`next_shard_iterator(Option<String>)`](crate::operation::get_records::GetRecordsOutput::next_shard_iterator): <p>The next position in the shard from which to start sequentially reading data records. If set to <code>null</code>, the shard has been closed and the requested iterator does not return any more data. </p>
    ///   - [`millis_behind_latest(Option<i64>)`](crate::operation::get_records::GetRecordsOutput::millis_behind_latest): <p>The number of milliseconds the <code>GetRecords</code> response is from the tip of the stream, indicating how far behind current time the consumer is. A value of zero indicates that record processing is caught up, and there are no new records to process at this moment.</p>
    ///   - [`child_shards(Option<Vec::<ChildShard>>)`](crate::operation::get_records::GetRecordsOutput::child_shards): <p>The list of the current shard's child shards, returned in the <code>GetRecords</code> API's response only when the end of the current shard is reached.</p>
    /// - On failure, responds with [`SdkError<GetRecordsError>`](crate::operation::get_records::GetRecordsError)
    pub fn get_records(&self) -> crate::operation::get_records::builders::GetRecordsFluentBuilder {
        crate::operation::get_records::builders::GetRecordsFluentBuilder::new(self.handle.clone())
    }
}

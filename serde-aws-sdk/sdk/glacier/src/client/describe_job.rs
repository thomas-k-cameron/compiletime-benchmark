// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeJob`](crate::operation::describe_job::builders::DescribeJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::set_account_id): <p>The <code>AccountId</code> value is the AWS account ID of the account that owns the vault. You can either specify an AWS account ID or optionally a single '<code>-</code>' (hyphen), in which case Amazon S3 Glacier uses the AWS account ID associated with the credentials used to sign the request. If you use an account ID, do not include any hyphens ('-') in the ID. </p>
    ///   - [`vault_name(impl Into<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::vault_name) / [`set_vault_name(Option<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::set_vault_name): <p>The name of the vault.</p>
    ///   - [`job_id(impl Into<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::describe_job::builders::DescribeJobFluentBuilder::set_job_id): <p>The ID of the job to describe.</p>
    /// - On success, responds with [`DescribeJobOutput`](crate::operation::describe_job::DescribeJobOutput) with field(s):
    ///   - [`job_id(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::job_id): <p>An opaque string that identifies an Amazon S3 Glacier job.</p>
    ///   - [`job_description(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::job_description): <p>The job description provided when initiating the job.</p>
    ///   - [`action(Option<ActionCode>)`](crate::operation::describe_job::DescribeJobOutput::action): <p>The job type. This value is either <code>ArchiveRetrieval</code>, <code>InventoryRetrieval</code>, or <code>Select</code>. </p>
    ///   - [`archive_id(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::archive_id): <p>The archive ID requested for a select job or archive retrieval. Otherwise, this field is null.</p>
    ///   - [`vault_arn(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::vault_arn): <p>The Amazon Resource Name (ARN) of the vault from which an archive retrieval was requested.</p>
    ///   - [`creation_date(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::creation_date): <p>The UTC date when the job was created. This value is a string representation of ISO 8601 date format, for example <code>"2012-03-20T17:03:43.221Z"</code>.</p>
    ///   - [`completed(bool)`](crate::operation::describe_job::DescribeJobOutput::completed): <p>The job status. When a job is completed, you get the job's output using Get Job Output (GET output).</p>
    ///   - [`status_code(Option<StatusCode>)`](crate::operation::describe_job::DescribeJobOutput::status_code): <p>The status code can be <code>InProgress</code>, <code>Succeeded</code>, or <code>Failed</code>, and indicates the status of the job.</p>
    ///   - [`status_message(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::status_message): <p>A friendly message that describes the job status.</p>
    ///   - [`archive_size_in_bytes(Option<i64>)`](crate::operation::describe_job::DescribeJobOutput::archive_size_in_bytes): <p>For an archive retrieval job, this value is the size in bytes of the archive being requested for download. For an inventory retrieval or select job, this value is null.</p>
    ///   - [`inventory_size_in_bytes(Option<i64>)`](crate::operation::describe_job::DescribeJobOutput::inventory_size_in_bytes): <p>For an inventory retrieval job, this value is the size in bytes of the inventory requested for download. For an archive retrieval or select job, this value is null.</p>
    ///   - [`sns_topic(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::sns_topic): <p>An Amazon SNS topic that receives notification.</p>
    ///   - [`completion_date(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::completion_date): <p>The UTC time that the job request completed. While the job is in progress, the value is null.</p>
    ///   - [`sha256_tree_hash(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::sha256_tree_hash): <p>For an archive retrieval job, this value is the checksum of the archive. Otherwise, this value is null.</p>  <p>The SHA256 tree hash value for the requested range of an archive. If the <b>InitiateJob</b> request for an archive specified a tree-hash aligned range, then this field returns a value.</p>  <p>If the whole archive is retrieved, this value is the same as the ArchiveSHA256TreeHash value.</p>  <p>This field is null for the following:</p>  <ul>   <li> <p>Archive retrieval jobs that specify a range that is not tree-hash aligned</p> </li>  </ul>  <ul>   <li> <p>Archival jobs that specify a range that is equal to the whole archive, when the job status is <code>InProgress</code> </p> </li>  </ul>  <ul>   <li> <p>Inventory jobs</p> </li>   <li> <p>Select jobs</p> </li>  </ul>
    ///   - [`archive_sha256_tree_hash(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::archive_sha256_tree_hash): <p>The SHA256 tree hash of the entire archive for an archive retrieval. For inventory retrieval or select jobs, this field is null.</p>
    ///   - [`retrieval_byte_range(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::retrieval_byte_range): <p>The retrieved byte range for archive retrieval jobs in the form <i>StartByteValue</i>-<i>EndByteValue</i>. If no range was specified in the archive retrieval, then the whole archive is retrieved. In this case, <i>StartByteValue</i> equals 0 and <i>EndByteValue</i> equals the size of the archive minus 1. For inventory retrieval or select jobs, this field is null. </p>
    ///   - [`tier(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::tier): <p>The tier to use for a select or an archive retrieval. Valid values are <code>Expedited</code>, <code>Standard</code>, or <code>Bulk</code>. <code>Standard</code> is the default.</p>
    ///   - [`inventory_retrieval_parameters(Option<InventoryRetrievalJobDescription>)`](crate::operation::describe_job::DescribeJobOutput::inventory_retrieval_parameters): <p>Parameters used for range inventory retrieval.</p>
    ///   - [`job_output_path(Option<String>)`](crate::operation::describe_job::DescribeJobOutput::job_output_path): <p>Contains the job output location.</p>
    ///   - [`select_parameters(Option<SelectParameters>)`](crate::operation::describe_job::DescribeJobOutput::select_parameters): <p>Contains the parameters used for a select.</p>
    ///   - [`output_location(Option<OutputLocation>)`](crate::operation::describe_job::DescribeJobOutput::output_location): <p>Contains the location where the data from the select job is stored.</p>
    /// - On failure, responds with [`SdkError<DescribeJobError>`](crate::operation::describe_job::DescribeJobError)
    pub fn describe_job(
        &self,
    ) -> crate::operation::describe_job::builders::DescribeJobFluentBuilder {
        crate::operation::describe_job::builders::DescribeJobFluentBuilder::new(self.handle.clone())
    }
}

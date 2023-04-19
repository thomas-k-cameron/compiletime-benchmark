// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSnapshot`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`description(impl Into<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::set_description): <p>A description for the snapshot.</p>
    ///   - [`outpost_arn(impl Into<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::outpost_arn) / [`set_outpost_arn(Option<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::set_outpost_arn): <p>The Amazon Resource Name (ARN) of the Outpost on which to create a local snapshot.</p>  <ul>   <li> <p>To create a snapshot of a volume in a Region, omit this parameter. The snapshot is created in the same Region as the volume.</p> </li>   <li> <p>To create a snapshot of a volume on an Outpost and store the snapshot in the Region, omit this parameter. The snapshot is created in the Region for the Outpost.</p> </li>   <li> <p>To create a snapshot of a volume on an Outpost and store the snapshot on an Outpost, specify the ARN of the destination Outpost. The snapshot must be created on the same Outpost as the volume.</p> </li>  </ul>  <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-snapshot">Create local snapshots from volumes on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    ///   - [`volume_id(impl Into<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::set_volume_id): <p>The ID of the Amazon EBS volume.</p>
    ///   - [`tag_specifications(Vec<TagSpecification>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec<TagSpecification>>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::set_tag_specifications): <p>The tags to apply to the snapshot during creation.</p>
    ///   - [`dry_run(bool)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::set_dry_run): <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    /// - On success, responds with [`CreateSnapshotOutput`](crate::operation::create_snapshot::CreateSnapshotOutput) with field(s):
    ///   - [`data_encryption_key_id(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::data_encryption_key_id): <p>The data encryption key identifier for the snapshot. This value is a unique identifier that corresponds to the data encryption key that was used to encrypt the original volume or snapshot copy. Because data encryption keys are inherited by volumes created from snapshots, and vice versa, if snapshots share the same data encryption key identifier, then they belong to the same volume/snapshot lineage. This parameter is only returned by <code>DescribeSnapshots</code>.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::description): <p>The description for the snapshot.</p>
    ///   - [`encrypted(Option<bool>)`](crate::operation::create_snapshot::CreateSnapshotOutput::encrypted): <p>Indicates whether the snapshot is encrypted.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::kms_key_id): <p>The Amazon Resource Name (ARN) of the Key Management Service (KMS) KMS key that was used to protect the volume encryption key for the parent volume.</p>
    ///   - [`owner_id(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::owner_id): <p>The ID of the Amazon Web Services account that owns the EBS snapshot.</p>
    ///   - [`progress(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::progress): <p>The progress of the snapshot, as a percentage.</p>
    ///   - [`snapshot_id(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::snapshot_id): <p>The ID of the snapshot. Each snapshot receives a unique identifier when it is created.</p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::create_snapshot::CreateSnapshotOutput::start_time): <p>The time stamp when the snapshot was initiated.</p>
    ///   - [`state(Option<SnapshotState>)`](crate::operation::create_snapshot::CreateSnapshotOutput::state): <p>The snapshot state.</p>
    ///   - [`state_message(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::state_message): <p>Encrypted Amazon EBS snapshots are copied asynchronously. If a snapshot copy operation fails (for example, if the proper Key Management Service (KMS) permissions are not obtained) this field displays error state details to help you diagnose why the error occurred. This parameter is only returned by <code>DescribeSnapshots</code>.</p>
    ///   - [`volume_id(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::volume_id): <p>The ID of the volume that was used to create the snapshot. Snapshots created by the <code>CopySnapshot</code> action have an arbitrary volume ID that should not be used for any purpose.</p>
    ///   - [`volume_size(Option<i32>)`](crate::operation::create_snapshot::CreateSnapshotOutput::volume_size): <p>The size of the volume, in GiB.</p>
    ///   - [`owner_alias(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::owner_alias): <p>The Amazon Web Services owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console.</p>
    ///   - [`outpost_arn(Option<String>)`](crate::operation::create_snapshot::CreateSnapshotOutput::outpost_arn): <p>The ARN of the Outpost on which the snapshot is stored. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html">Amazon EBS local snapshots on Outposts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    ///   - [`tags(Option<Vec<Tag>>)`](crate::operation::create_snapshot::CreateSnapshotOutput::tags): <p>Any tags assigned to the snapshot.</p>
    ///   - [`storage_tier(Option<StorageTier>)`](crate::operation::create_snapshot::CreateSnapshotOutput::storage_tier): <p>The storage tier in which the snapshot is stored. <code>standard</code> indicates that the snapshot is stored in the standard snapshot storage tier and that it is ready for use. <code>archive</code> indicates that the snapshot is currently archived and that it must be restored before it can be used.</p>
    ///   - [`restore_expiry_time(Option<DateTime>)`](crate::operation::create_snapshot::CreateSnapshotOutput::restore_expiry_time): <p>Only for archived snapshots that are temporarily restored. Indicates the date and time when a temporarily restored snapshot will be automatically re-archived.</p>
    /// - On failure, responds with [`SdkError<CreateSnapshotError>`](crate::operation::create_snapshot::CreateSnapshotError)
    pub fn create_snapshot(
        &self,
    ) -> crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder {
        crate::operation::create_snapshot::builders::CreateSnapshotFluentBuilder::new(
            self.handle.clone(),
        )
    }
}

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a block device for an EBS volume.</p>
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct EbsBlockDevice {
    /// <p>Indicates whether the EBS volume is deleted on instance termination. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#preserving-volumes-on-termination">Preserving Amazon EBS volumes on instance termination</a> in the <i>Amazon EC2 User Guide</i>.</p>
    #[doc(hidden)]
    pub delete_on_termination: std::option::Option<bool>,
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000-16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100-64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100-64,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io1</code> and <code>io2</code> volumes, we guarantee 64,000 IOPS only for <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a>. Other instance families guarantee performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    #[doc(hidden)]
    pub iops: std::option::Option<i32>,
    /// <p>The ID of the snapshot.</p>
    #[doc(hidden)]
    pub snapshot_id: std::option::Option<std::string::String>,
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>:1-16,384</p> </li>
    /// <li> <p> <code>io1</code> and <code>io2</code>: 4-16,384</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125-16,384</p> </li>
    /// <li> <p> <code>standard</code>: 1-1,024</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub volume_size: std::option::Option<i32>,
    /// <p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon EC2 User Guide</i>. If the volume type is <code>io1</code> or <code>io2</code>, you must specify the IOPS that the volume supports.</p>
    #[doc(hidden)]
    pub volume_type: std::option::Option<crate::types::VolumeType>,
    /// <p>Identifier (key ID, key alias, ID ARN, or alias ARN) for a customer managed CMK under which the EBS volume is encrypted.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotFleet.html">RequestSpotFleet</a>, and <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a>.</p>
    #[doc(hidden)]
    pub kms_key_id: std::option::Option<std::string::String>,
    /// <p>The throughput that the volume supports, in MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    #[doc(hidden)]
    pub throughput: std::option::Option<i32>,
    /// <p>The ARN of the Outpost on which the snapshot is stored.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateImage.html"> CreateImage</a>.</p>
    #[doc(hidden)]
    pub outpost_arn: std::option::Option<std::string::String>,
    /// <p>Indicates whether the encryption state of an EBS volume is changed while being restored from a backing snapshot. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-parameters">Amazon EBS encryption</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>In no case can you remove encryption from an encrypted volume.</p>
    /// <p>Encrypted volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    /// <p>This parameter is not returned by <code>DescribeImageAttribute</code>.</p>
    /// <p>For <code>CreateImage</code> and <code>RegisterImage</code>, whether you can include this parameter, and the allowed values differ depending on the type of block device mapping you are creating.</p>
    /// <ul>
    /// <li> <p>If you are creating a block device mapping for a <b>new (empty) volume</b>, you can include this parameter, and specify either <code>true</code> for an encrypted volume, or <code>false</code> for an unencrypted volume. If you omit this parameter, it defaults to <code>false</code> (unencrypted).</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted or unencrypted snapshot</b>, you must omit this parameter. If you include this parameter, the request will fail, regardless of the value that you specify.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing unencrypted volume</b>, you can include this parameter, but you must specify <code>false</code>. If you specify <code>true</code>, the request will fail. In this case, we recommend that you omit the parameter.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted volume</b>, you can include this parameter, and specify either <code>true</code> or <code>false</code>. However, if you specify <code>false</code>, the parameter is ignored and the block device mapping is always encrypted. In this case, we recommend that you omit the parameter.</p> </li>
    /// </ul>
    #[doc(hidden)]
    pub encrypted: std::option::Option<bool>,
}
impl EbsBlockDevice {
    /// <p>Indicates whether the EBS volume is deleted on instance termination. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#preserving-volumes-on-termination">Preserving Amazon EBS volumes on instance termination</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn delete_on_termination(&self) -> std::option::Option<bool> {
        self.delete_on_termination
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000-16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100-64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100-64,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io1</code> and <code>io2</code> volumes, we guarantee 64,000 IOPS only for <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a>. Other instance families guarantee performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn iops(&self) -> std::option::Option<i32> {
        self.iops
    }
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(&self) -> std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>:1-16,384</p> </li>
    /// <li> <p> <code>io1</code> and <code>io2</code>: 4-16,384</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125-16,384</p> </li>
    /// <li> <p> <code>standard</code>: 1-1,024</p> </li>
    /// </ul>
    pub fn volume_size(&self) -> std::option::Option<i32> {
        self.volume_size
    }
    /// <p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon EC2 User Guide</i>. If the volume type is <code>io1</code> or <code>io2</code>, you must specify the IOPS that the volume supports.</p>
    pub fn volume_type(&self) -> std::option::Option<&crate::types::VolumeType> {
        self.volume_type.as_ref()
    }
    /// <p>Identifier (key ID, key alias, ID ARN, or alias ARN) for a customer managed CMK under which the EBS volume is encrypted.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotFleet.html">RequestSpotFleet</a>, and <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a>.</p>
    pub fn kms_key_id(&self) -> std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The throughput that the volume supports, in MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn throughput(&self) -> std::option::Option<i32> {
        self.throughput
    }
    /// <p>The ARN of the Outpost on which the snapshot is stored.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateImage.html"> CreateImage</a>.</p>
    pub fn outpost_arn(&self) -> std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>Indicates whether the encryption state of an EBS volume is changed while being restored from a backing snapshot. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-parameters">Amazon EBS encryption</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>In no case can you remove encryption from an encrypted volume.</p>
    /// <p>Encrypted volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    /// <p>This parameter is not returned by <code>DescribeImageAttribute</code>.</p>
    /// <p>For <code>CreateImage</code> and <code>RegisterImage</code>, whether you can include this parameter, and the allowed values differ depending on the type of block device mapping you are creating.</p>
    /// <ul>
    /// <li> <p>If you are creating a block device mapping for a <b>new (empty) volume</b>, you can include this parameter, and specify either <code>true</code> for an encrypted volume, or <code>false</code> for an unencrypted volume. If you omit this parameter, it defaults to <code>false</code> (unencrypted).</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted or unencrypted snapshot</b>, you must omit this parameter. If you include this parameter, the request will fail, regardless of the value that you specify.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing unencrypted volume</b>, you can include this parameter, but you must specify <code>false</code>. If you specify <code>true</code>, the request will fail. In this case, we recommend that you omit the parameter.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted volume</b>, you can include this parameter, and specify either <code>true</code> or <code>false</code>. However, if you specify <code>false</code>, the parameter is ignored and the block device mapping is always encrypted. In this case, we recommend that you omit the parameter.</p> </li>
    /// </ul>
    pub fn encrypted(&self) -> std::option::Option<bool> {
        self.encrypted
    }
}
impl EbsBlockDevice {
    /// Creates a new builder-style object to manufacture [`EbsBlockDevice`](crate::types::EbsBlockDevice).
    pub fn builder() -> crate::types::builders::EbsBlockDeviceBuilder {
        crate::types::builders::EbsBlockDeviceBuilder::default()
    }
}

/// This is the datatype returned when calling `Builder::build()`.
#[allow(dead_code)]
pub type OutputShape = crate::types::EbsBlockDevice;
/// A builder for [`EbsBlockDevice`](crate::types::EbsBlockDevice).
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-serialize"),
    derive(serde::Serialize)
)]
#[cfg_attr(
    all(aws_sdk_unstable, feature = "serde-deserialize"),
    derive(serde::Deserialize)
)]
pub struct EbsBlockDeviceBuilder {
    pub(crate) delete_on_termination: std::option::Option<bool>,
    pub(crate) iops: std::option::Option<i32>,
    pub(crate) snapshot_id: std::option::Option<std::string::String>,
    pub(crate) volume_size: std::option::Option<i32>,
    pub(crate) volume_type: std::option::Option<crate::types::VolumeType>,
    pub(crate) kms_key_id: std::option::Option<std::string::String>,
    pub(crate) throughput: std::option::Option<i32>,
    pub(crate) outpost_arn: std::option::Option<std::string::String>,
    pub(crate) encrypted: std::option::Option<bool>,
}
impl EbsBlockDeviceBuilder {
    /// <p>Indicates whether the EBS volume is deleted on instance termination. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#preserving-volumes-on-termination">Preserving Amazon EBS volumes on instance termination</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn delete_on_termination(mut self, input: bool) -> Self {
        self.delete_on_termination = Some(input);
        self
    }
    /// <p>Indicates whether the EBS volume is deleted on instance termination. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/terminating-instances.html#preserving-volumes-on-termination">Preserving Amazon EBS volumes on instance termination</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_delete_on_termination(mut self, input: std::option::Option<bool>) -> Self {
        self.delete_on_termination = input;
        self
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000-16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100-64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100-64,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io1</code> and <code>io2</code> volumes, we guarantee 64,000 IOPS only for <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a>. Other instance families guarantee performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn iops(mut self, input: i32) -> Self {
        self.iops = Some(input);
        self
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000-16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100-64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100-64,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io1</code> and <code>io2</code> volumes, we guarantee 64,000 IOPS only for <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a>. Other instance families guarantee performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn set_iops(mut self, input: std::option::Option<i32>) -> Self {
        self.iops = input;
        self
    }
    /// <p>The ID of the snapshot.</p>
    pub fn snapshot_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.snapshot_id = Some(input.into());
        self
    }
    /// <p>The ID of the snapshot.</p>
    pub fn set_snapshot_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>:1-16,384</p> </li>
    /// <li> <p> <code>io1</code> and <code>io2</code>: 4-16,384</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125-16,384</p> </li>
    /// <li> <p> <code>standard</code>: 1-1,024</p> </li>
    /// </ul>
    pub fn volume_size(mut self, input: i32) -> Self {
        self.volume_size = Some(input);
        self
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>:1-16,384</p> </li>
    /// <li> <p> <code>io1</code> and <code>io2</code>: 4-16,384</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125-16,384</p> </li>
    /// <li> <p> <code>standard</code>: 1-1,024</p> </li>
    /// </ul>
    pub fn set_volume_size(mut self, input: std::option::Option<i32>) -> Self {
        self.volume_size = input;
        self
    }
    /// <p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon EC2 User Guide</i>. If the volume type is <code>io1</code> or <code>io2</code>, you must specify the IOPS that the volume supports.</p>
    pub fn volume_type(mut self, input: crate::types::VolumeType) -> Self {
        self.volume_type = Some(input);
        self
    }
    /// <p>The volume type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon EC2 User Guide</i>. If the volume type is <code>io1</code> or <code>io2</code>, you must specify the IOPS that the volume supports.</p>
    pub fn set_volume_type(mut self, input: std::option::Option<crate::types::VolumeType>) -> Self {
        self.volume_type = input;
        self
    }
    /// <p>Identifier (key ID, key alias, ID ARN, or alias ARN) for a customer managed CMK under which the EBS volume is encrypted.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotFleet.html">RequestSpotFleet</a>, and <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a>.</p>
    pub fn kms_key_id(mut self, input: impl Into<std::string::String>) -> Self {
        self.kms_key_id = Some(input.into());
        self
    }
    /// <p>Identifier (key ID, key alias, ID ARN, or alias ARN) for a customer managed CMK under which the EBS volume is encrypted.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RunInstances.html">RunInstances</a>, <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotFleet.html">RequestSpotFleet</a>, and <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_RequestSpotInstances.html">RequestSpotInstances</a>.</p>
    pub fn set_kms_key_id(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The throughput that the volume supports, in MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn throughput(mut self, input: i32) -> Self {
        self.throughput = Some(input);
        self
    }
    /// <p>The throughput that the volume supports, in MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn set_throughput(mut self, input: std::option::Option<i32>) -> Self {
        self.throughput = input;
        self
    }
    /// <p>The ARN of the Outpost on which the snapshot is stored.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateImage.html"> CreateImage</a>.</p>
    pub fn outpost_arn(mut self, input: impl Into<std::string::String>) -> Self {
        self.outpost_arn = Some(input.into());
        self
    }
    /// <p>The ARN of the Outpost on which the snapshot is stored.</p>
    /// <p>This parameter is only supported on <code>BlockDeviceMapping</code> objects called by <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateImage.html"> CreateImage</a>.</p>
    pub fn set_outpost_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>Indicates whether the encryption state of an EBS volume is changed while being restored from a backing snapshot. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-parameters">Amazon EBS encryption</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>In no case can you remove encryption from an encrypted volume.</p>
    /// <p>Encrypted volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    /// <p>This parameter is not returned by <code>DescribeImageAttribute</code>.</p>
    /// <p>For <code>CreateImage</code> and <code>RegisterImage</code>, whether you can include this parameter, and the allowed values differ depending on the type of block device mapping you are creating.</p>
    /// <ul>
    /// <li> <p>If you are creating a block device mapping for a <b>new (empty) volume</b>, you can include this parameter, and specify either <code>true</code> for an encrypted volume, or <code>false</code> for an unencrypted volume. If you omit this parameter, it defaults to <code>false</code> (unencrypted).</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted or unencrypted snapshot</b>, you must omit this parameter. If you include this parameter, the request will fail, regardless of the value that you specify.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing unencrypted volume</b>, you can include this parameter, but you must specify <code>false</code>. If you specify <code>true</code>, the request will fail. In this case, we recommend that you omit the parameter.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted volume</b>, you can include this parameter, and specify either <code>true</code> or <code>false</code>. However, if you specify <code>false</code>, the parameter is ignored and the block device mapping is always encrypted. In this case, we recommend that you omit the parameter.</p> </li>
    /// </ul>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = Some(input);
        self
    }
    /// <p>Indicates whether the encryption state of an EBS volume is changed while being restored from a backing snapshot. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-parameters">Amazon EBS encryption</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>In no case can you remove encryption from an encrypted volume.</p>
    /// <p>Encrypted volumes can only be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    /// <p>This parameter is not returned by <code>DescribeImageAttribute</code>.</p>
    /// <p>For <code>CreateImage</code> and <code>RegisterImage</code>, whether you can include this parameter, and the allowed values differ depending on the type of block device mapping you are creating.</p>
    /// <ul>
    /// <li> <p>If you are creating a block device mapping for a <b>new (empty) volume</b>, you can include this parameter, and specify either <code>true</code> for an encrypted volume, or <code>false</code> for an unencrypted volume. If you omit this parameter, it defaults to <code>false</code> (unencrypted).</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted or unencrypted snapshot</b>, you must omit this parameter. If you include this parameter, the request will fail, regardless of the value that you specify.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing unencrypted volume</b>, you can include this parameter, but you must specify <code>false</code>. If you specify <code>true</code>, the request will fail. In this case, we recommend that you omit the parameter.</p> </li>
    /// <li> <p>If you are creating a block device mapping from an <b>existing encrypted volume</b>, you can include this parameter, and specify either <code>true</code> or <code>false</code>. However, if you specify <code>false</code>, the parameter is ignored and the block device mapping is always encrypted. In this case, we recommend that you omit the parameter.</p> </li>
    /// </ul>
    pub fn set_encrypted(mut self, input: std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// Consumes the builder and constructs a [`EbsBlockDevice`](crate::types::EbsBlockDevice).
    pub fn build(self) -> crate::types::EbsBlockDevice {
        crate::types::EbsBlockDevice {
            delete_on_termination: self.delete_on_termination,
            iops: self.iops,
            snapshot_id: self.snapshot_id,
            volume_size: self.volume_size,
            volume_type: self.volume_type,
            kms_key_id: self.kms_key_id,
            throughput: self.throughput,
            outpost_arn: self.outpost_arn,
            encrypted: self.encrypted,
        }
    }
}

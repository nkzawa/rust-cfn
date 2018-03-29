/// The [`AWS::Batch::ComputeEnvironment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-computeenvironment.html) resource type.
pub struct ComputeEnvironment {
    properties: ComputeEnvironmentProperties
}

/// Properties for the `ComputeEnvironment` resource.
#[derive(Serialize, Deserialize)]
pub struct ComputeEnvironmentProperties {
    #[serde(rename="ComputeEnvironmentName")]
    pub compute_environment_name: String,
    #[serde(rename="ComputeResources")]
    pub compute_resources: self::compute_environment::ComputeResources,
    #[serde(rename="ServiceRole")]
    pub service_role: String,
    #[serde(rename="State")]
    pub state: String,
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for ComputeEnvironment {
    type Properties = ComputeEnvironmentProperties;
    const TYPE: &'static str = "AWS::Batch::ComputeEnvironment";
    fn properties(&self) -> &ComputeEnvironmentProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut ComputeEnvironmentProperties {
        &mut self.properties
    }
}

impl From<ComputeEnvironmentProperties> for ComputeEnvironment {
    fn from(properties: ComputeEnvironmentProperties) -> ComputeEnvironment {
        ComputeEnvironment { properties }
    }
}

/// The [`AWS::Batch::JobDefinition`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobdefinition.html) resource type.
pub struct JobDefinition {
    properties: JobDefinitionProperties
}

/// Properties for the `JobDefinition` resource.
#[derive(Serialize, Deserialize)]
pub struct JobDefinitionProperties {
    #[serde(rename="ContainerProperties")]
    pub container_properties: self::job_definition::ContainerProperties,
    #[serde(rename="JobDefinitionName")]
    pub job_definition_name: String,
    #[serde(rename="Parameters")]
    pub parameters: ::json::Value,
    #[serde(rename="RetryStrategy")]
    pub retry_strategy: self::job_definition::RetryStrategy,
    #[serde(rename="Type")]
    pub type_: String,
}

impl<'a> ::Resource<'a> for JobDefinition {
    type Properties = JobDefinitionProperties;
    const TYPE: &'static str = "AWS::Batch::JobDefinition";
    fn properties(&self) -> &JobDefinitionProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobDefinitionProperties {
        &mut self.properties
    }
}

impl From<JobDefinitionProperties> for JobDefinition {
    fn from(properties: JobDefinitionProperties) -> JobDefinition {
        JobDefinition { properties }
    }
}

/// The [`AWS::Batch::JobQueue`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-batch-jobqueue.html) resource type.
pub struct JobQueue {
    properties: JobQueueProperties
}

/// Properties for the `JobQueue` resource.
#[derive(Serialize, Deserialize)]
pub struct JobQueueProperties {
    #[serde(rename="ComputeEnvironmentOrder")]
    pub compute_environment_order: Vec<self::job_queue::ComputeEnvironmentOrder>,
    #[serde(rename="JobQueueName")]
    pub job_queue_name: String,
    #[serde(rename="Priority")]
    pub priority: u32,
    #[serde(rename="State")]
    pub state: String,
}

impl<'a> ::Resource<'a> for JobQueue {
    type Properties = JobQueueProperties;
    const TYPE: &'static str = "AWS::Batch::JobQueue";
    fn properties(&self) -> &JobQueueProperties {
        &self.properties
    }
    fn properties_mut(&mut self) -> &mut JobQueueProperties {
        &mut self.properties
    }
}

impl From<JobQueueProperties> for JobQueue {
    fn from(properties: JobQueueProperties) -> JobQueue {
        JobQueue { properties }
    }
}

pub mod compute_environment {
    /// The [`AWS::Batch::ComputeEnvironment.ComputeResources`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-computeenvironment-computeresources.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ComputeResources {
        #[serde(rename="BidPercentage")]
        pub bid_percentage: u32,
        #[serde(rename="DesiredvCpus")]
        pub desiredv_cpus: u32,
        #[serde(rename="Ec2KeyPair")]
        pub ec2_key_pair: String,
        #[serde(rename="ImageId")]
        pub image_id: String,
        #[serde(rename="InstanceRole")]
        pub instance_role: String,
        #[serde(rename="InstanceTypes")]
        pub instance_types: Vec<String>,
        #[serde(rename="MaxvCpus")]
        pub maxv_cpus: u32,
        #[serde(rename="MinvCpus")]
        pub minv_cpus: u32,
        #[serde(rename="SecurityGroupIds")]
        pub security_group_ids: Vec<String>,
        #[serde(rename="SpotIamFleetRole")]
        pub spot_iam_fleet_role: String,
        #[serde(rename="Subnets")]
        pub subnets: Vec<String>,
        #[serde(rename="Tags")]
        pub tags: ::json::Value,
        #[serde(rename="Type")]
        pub type_: String,
    }

}

pub mod job_definition {
    /// The [`AWS::Batch::JobDefinition.ContainerProperties`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-containerproperties.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ContainerProperties {
        #[serde(rename="Command")]
        pub command: Vec<String>,
        #[serde(rename="Environment")]
        pub environment: Vec<Environment>,
        #[serde(rename="Image")]
        pub image: String,
        #[serde(rename="JobRoleArn")]
        pub job_role_arn: String,
        #[serde(rename="Memory")]
        pub memory: u32,
        #[serde(rename="MountPoints")]
        pub mount_points: Vec<MountPoints>,
        #[serde(rename="Privileged")]
        pub privileged: bool,
        #[serde(rename="ReadonlyRootFilesystem")]
        pub readonly_root_filesystem: bool,
        #[serde(rename="Ulimits")]
        pub ulimits: Vec<Ulimit>,
        #[serde(rename="User")]
        pub user: String,
        #[serde(rename="Vcpus")]
        pub vcpus: u32,
        #[serde(rename="Volumes")]
        pub volumes: Vec<Volumes>,
    }

    /// The [`AWS::Batch::JobDefinition.Environment`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-environment.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Environment {
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="Value")]
        pub value: String,
    }

    /// The [`AWS::Batch::JobDefinition.MountPoints`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-mountpoints.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct MountPoints {
        #[serde(rename="ContainerPath")]
        pub container_path: String,
        #[serde(rename="ReadOnly")]
        pub read_only: bool,
        #[serde(rename="SourceVolume")]
        pub source_volume: String,
    }

    /// The [`AWS::Batch::JobDefinition.RetryStrategy`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-retrystrategy.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct RetryStrategy {
        #[serde(rename="Attempts")]
        pub attempts: u32,
    }

    /// The [`AWS::Batch::JobDefinition.Ulimit`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-ulimit.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Ulimit {
        #[serde(rename="HardLimit")]
        pub hard_limit: u32,
        #[serde(rename="Name")]
        pub name: String,
        #[serde(rename="SoftLimit")]
        pub soft_limit: u32,
    }

    /// The [`AWS::Batch::JobDefinition.Volumes`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumes.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct Volumes {
        #[serde(rename="Host")]
        pub host: VolumesHost,
        #[serde(rename="Name")]
        pub name: String,
    }

    /// The [`AWS::Batch::JobDefinition.VolumesHost`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobdefinition-volumeshost.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct VolumesHost {
        #[serde(rename="SourcePath")]
        pub source_path: String,
    }

}

pub mod job_queue {
    /// The [`AWS::Batch::JobQueue.ComputeEnvironmentOrder`](http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-batch-jobqueue-computeenvironmentorder.html) property type.
    #[derive(Serialize, Deserialize)]
    pub struct ComputeEnvironmentOrder {
        #[serde(rename="ComputeEnvironment")]
        pub compute_environment: String,
        #[serde(rename="Order")]
        pub order: u32,
    }

}

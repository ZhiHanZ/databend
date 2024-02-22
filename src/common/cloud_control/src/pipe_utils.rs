// Copyright 2021 Datafuse Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use chrono::{DateTime, TimeZone, Utc};
use databend_common_exception::ErrorCode;
use databend_common_exception::Result;

pub struct StageFileInfo {
    pub path: Option<String>,
    pub pattern: Option<String>,
}
pub struct StageInfo {
    pub stage_name: String,
    pub stage_type: String,
    // stageFileInfo
    pub stage_file_info: StageFileInfo,
}

impl TryFrom<crate::pb::StageInfo> for StageInfo {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::StageInfo) -> databend_common_exception::Result<Self> {
        Ok(StageInfo {
            stage_name: value.stage_name,
            stage_type: value.stage_type,
            stage_file_info: StageFileInfo{
                path: value.path,
                pattern: value.pattern,
            },
        })
    }
}

pub struct SourceOptions {
    pub stage_info: StageInfo
}

impl TryFrom<crate::pb::SourceOptions> for SourceOptions {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::SourceOptions) -> databend_common_exception::Result<Self> {
        let stage_info = value.stage_info.ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat("pipe source options is missing".to_string()))?;
        let stage_info = StageInfo::try_from(stage_info)?;
        Ok(SourceOptions {
            stage_info
        })
    }
}

pub enum FileDetectionOptions {
    None,
}

impl TryFrom<crate::pb::FileDetectionOptions> for FileDetectionOptions {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::FileDetectionOptions) -> databend_common_exception::Result<Self> {
        crate::pb::FileDetectionMode::try_from(value.mode)
            .and_then(|file_detection_mode| {
                match file_detection_mode {
                    crate::pb::FileDetectionMode::None => {
                        Ok(FileDetectionOptions::None)
                    }
                }
            }).map_err(|_| ErrorCode::IllegalCloudControlMessageFormat(format!("Unknown file detection mode: {}", value.mode)))
    }
}

pub struct TargetOptions {
    pub target_database: String,
    pub target_table: String,
}

impl TryFrom<crate::pb::TargetOptions> for TargetOptions {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::TargetOptions) -> databend_common_exception::Result<Self> {
        Ok(TargetOptions {
            target_database: value.target_database,
            target_table: value.target_table,
        })
    }
}
pub enum PipeState {
    RUNNING,
    STOP,
    STARTED,
}

impl TryFrom<crate::pb::pipe_status::Status> for PipeState {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::pipe_status::Status) -> Result<Self> {
        match value {
            crate::pb::pipe_status::Status::Running => Ok(PipeState::RUNNING),
            crate::pb::pipe_status::Status::Stop => Ok(PipeState::STOP),
            crate::pb::pipe_status::Status::Started => Ok(PipeState::STARTED),
        }
    }
}

impl TryFrom<i32> for PipeState {
    type Error = ErrorCode;

    fn try_from(value: i32) -> Result<Self> {
        crate::pb::pipe_status::Status::from_i32(value)
            .ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat(format!("Unknown pipe status: {}", value)))
            .and_then(|status| PipeState::try_from(status))
    }
}
pub struct PipeStats {
    pub pipe_id: i64,
    pub total_count: i64,
    pub success_count: i64,
    pub processing_count: i64,
    pub failed_count: i64,
    pub pending_count: i64,
    pub queuing_count: i64,
    pub message: String,
}

impl TryFrom<crate::pb::PipeStats> for PipeStats {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::PipeStats) -> databend_common_exception::Result<Self> {
        Ok(PipeStats {
            pipe_id: value.pipe_id,
            total_count: value.total_count,
            success_count: value.success_count,
            processing_count: value.processing_count,
            failed_count: value.failed_count,
            pending_count: value.pending_count,
            queuing_count: value.queuing_count,
            message: value.message,
        })
    }
}

pub struct PipeStatus {
    pub pipe_name: String,
    pub definition: String,
    pub source_options: SourceOptions,
    pub file_detection_options: FileDetectionOptions,
    pub target_options: TargetOptions,
    pub status: PipeState,
    pub stats: PipeStats,
    pub warehouse_name: String,
    pub created_time: DateTime<Utc>,
    pub created_by: String,
    pub updated_time: DateTime<Utc>,
    pub updated_by: String,
}

fn timestamp_to_datetime_utc(ts: Option<prost_types::Timestamp>) -> Result<DateTime<Utc>> {
    if ts.is_none() {
        return Err(ErrorCode::IllegalCloudControlMessageFormat("Timestamp is missing".to_string()));
    }
    let ts = ts.unwrap();
    // Convert the seconds and nanoseconds from the Timestamp into a DateTime<Utc>
    return Utc.timestamp_opt(ts.seconds, ts.nanos as u32).single().ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat(format!("Invalid timestamp: {:?}", ts)));
}

impl TryFrom<crate::pb::PipeStatus> for PipeStatus {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::PipeStatus) -> databend_common_exception::Result<Self> {
        let source_options = value.source_options.ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat("pipe source options is missing".to_string()))?;
        let source_options = SourceOptions::try_from(source_options)?;
        let file_detection_options = value.file_detection_options.ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat("pipe file detection options is missing".to_string()))?;
        let file_detection_options = FileDetectionOptions::try_from(file_detection_options)?;
        let target_options = value.target_options.ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat("pipe target options is missing".to_string()))?;
        let target_options = TargetOptions::try_from(target_options)?;
        let stats = value.stats.ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat("pipe stats is missing".to_string()))?;
        let stats = PipeStats::try_from(stats)?;
        let created_time = timestamp_to_datetime_utc(value.created_time)?;
        let updated_time = timestamp_to_datetime_utc(value.updated_time)?;
        let status = PipeState::try_from(value.status).map_err(|_| ErrorCode::IllegalCloudControlMessageFormat(format!("Unknown pipe status: {}", value.status)))?;
        Ok(PipeStatus {
            pipe_name: value.pipe_name,
            definition: value.definition,
            source_options,
            file_detection_options,
            target_options,
            status,
            stats,
            warehouse_name: value.warehouse_name,
            created_time,
            created_by: value.created_by,
            updated_time,
            updated_by: value.updated_by,
        })
    }
}

pub enum PipeTaskState {
    Pending,
    Queuing,
    Processing,
    Success,
    Failed,
}

impl TryFrom<crate::pb::pipe_task::Status> for PipeTaskState {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::pipe_task::Status) -> Result<Self> {
        match value {
            crate::pb::pipe_task::Status::Pending => Ok(PipeTaskState::Pending),
            crate::pb::pipe_task::Status::Queuing => Ok(PipeTaskState::Queuing),
            crate::pb::pipe_task::Status::Processing => Ok(PipeTaskState::Processing),
            crate::pb::pipe_task::Status::Success => Ok(PipeTaskState::Success),
            crate::pb::pipe_task::Status::Failed => Ok(PipeTaskState::Failed),
        }
    }
}

impl TryFrom<i32> for PipeTaskState {
    type Error = ErrorCode;

    fn try_from(value: i32) -> Result<Self> {
        crate::pb::pipe_task::Status::from_i32(value)
            .ok_or_else(|| ErrorCode::IllegalCloudControlMessageFormat(format!("Unknown pipe task status: {}", value)))
            .and_then(|status| PipeTaskState::try_from(status))
    }
}

pub struct PipeTask {
    pub task_id: u64,
    pub tenant_id: String,
    pub pipe_name: String,
    pub stage: String,
    pub file: String,
    pub file_size: u64,
    pub status: PipeTaskState,
    pub duration: i64,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub error: Option<PipeError>,
}

impl TryFrom<crate::pb::PipeTask> for PipeTask {
    type Error = ErrorCode;

    fn try_from(value: crate::pb::PipeTask) -> databend_common_exception::Result<Self> {
        let started_at = timestamp_to_datetime_utc(value.started_at)?;
        let finished_at = timestamp_to_datetime_utc(value.finished_at)?;
        let status = PipeTaskState::try_from(value.status).map_err(|_| ErrorCode::IllegalCloudControlMessageFormat(format!("Unknown pipe task status: {}", value.status)))?;
        Ok(PipeTask {
            task_id: value.task_id,
            tenant_id: value.tenant_id,
            pipe_name: value.pipe_name,
            stage: value.stage,
            file: value.file,
            file_size: value.file_size,
            duration: value.duration,
            status,
            error: value.error.map(|e| PipeError {
                kind: e.kind,
                message: e.message,
                code: e.code,
            }),
            started_at,
            finished_at,
        })
    }
}

pub struct PipeError {
    pub kind: String,
    pub message: String,
    pub code: i32,
}
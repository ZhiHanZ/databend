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

use std::sync::Arc;
use std::time::Duration;

use databend_common_catalog::table_context::TableContext;
use databend_common_cloud_control::client_config::build_client_config;
use databend_common_cloud_control::client_config::ClientConfig;
use databend_common_cloud_control::pb::{SourceOptions, StageInfo};
use databend_common_exception::{ErrorCode, Result};
use databend_common_sql::plans::CopyIntoTablePlan;

use crate::sessions::QueryContext;

pub fn get_target_options(
    plan: &Box<CopyIntoTablePlan>,
) -> databend_common_cloud_control::pb::TargetOptions {
    databend_common_cloud_control::pb::TargetOptions {
        target_database: plan.database_name.clone(),
        target_table: plan.table_name.clone(),
    }
}

pub fn get_source_options(
    plan: &Box<CopyIntoTablePlan>,
) -> SourceOptions {
    let stage_file_info = &plan.stage_table_info.files_info;
    let path = if stage_file_info.path.is_empty() {
        None
    } else {
        Some(stage_file_info.path.clone())
    };
    let pattern = stage_file_info.pattern.clone();
    let stage_info = &plan.stage_table_info.stage_info;
    return SourceOptions {
        stage_info: Some(StageInfo {
            stage_name: stage_info.stage_name.clone(),
            stage_type: stage_info.stage_type.to_string(),
            path,
            pattern,
        }),
    };
}

pub fn get_client_config(ctx: Arc<QueryContext>, timeout: Duration) -> Result<ClientConfig> {
    let tenant = ctx.get_tenant();
    let user = ctx.get_current_user()?.identity().to_string();
    let query_id = ctx.get_id();

    Ok(build_client_config(tenant, user, query_id, timeout))
}

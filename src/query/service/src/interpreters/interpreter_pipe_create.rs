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

use databend_common_cloud_control::cloud_api::CloudControlApiProvider;
use databend_common_cloud_control::pb::CreatePipeRequest;
use databend_common_cloud_control::task_client::make_request;
use databend_common_config::GlobalConfig;
use databend_common_exception::ErrorCode;
use databend_common_exception::Result;
use databend_common_sql::plans::CreatePipePlan;

use crate::interpreters::common::{get_source_options, get_target_options};
use crate::interpreters::common::cloud::get_client_config;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

#[derive(Debug)]
pub struct CreatePipeInterpreter {
    ctx: Arc<QueryContext>,
    plan: CreatePipePlan,
}

impl CreatePipeInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: CreatePipePlan) -> Result<Self> {
        Ok(CreatePipeInterpreter { ctx, plan })
    }
}

impl CreatePipeInterpreter {
    // todo owner info add-on
    fn build_request(&self) -> CreatePipeRequest {
        let plan = self.plan.clone();
        CreatePipeRequest {
            pipe_name: plan.pipe_name,
            tenant_id: plan.tenant,
            source_options: Some(get_source_options(&plan.copy_stmt)),
            target_options: Some(get_target_options(&plan.copy_stmt)),
            file_detection_options: None,
            warehouse_name: plan.warehouse_name,
            definition: plan.definition,
        }
    }
}

#[async_trait::async_trait]
impl Interpreter for CreatePipeInterpreter {
    fn name(&self) -> &str {
        "CreatePipeInterpreter"
    }

    #[minitrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let config = GlobalConfig::instance();
        if config.query.cloud_control_grpc_server_address.is_none() {
            return Err(ErrorCode::CloudControlNotEnabled(
                "cannot create task without cloud control enabled, please set cloud_control_grpc_server_address in config",
            ));
        }
        let cloud_api = CloudControlApiProvider::instance();
        let pipe_client = cloud_api.get_pipe_client();
        let req = self.build_request();
        let config = get_client_config(self.ctx.clone(), cloud_api.get_timeout())?;
        let req = make_request(req, config);
        pipe_client.create_pipe(req).await?;
        Ok(PipelineBuildResult::create())
    }
}

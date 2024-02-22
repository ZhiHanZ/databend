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


use std::fmt::format;
use databend_common_ast::ast::{CopyIntoTableStmt, CreatePipeStmt};
use databend_common_ast::Dialect;
use databend_common_ast::parser::{parse_sql, tokenize_sql};
use databend_common_exception::ErrorCode;
use databend_common_exception::Result;
use databend_common_meta_app::principal::StageType;
use databend_common_users::UserApiProvider;

use crate::plans::{CopyIntoTablePlan, CreatePipePlan};
use crate::plans::Plan;
use crate::{BindContext, Binder};

fn verify_pipe_sql(sql: &String) -> Result<()> {
    let tokens = tokenize_sql(sql.as_str()).map_err(|e| {
        ErrorCode::SyntaxException(format!(
            "syntax error for pipe formatted sql: {}, error: {:?}",
            sql, e
        ))
    })?;
    parse_sql(&tokens, Dialect::PostgreSQL).map_err(|e| {
        ErrorCode::SyntaxException(format!(
            "syntax error for pipe formatted sql: {}, error: {:?}",
            sql, e
        ))
    })?;
    Ok(())
}

#[async_backtrace::framed]
async fn verify_copy_plan(tenant: &String, plan: &Box<CopyIntoTablePlan>) -> Result<()> {
    if plan.stage_table_info.stage_info.stage_type != StageType::External {
        return Err(ErrorCode::IllegalCloudControlMessageFormat(
            format!("Currently pipe only support external stage, Invalid pipe stage type: {:?}", plan.stage_table_info.stage_info.stage_type),
        ));
    }
    let user_mgr = UserApiProvider::instance();
    let stage = user_mgr.get_stage(tenant, plan.stage_table_info.stage_info.stage_name.as_str()).await.map_err(|e| {
        ErrorCode::IllegalCloudControlMessageFormat(format!("Pipe stage is unavailable, Failed to get stage: {}", e))
    })?;
    if stage.stage_type != StageType::External {
        return Err(ErrorCode::IllegalCloudControlMessageFormat(
            format!("Currently pipe only support external stage, Invalid pipe stage type: {:?}", stage.stage_type),
        ));
    }
    Ok(())
}
fn rewrite_copy_statement(stmt: &CopyIntoTableStmt) -> Result<CopyIntoTableStmt> {
    let mut stmt = stmt.clone();
    stmt.files = None;
    let candidate = stmt.clone();
    let formatted = format!("{} FILES = ('books.parquet')", candidate);
    verify_pipe_sql(&formatted)?;
    let formatted_case_2 = format!("{} FILES = ('books.parquet', 'books2.parquet')", candidate);
    verify_pipe_sql(&formatted_case_2)?;
    Ok(stmt)
}
impl Binder {
    #[async_backtrace::framed]
    pub(in crate::planner::binder) async fn bind_create_pipe(
        &mut self,
        bind_context: &mut BindContext,
        stmt: &CreatePipeStmt,
    ) -> Result<Plan> {
        let CreatePipeStmt {
            if_not_exists,
            name,
            auto_ingest,
            comments,
            copy_stmt,
        } = stmt;
        let tenant = self.ctx.get_tenant();
        let rewrite_copy_stmt = rewrite_copy_statement(copy_stmt)?;
        let description = rewrite_copy_stmt.to_string();

        let copy_stmt = self.bind_copy_into_table(bind_context, copy_stmt).await?;
        let copy_stmt = match copy_stmt {
            Plan::CopyIntoTable(plan) => plan,
            _ => {
                return Err(ErrorCode::SyntaxException(
                    "Create pipe statement must be followed by a copy statement".to_string(),
                ));
            }
        };
        verify_copy_plan(&tenant, &copy_stmt).await?;
        // rewrite and validate the copy statement
        let plan = CreatePipePlan {
            if_not_exists: *if_not_exists,
            tenant,
            pipe_name: name.to_string(),
            auto_ingest: *auto_ingest,
            comment: comments.to_string(),
            copy_stmt: copy_stmt.clone(),
            definition: description,
            warehouse_name: None,
        };
        Ok(Plan::CreatePipe(Box::new(plan)))
    }


}

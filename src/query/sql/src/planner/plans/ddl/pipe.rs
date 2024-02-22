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

use databend_common_expression::DataSchemaRef;
use databend_common_expression::DataSchemaRefExt;
use crate::plans::{CopyIntoTablePlan};


#[derive(Clone, Debug)]
pub struct CreatePipePlan {
    pub if_not_exists: bool,
    pub tenant: String,
    pub pipe_name: String,
    pub auto_ingest: bool,
    pub copy_stmt: Box<CopyIntoTablePlan>,
    pub definition: String,
    pub warehouse_name: Option<String>,
    pub comment: String,
}

impl CreatePipePlan {
    pub fn schema(&self) -> DataSchemaRef {
        DataSchemaRefExt::create(vec![])
    }
}

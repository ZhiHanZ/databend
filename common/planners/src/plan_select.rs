// Copyright 2020-2021 The Datafuse Authors.
//
// SPDX-License-Identifier: Apache-2.0.

use std::sync::Arc;

use common_datavalues::DataSchemaRef;

use crate::PlanNode;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SelectPlan {
    pub input: Arc<PlanNode>
}

impl SelectPlan {
    pub fn schema(&self) -> DataSchemaRef {
        self.input.schema()
    }

    pub fn child(&self) -> Arc<PlanNode> {
        self.input.clone()
    }

    pub fn set_child(&mut self, input: &PlanNode) {
        self.input = Arc::new(input.clone());
    }
}

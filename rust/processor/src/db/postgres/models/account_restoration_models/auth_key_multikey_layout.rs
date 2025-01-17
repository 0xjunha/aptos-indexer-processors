// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use crate::schema::auth_key_multikey_layout::{self};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, FieldCount, Identifiable, Insertable, Serialize)]
#[diesel(primary_key(auth_key))]
#[diesel(table_name = auth_key_multikey_layout)]
pub struct AuthKeyMultikeyLayout {
    pub auth_key: String,
    pub signatures_required: i64,
    pub multikey_layout_with_prefixes: serde_json::Value,
    pub multikey_type: String,
}

use sea_orm::{Condition, DynIden, Iden, Identity, RelationDef, entity::prelude::SeaRc};
use sea_query::{ConditionType, Expr, TableRef};

#[allow(clippy::needless_pass_by_value)]
pub(crate) fn join_tbl_on_condition(
    from_tbl: SeaRc<dyn Iden>,
    to_tbl: SeaRc<dyn Iden>,
    owner_keys: Identity,
    foreign_keys: Identity,
) -> Condition {
    let mut cond = Condition::all();
    for (owner_key, foreign_key) in owner_keys.into_iter().zip(foreign_keys.into_iter()) {
        cond = cond.add(
            Expr::col((SeaRc::clone(&from_tbl), owner_key))
                .equals((SeaRc::clone(&to_tbl), foreign_key)),
        );
    }
    cond
}

pub(crate) fn unpack_table_ref(table_ref: &TableRef) -> DynIden {
    match table_ref {
        TableRef::Table(tbl)
        | TableRef::SchemaTable(_, tbl)
        | TableRef::DatabaseSchemaTable(_, _, tbl)
        | TableRef::TableAlias(tbl, _)
        | TableRef::SchemaTableAlias(_, tbl, _)
        | TableRef::DatabaseSchemaTableAlias(_, _, tbl, _)
        | TableRef::SubQuery(_, tbl)
        | TableRef::ValuesList(_, tbl)
        | TableRef::FunctionCall(_, tbl) => SeaRc::clone(tbl),
    }
}

pub(crate) fn unpack_table_alias(table_ref: &TableRef) -> Option<DynIden> {
    match table_ref {
        TableRef::Table(_)
        | TableRef::SchemaTable(_, _)
        | TableRef::DatabaseSchemaTable(_, _, _)
        | TableRef::SubQuery(_, _)
        | TableRef::ValuesList(_, _) => None,
        TableRef::TableAlias(_, alias)
        | TableRef::SchemaTableAlias(_, _, alias)
        | TableRef::DatabaseSchemaTableAlias(_, _, _, alias)
        | TableRef::FunctionCall(_, alias) => Some(SeaRc::clone(alias)),
    }
}

pub(crate) fn join_condition(mut rel: RelationDef) -> Condition {
    // Use table alias (if any) to construct the join condition
    let from_tbl = match unpack_table_alias(&rel.from_tbl) {
        Some(alias) => alias,
        None => unpack_table_ref(&rel.from_tbl),
    };
    let to_tbl = match unpack_table_alias(&rel.to_tbl) {
        Some(alias) => alias,
        None => unpack_table_ref(&rel.to_tbl),
    };
    let owner_keys = rel.from_col;
    let foreign_keys = rel.to_col;

    let mut condition = match rel.condition_type {
        ConditionType::All => Condition::all(),
        ConditionType::Any => Condition::any(),
    };

    condition = condition.add(join_tbl_on_condition(
        SeaRc::clone(&from_tbl),
        SeaRc::clone(&to_tbl),
        owner_keys,
        foreign_keys,
    ));
    if let Some(f) = rel.on_condition.take() {
        condition = condition.add(f(from_tbl, to_tbl));
    }

    condition
}

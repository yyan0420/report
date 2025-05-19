use async_graphql::{Context, Object, Result};
#[derive(Debug, Clone)]
pub struct Planogram {
    pub id: i32,
    pub name: String,
}

// Make it GraphQL-compatible
#[async_graphql::Object]
impl Planogram {
    async fn id(&self) -> i32 {
        self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }
}

pub(crate) struct Mutation;

#[Object]
impl Mutation {
    async fn update_planogram(
        &self,
        _ctx: &Context<'_>,
    ) -> Result<Planogram> {
        Ok(Planogram {
            id: 42,
            name: "Dummy Planogram".to_string(),
        })
    }
}

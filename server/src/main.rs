use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode,
};

#[derive(GraphQLObject)]
/// A housemate
struct Housemate {
    /// The housemate's first name
    name: String,
    /// The housemate's age
    age: i32,
}

/// A house
struct House {
    /// The house's address
    address: Option<String>,
    /// The house's residents
    inhabitants: Vec<Housemate>,
}

struct Query;

#[graphql_object]
impl Query {
    // fn housemateWithName(name: String) -> FieldResult<Option<Housemate>> {
    //     // Look up user in database...
    // }
    fn hello(&self) -> FieldResult<&str> {
        Ok("hello world")
    }
}

fn main() {
    let schema = RootNode::new(
        Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );
}

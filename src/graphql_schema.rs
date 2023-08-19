extern crate dotenv;
use std::env;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use crate::schema::todos;

// import EmptyMutation and RootNode from juniper.
use juniper::{EmptyMutation,RootNode};


fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


// Define the structure of a todo by setting the fields of the Todo.
struct Todo{
    id:i32,
    title:String,
    completed:bool
}

// Describe the Todo object by defining what each field should return.
#[juniper::object(description="A todo")]
impl Todo{
    pub fn id(&self)->i32{
        self.id
    }

    pub fn title(&self)->&str{
        self.title.as_str()
    }

    pub fn completed(&self)->bool{
        self.completed
    }
}

// Next, define the root query.
pub struct QueryRoot;

// Implement the root query to return some dummy todos.
#[juniper::object]
impl QueryRoot {
    fn todos() -> Vec<Todo> {
        use crate::schema::todos::dsl::*;

        let connection = establish_connection();
        let results = todos.load::<Todo>(&connection).expect("Error loading todos");

        results
    }
}

// nitialize the schema with the root query and empty mutation.
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
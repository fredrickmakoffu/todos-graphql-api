// import EmptyMutation and RootNode from juniper.
use juniper::{EmptyMutation,RootNode};

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
        vec![
            Todo{
                id:1,
                title:"Code in Rust".to_string(),
                completed:false
            },
            Todo{
                id:2,
                title:"Cook supper meal".to_string(),
                completed:false
            }
        ]
    }
}

// nitialize the schema with the root query and empty mutation.
pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new())
}
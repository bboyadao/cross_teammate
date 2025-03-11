use async_trait::async_trait;
use derive_builder::Builder;
use uuid::{Uuid};
use serde::{Deserialize, Serialize};
use serde_json::Result as JsonResult;

pub struct CusUuid(pub Uuid);

uniffi::custom_type!(Uuid, String, {
    remote,
    try_lift: |val| Ok(Uuid::parse_str(&val)?),
    lower: |obj| obj.into(),
});

fn new_uuid() -> Uuid {
    Uuid::new_v4()
}

#[derive(Debug, Clone, Serialize, Deserialize, Builder)]
pub struct User {
    pub name: String,
    #[serde(default = "new_uuid")]
    pub id: Uuid,
}

#[async_trait]
pub trait ToJson {
    async fn to_json(&self) -> JsonResult<String>;
}
#[async_trait]
impl ToJson for User {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
impl User {
    pub fn new(name: &str, id: Option<Uuid>) -> Self {
        Self {
            name: name.to_string(),
            id: id.unwrap_or_else(new_uuid),
        }
    }
}
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Expense {
    pub amount: u64,
    pub user: User,
    pub paid: u64,
    #[builder(default)]
    pub have_to_pay: u64,

    #[builder(default)]
    pub need_to_earn: u64,
}

#[async_trait]
impl ToJson for Expense {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct Teammate {
    pub users: Vec<User>,
    pub expenses: Vec<Expense>,
}
impl Teammate {
    pub async fn anew(expenses: Vec<Expense>) -> Self{
        let mut users: Vec<User> = Vec::new();
        Teammate { users, expenses: expenses }
    }
    // pub async fn calculate(self) -> Result<Vec<Payment>, Box<dyn Error>> {
    //     todo!()
    // }

}

#[async_trait]
impl ToJson for Teammate {
    async fn to_json(&self) -> JsonResult<String> {
        serde_json::to_string(self)
    }
}
uniffi::include_scaffolding!("teammate");
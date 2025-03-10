use derive_builder::Builder;
use uuid::{Uuid};
use serde::{Deserialize, Serialize};

pub struct CusUuid(pub Uuid);

// uniffi::custom_type!(CusUuid, Uuid);
uniffi::custom_type!(Uuid, String, {
    // Remote is required since `Url` is from a different crate
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

impl User {
    // Corrected constructor
    pub fn new(name: &str, id: Option<Uuid>) -> Self {
        Self {
            name: name.to_string(),
            id: id.unwrap_or_else(new_uuid), // Default to new UUID if None
        }
    }
}
#[derive(Debug, Clone, Builder)]
pub struct Expense {
    amount: u64,
    user: User,
    paid: u64,
    #[builder(default)]
    pub have_to_pay: u64,

    #[builder(default)]
    pub need_to_earn: u64,
}


pub struct Payment {
    pub src: User,
    pub dst: User,
    pub amount: u64,
}

pub struct Teammate {
    pub users: Vec<User>,
    pub expense: Vec<Expense>,
}
impl Teammate {
    pub async fn anew(exps: Vec<Expense>) -> Self{
        let mut users: Vec<User> = Vec::new();

        // Collect unique users from expenses
        // for exp in &exps {
        //     if !users.iter().any(|u| u.name == exp.user) {
        //         users.push(User::new(&exp.user, None));
        //     }
        // }

        Teammate { users, expense: exps }
    }
    // pub async fn calculate(self) -> Result<Vec<Payment>, Box<dyn Error>> {
    //     todo!()
    // }
}
uniffi::include_scaffolding!("teammate");
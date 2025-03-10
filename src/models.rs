#[derive(Debug, Clone)]
pub struct Payment {
    pub src: PyObject,
    pub dst: PyObject,
    pub amount: i64,
}

#[derive(Debug, Clone, Builder)]
pub struct Act {
    pub user: PyObject,
    pub paid: i64,

    #[builder(default="0")]
    pub have_to_pay: i64,

    #[builder(default = "0")]
    pub need_to_earn: i64,
}


impl Act {
    #[new]
    pub fn new(user: PyObject, paid: i64) -> Self {
        Act {
            user,
            paid,
            have_to_pay: 0,
            need_to_earn: 0,
        }
    }
}


#[derive(Debug, Clone)]
pub struct User {
    pub user: !
}


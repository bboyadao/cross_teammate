use wasm_bindgen::prelude::*;
use mate_core::models::expenses::Expense as CoreExpense;
use mate_core::models::teammate::Teammate as CoreTeammate;
use mate_core::traits::users::Users;
use mate_core::models::users::User as CoreUser;

// ── JsUlid ───────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct JsUlid {
    value: String,
}

#[wasm_bindgen]
impl JsUlid {
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsUlid {
        JsUlid { value: mate_core::new_ulid().to_string() }
    }

    #[wasm_bindgen(js_name = fromString)]
    pub fn from_string(s: &str) -> Result<JsUlid, JsValue> {
        mate_core::ulid_from_string(s)
            .map(|id| JsUlid { value: id.to_string() })
            .map_err(|e| JsValue::from_str(&e))
    }
}

impl Default for JsUlid {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for JsUlid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[wasm_bindgen]
impl JsUlid {
    #[wasm_bindgen(js_name = toString)]
    pub fn to_js_string(&self) -> String {
        self.value.clone()
    }
}

// ── JsUser ───────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct JsUser {
    inner: CoreUser,
}

#[wasm_bindgen]
impl JsUser {
    /// Create a new user with an auto-generated ULID.
    #[wasm_bindgen(constructor)]
    pub fn new(name: &str) -> JsUser {
        JsUser { inner: CoreUser::new(name, None) }
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.inner.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> String {
        self.inner.id.to_string()
    }
}

// ── JsExpense ────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct JsExpense {
    inner: CoreExpense,
}

#[wasm_bindgen]
impl JsExpense {
    /// `user`  – the JsUser who made this expense  
    /// `paid`  – how much they actually paid
    #[wasm_bindgen(constructor)]
    pub fn new(user: &JsUser, paid: u64) -> JsExpense {
        JsExpense {
            inner: CoreExpense {
                amount: paid,
                user: user.inner.clone(),
                paid,
                have_to_pay: 0,
                need_to_earn: 0,
            },
        }
    }

    #[wasm_bindgen(getter)]
    pub fn paid(&self) -> u64 {
        self.inner.paid
    }

    #[wasm_bindgen(getter)]
    pub fn amount(&self) -> u64 {
        self.inner.amount
    }
}

// ── JsPayment ────────────────────────────────────────────────────────────────

/// A single settled payment: `src` owes `amount` to `dst`.
#[wasm_bindgen]
pub struct JsPayment {
    pub amount: u64,
    src_name: String,
    dst_name: String,
}

#[wasm_bindgen]
impl JsPayment {
    #[wasm_bindgen(getter)]
    pub fn src(&self) -> String {
        self.src_name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn dst(&self) -> String {
        self.dst_name.clone()
    }
}

// ── JsTeammate ───────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct JsTeammate {
    inner: CoreTeammate,
}

#[wasm_bindgen]
impl JsTeammate {
    /// Build a Teammate from a JS Array of JsExpense objects.
    ///
    /// ```js
    /// const tm = JsTeammate.from_expenses([
    ///   new JsExpense(new JsUser("Alice"), 300n),
    ///   new JsExpense(new JsUser("Bob"),   0n),
    /// ]);
    /// const payments = tm.calculate();
    /// ```
    #[wasm_bindgen]
    pub fn from_expenses(expenses: Vec<JsExpense>) -> JsTeammate {
        let core_expenses: Vec<CoreExpense> =
            expenses.into_iter().map(|e| e.inner).collect();
        let users: Vec<CoreUser> =
            core_expenses.iter().map(|e| e.user.clone()).collect();
        JsTeammate {
            inner: CoreTeammate { users, expenses: core_expenses },
        }
    }

    /// Compute the minimal settlement payments.
    /// Returns a JS Array of JsPayment.
    #[wasm_bindgen]
    pub fn calculate(&self) -> Vec<JsPayment> {
        self.inner
            .calculate()
            .into_iter()
            .map(|p| JsPayment {
                amount: p.amount,
                src_name: p.src.name,
                dst_name: p.dst.name,
            })
            .collect()
    }
}

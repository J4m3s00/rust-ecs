use std::any::Any;

use serde::{Deserialize, Serialize};

pub trait Component: Any + Send + Sync + Serialize + for<'a> Deserialize<'a> {}

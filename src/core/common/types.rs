use std::sync::Arc;
use std::sync::Mutex;

pub type Void = ();
pub type ArcMutex<T> = Arc<Mutex<T>>;
pub type Result<T> = std::io::Result<T>;

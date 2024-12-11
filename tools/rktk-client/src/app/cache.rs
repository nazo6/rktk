use std::{any::Any, cell::RefCell, collections::HashMap, future::Future, rc::Rc};

use dioxus::hooks::{use_context, use_context_provider};

#[derive(Clone)]
pub struct Cache(Rc<RefCell<HashMap<&'static str, Box<dyn Any>>>>);

pub async fn with_cache<T: Clone + 'static, E, F: Future<Output = Result<T, E>>>(
    cache: Cache,
    key: &'static str,
    fut: F,
) -> Result<T, E> {
    if let Some(val) = cache.0.borrow_mut().get(key) {
        if let Some(val) = val.downcast_ref::<T>() {
            return Ok(val.clone());
        }
    }

    let res = fut.await;
    if let Ok(val) = &res {
        cache.0.borrow_mut().insert(key, Box::new(val.clone()));
    }

    res
}

pub fn invalidate_cache(cache: Cache, key: &'static str) {
    cache.0.borrow_mut().remove(key);
}

pub fn use_cache_context_provider() -> Cache {
    use_context_provider(|| Cache(Rc::new(RefCell::new(HashMap::new()))))
}

pub fn use_cache() -> Cache {
    use_context::<Cache>()
}

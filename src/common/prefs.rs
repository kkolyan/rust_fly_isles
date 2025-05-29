use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::MutexGuard;
use macroquad::logging::error;

#[derive(Clone, Debug)]
pub struct Pref<T> {
    name: &'static str,
    default: T,
    cached_value: Rc<RefCell<Option<T>>>,
}

fn get_storage<'a>() -> MutexGuard<'a, quad_storage::LocalStorage> {
    quad_storage::STORAGE.lock().unwrap()
}

impl<T: FromStr + ToString + Clone> Pref<T> {
    pub fn new(name: &'static str, default: T) -> Self {
        let value = get_storage()
            .get(name)
            .and_then(|s| {
                let result = T::from_str(s.as_str());
                match result {
                    Ok(result) => Some(result),
                    Err(err) => {
                        error!("failed to parse pref '{}': {}", name, s);
                        None
                    }
                }
            });
        Pref {
            name,
            default,
            cached_value: Rc::new(RefCell::new(value)),
        }
    }

    pub fn get(&self) -> T {
        self.cached_value.borrow().as_ref().unwrap_or(&self.default).clone()
    }

    pub fn set(&mut self, value: T) {
        get_storage().set(self.name, value.to_string().as_str());
        *self.cached_value.borrow_mut() = Some(value);
    }
}
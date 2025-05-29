
#[macro_export] macro_rules! make_bounded_composite {
    ($vis: vis, $struct_name: ident $(<$struct_name_param:ident>)?, $bound: ident $(<$bound_param:ident>)?) => {

        #[derive(core::clone::Clone, std::fmt::Debug)]
        $vis struct $struct_name$(<$struct_name_param>)? {
            target: crate::common::composite::Composite,
            ptrs: std::collections::HashMap<std::any::TypeId, *const dyn $bound$(<$bound_param>)?>,
        }

        impl $(<$struct_name_param>)? $struct_name$(<$struct_name_param>)? {
            pub fn new() -> $struct_name$(<$struct_name_param>)? {
                $struct_name {
                    target: crate::common::composite::Composite::new(),
                    ptrs: std::collections::HashMap::new(),
                }
            }

            pub fn get<T: $bound$(<$bound_param>)? + 'static>(&self) -> Option<&T> {
                self.target.get::<T>()
            }

            pub fn remove<T: $bound$(<$bound_param>)? + Clone + std::fmt::Debug + Sized + 'static>(&mut self) -> Option<T> {
                self.ptrs.remove(&std::any::TypeId::of::<T>());
                self.target.remove::<T>()
            }

            pub fn put<T: $bound$(<$bound_param>)? + Clone + std::fmt::Debug + 'static>(&mut self, value: T) {
                self.target.put::<T>(value);
                self.ptrs.insert(std::any::TypeId::of::<T>(), self.target.get::<T>().unwrap());
            }

            pub fn iter<'a>(&'a self) -> std::iter::Map<
                std::collections::hash_map::Iter<
                    'a,
                    std::any::TypeId,
                    *const dyn $bound$(<$bound_param>)?
                >,
                fn(
                    (
                        &'a std::any::TypeId,
                        &'a *const dyn $bound$(<$bound_param>)?
                    )
                ) -> &'a dyn $bound$(<$bound_param>)?
            > {
                self.ptrs.iter().map(|(_, value)| unsafe { value.as_ref().unwrap() })
            }
        }
    };
}

trait TestTrait {}

make_bounded_composite!(,TestTraitComposite, TestTrait);
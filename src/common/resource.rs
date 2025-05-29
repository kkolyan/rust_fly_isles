use std::any::Any;
use std::any::TypeId;
use std::cell::RefCell;
use std::cell::Cell;
use std::cell::UnsafeCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;

use futures::future::LocalBoxFuture;
use futures::FutureExt;
use macroquad::rand::rand;
use crate::common::perf::perf_task;
use crate::error;


pub type ResourceLoadAsync<T> = fn(ResourceManager) -> LocalBoxFuture<'static, T>;
pub type ResourceLoad<T> = fn(ResourceManager) -> T;

#[derive(Debug, Clone)]
pub struct ResourceManager {
    inner: Rc<ResourceManagerBox>,
}

#[derive(Debug)]
struct ResourceManagerBox {
    on_progress_change: fn(i32, i32) -> LocalBoxFuture<'static, ()>,
    completed: Cell<i32>,
    pending: Cell<i32>,
    cache: RefCell<HashMap<TypeId, RawTypeManager>>,
    tasks: RefCell<VecDeque<Box<dyn LoadingTask>>>,
    seal: UnsafeCell<Cell<bool>>,
}


trait LoadingTask: Debug {
    fn load(&self, rm: ResourceManager) -> LocalBoxFuture<()>;
}

#[derive(Debug)]
enum GenericResourceLoad<T> {
    Plain(ResourceLoad<T>),
    Async(ResourceLoadAsync<T>),
}

#[derive(Debug)]
struct SpecializedLoadingTask<T> {
    data: Weak<ResourceBox<T>>,
    loader: GenericResourceLoad<T>,
}

impl<T: Debug> LoadingTask for SpecializedLoadingTask<T> {
    fn load(&self, rm: ResourceManager) -> LocalBoxFuture<()> {
        async move {
            let value = match self.loader {
                GenericResourceLoad::Plain(f) => {
                    async {
                        f(rm.clone())
                    }.await
                }
                GenericResourceLoad::Async(f) => {
                    f(rm.clone()).await
                }
            };
            if let Some(data) = &self.data.upgrade() {
                data.set_value(value);
            }
        }.boxed_local()
    }
}

pub trait ResourceGet<T> {
    fn get(&self, rm: &ResourceManager) -> Resource<T>;
}

impl<T: 'static + Debug> ResourceGet<T> for ResourceLoadAsync<T> {
    fn get(&self, rm: &ResourceManager) -> Resource<T> {
        rm.allocate_res(GenericResourceLoad::Async(*self), ResourceKey::Async(self), false)
    }
}

impl<T: 'static + Debug> ResourceGet<T> for ResourceLoad<T> {
    fn get(&self, rm: &ResourceManager) -> Resource<T> {
        rm.allocate_res(GenericResourceLoad::Plain(*self), ResourceKey::Plain(self), true)
    }
}

pub trait ResourceManagerRc {
    fn update_progress(&self) -> LocalBoxFuture<'static, ()>;
    fn poll_tasks(&self) -> LocalBoxFuture<'static, ()>;
}

impl ResourceManager {
    pub fn perform_debug_checks(&self) {
        let rm = &self.inner;
        if rm.completed.get() != rm.pending.get() {
            error!("WARNING: completed != pending ({} != {})", rm.completed.get(), rm.pending.get());
        }
    }
}

impl ResourceManager {
    pub fn new(on_progress_change: fn(i32, i32) -> LocalBoxFuture<'static, ()>) -> ResourceManager {
        ResourceManager {
            inner: Rc::new(ResourceManagerBox {
                on_progress_change,
                completed: Default::default(),
                pending: Default::default(),
                cache: Default::default(),
                tasks: Default::default(),
                seal: UnsafeCell::new(Cell::new(false)),
            })
        }
    }

    fn allocate_res<T: 'static + Debug>(&self, loader: GenericResourceLoad<T>, key: ResourceKey<T>, load_synchronously: bool) -> Resource<T> {
        let res_box = Rc::new(ResourceBox::new(SealPtr { sealed: self.inner.seal.get() }));
        {
            let type_id = TypeId::of::<T>();
            let mut by_type = self.inner.cache.borrow_mut();
            let by_ptr = by_type
                .entry(type_id)
                .or_insert_with(|| SpecializedTypeManager::<T>::new().despecialize())
                .specialize::<T>();
            if let Some(value) = by_ptr.values.get(&key) {
                if let Some(value) = value.upgrade() {
                    perf_task("resource returned (cached)");
                    return Resource(ResourceImpl::Managed(value));
                }
            }
            by_ptr.values.insert(key, Rc::downgrade(&res_box));
        }
        if load_synchronously {
            let value = match loader {
                GenericResourceLoad::Plain(f) => f(self.clone()),
                GenericResourceLoad::Async(_) => panic!("cannot load synchronously async resource")
            };
            res_box.set_value(value);
        } else {
            let task = SpecializedLoadingTask {
                data: Rc::downgrade(&res_box),
                loader,
            };
            self.inner.pending.set(self.inner.pending.get() + 1);
            self.inner.tasks
                .borrow_mut()
                .push_back(Box::new(task));
        }
        Resource(ResourceImpl::Managed(res_box))
    }
}

//==================================================================================================

impl ResourceManagerRc for ResourceManager {
    fn update_progress(&self) -> LocalBoxFuture<'static, ()> {
        // perf_task("update progress");
        let rm = self.clone();
        async move {
            let rm = rm.inner;
            rm.completed.set(rm.completed.get() + 1);
            let on_progress_change = rm.on_progress_change;
            on_progress_change(rm.pending.get(), rm.completed.get()).await;
        }.boxed_local()
    }

    fn poll_tasks(&self) -> LocalBoxFuture<'static, ()> {
        let rm = self.clone();
        async move {
            let seal = unsafe { &*rm.inner.seal.get() };
            seal.set(false);
            loop {
                let task = rm.inner.tasks.borrow_mut().pop_front();
                if let Some(task) = task {
                    task.load(rm.clone()).await;
                    rm.update_progress().await;
                } else {
                    break;
                }
            }
            seal.set(true);
        }.boxed_local()
    }
}

#[derive(Debug)]
struct RawTypeManager(Box<dyn Any>);

impl RawTypeManager {
    fn specialize<T: 'static>(&mut self) -> &mut SpecializedTypeManager<T> {
        self.0.downcast_mut::<SpecializedTypeManager<T>>().unwrap()
    }
}

struct SpecializedTypeManager<T> {
    values: HashMap<ResourceKey<T>, Weak<ResourceBox<T>>>,
}

impl<T: 'static> SpecializedTypeManager<T> {
    fn new() -> SpecializedTypeManager<T> {
        SpecializedTypeManager {
            values: Default::default()
        }
    }
    fn despecialize(self) -> RawTypeManager {
        RawTypeManager(Box::new(self))
    }
}

enum ResourceKey<T> {
    Plain(*const ResourceLoad<T>),
    Async(*const ResourceLoadAsync<T>),
}

impl<T> PartialEq<Self> for ResourceKey<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ResourceKey::Plain(ptr) => {
                match other {
                    ResourceKey::Plain(other_ptr) => ptr == other_ptr,
                    ResourceKey::Async(_) => false,
                }
            }
            ResourceKey::Async(ptr) => {
                match other {
                    ResourceKey::Plain(_) => false,
                    ResourceKey::Async(other_ptr) => ptr == other_ptr,
                }
            }
        }
    }
}

impl<T> Eq for ResourceKey<T> {}

impl<T> Hash for ResourceKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self {
            ResourceKey::Plain(ptr) => ptr.hash(state),
            ResourceKey::Async(ptr) => ptr.hash(state),
        }
        state.finish();
    }
}

#[derive(Debug)]
pub struct Resource<T>(ResourceImpl<T>);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Uid([u32; 4]);

impl Uid {
    fn new() -> Self {
        Uid([rand(), rand(), rand(), rand(), ])
    }
}

#[derive(Debug, Clone)]
enum ResourceImpl<T> {
    Detached(Uid, Rc<T>),
    Managed(Rc<ResourceBox<T>>),
}

#[derive(Debug)]
struct ResourceBox<T> {
    uid: Uid,
    value: UnsafeCell<Option<T>>,
    seal: SealPtr,
}

impl<T> PartialEq<Self> for Resource<T> {
    fn eq(&self, other: &Self) -> bool {
        match &self.0 {
            ResourceImpl::Detached(a, _) => {
                match &other.0 {
                    ResourceImpl::Detached(b, _) => a == b,
                    ResourceImpl::Managed(_) => false,
                }
            }
            ResourceImpl::Managed(a) => {
                match &other.0 {
                    ResourceImpl::Detached(_, _) => false,
                    ResourceImpl::Managed(b) => a.value.get() == b.value.get(),
                }
            }
        }
    }
}

impl<T> Eq for Resource<T> {}

impl <T> Hash for Resource<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            ResourceImpl::Detached(a, _) => {
                a.hash(state);
            }
            ResourceImpl::Managed(a) => {
                a.uid.hash(state);
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct SealPtr {
    sealed: *const Cell<bool>,
}

impl<T> ResourceBox<T> {
    fn new(seal: SealPtr) -> Self {
        ResourceBox {
            uid: Uid::new(),
            value: UnsafeCell::new(None),
            seal,
        }
    }

    /*
    UB is theoretically expected in the future (when resource reloading is implemented), if some
    calls `poll_tasks` while immutable reference is active.
    To avoid it, call `poll_tasks` only in the root of main loop and, obviously, do not use
    references to reloadable resources at the same level or above (which is pretty unlikely to be useful).
     */

    fn set_value(&self, value: T) {
        unsafe {
            assert!(!(&*self.seal.sealed).get(), "cannot modify sealed resource");
            let ptr = self.value.get();
            // see unsafe.md for details
            *ptr = Some(value);
        }
    }

    fn get_value_ref(&self) -> &T {
        // see unsafe.md for details
        let value_ref = unsafe {
            assert!((&*self.seal.sealed).get(), "cannot read non-sealed resource");
            let ptr = self.value.get();
            &*ptr
        };
        value_ref
            .as_ref()
            .expect("res not loaded yet")
    }
}

impl<T> Resource<T> {
    pub fn detached(t: T) -> Self {
        Resource(ResourceImpl::Detached(Uid::new(), Rc::new(t)))
    }
}

impl<T> Clone for Resource<T> {
    fn clone(&self) -> Self {
        match &self.0 {
            ResourceImpl::Detached(uid, v) => Resource(ResourceImpl::Detached(*uid, v.clone())),
            ResourceImpl::Managed(v) => Resource(ResourceImpl::Managed(v.clone()))
        }
    }
}

impl<T> Deref for Resource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0 {
            ResourceImpl::Detached(_, v) => v.deref(),
            ResourceImpl::Managed(v) => v.get_value_ref(),
        }
    }
}

use arc_swap::ArcSwapOption;
use moss_applib::{Service, context::Event};
use moss_fs::FileSystem;
use moss_text::ReadOnlyStr;
use moss_workbench::workbench::Workbench;
use rustc_hash::FxHashMap;
use std::{
    any::{Any, TypeId},
    ops::{Deref, DerefMut},
    sync::{
        Arc, Mutex, Weak,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
};
use tauri::{AppHandle, Runtime as TauriRuntime};
use tokio::sync::RwLock;

use crate::{
    command::{CommandCallback, CommandDecl},
    models::types::{ColorThemeInfo, LocaleInfo},
};

pub struct AppPreferences {
    pub theme: RwLock<Option<ColorThemeInfo>>,
    pub locale: RwLock<Option<LocaleInfo>>,
}

pub struct AppDefaults {
    pub theme: ColorThemeInfo,
    pub locale: LocaleInfo,
}

type AnyService = Arc<dyn Any + Send + Sync>;

#[derive(Default)]
pub struct AppServices(FxHashMap<TypeId, AnyService>);

impl Deref for AppServices {
    type Target = FxHashMap<TypeId, AnyService>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppServices {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct AppCommands<R: TauriRuntime>(FxHashMap<ReadOnlyStr, CommandCallback<R>>);

impl<R: TauriRuntime> Default for AppCommands<R> {
    fn default() -> Self {
        Self(FxHashMap::default())
    }
}

impl<R: TauriRuntime> Deref for AppCommands<R> {
    type Target = FxHashMap<ReadOnlyStr, CommandCallback<R>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: TauriRuntime> DerefMut for AppCommands<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type Listener<T> = Arc<dyn Fn(&T) + Send + Sync + 'static>;

pub struct Subscriber<Callback> {
    callback: Callback,
    active: AtomicBool,
}

pub struct Subscription<T> {
    emitter: Weak<Mutex<EmitterState<T>>>,
    id: ArcSwapOption<usize>,
}

impl<T> Subscription<T> {
    pub fn unsubscribe(&self) {
        if let Some(id) = self.id.swap(None) {
            if let Some(inner) = self.emitter.upgrade() {
                inner.lock().unwrap().listeners.remove(&id);
            }
        }
    }
}

impl<T> Drop for Subscription<T> {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

struct EmitterState<T> {
    listeners: FxHashMap<usize, Listener<T>>,
    next_id: AtomicUsize,
}

pub struct Emitter<T> {
    state: Arc<Mutex<EmitterState<T>>>,
}

impl<T> Emitter<T> {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EmitterState {
                listeners: FxHashMap::default(),
                next_id: AtomicUsize::new(0),
            })),
        }
    }

    pub fn subscribe<F>(&self, f: F) -> Subscription<T>
    where
        F: Fn(&T) + Send + Sync + 'static,
    {
        let mut guard = self.state.lock().unwrap();
        let id = guard.next_id.fetch_add(1, Ordering::Relaxed);
        guard.listeners.insert(id, Arc::new(f));
        drop(guard);

        Subscription {
            emitter: Arc::downgrade(&self.state),
            id: ArcSwapOption::new(Some(Arc::new(id))),
        }
    }

    pub fn fire(&self, value: T) {
        let listeners = {
            let guard = self.state.lock().unwrap();
            guard.listeners.values().cloned().collect::<Vec<_>>()
        };

        for listener in listeners {
            listener(&value);
        }
    }
}

pub struct App<R: TauriRuntime> {
    pub(crate) fs: Arc<dyn FileSystem>,
    pub(crate) app_handle: AppHandle<R>,
    pub(crate) workbench: Workbench<R>,
    pub(crate) commands: AppCommands<R>,
    pub(crate) preferences: AppPreferences,
    pub(crate) defaults: AppDefaults,
    pub(crate) services: AppServices,
    // pub(crate) event_listeners: Arc<std::sync::RwLock<FxHashMap<TypeId, Vec<(usize, Listener)>>>>,
    // pub(crate) pending_events: Arc<std::sync::RwLock<Vec<(TypeId, Box<dyn Event>)>>>,
}

impl<R: TauriRuntime> Deref for App<R> {
    type Target = AppHandle<R>;

    fn deref(&self) -> &Self::Target {
        &self.app_handle
    }
}

pub struct AppBuilder<R: TauriRuntime> {
    fs: Arc<dyn FileSystem>,
    app_handle: AppHandle<R>,
    workbench: Workbench<R>,
    services: AppServices,
    defaults: AppDefaults,
    preferences: AppPreferences,
    commands: AppCommands<R>,
}

impl<R: TauriRuntime> AppBuilder<R> {
    pub fn new(
        app_handle: AppHandle<R>,
        workbench: Workbench<R>,
        defaults: AppDefaults,
        fs: Arc<dyn FileSystem>,
    ) -> Self {
        Self {
            fs,
            app_handle,
            workbench,
            defaults,
            preferences: AppPreferences {
                theme: RwLock::new(None),
                locale: RwLock::new(None),
            },
            commands: Default::default(),
            services: Default::default(),
        }
    }

    pub fn with_service<T: Service + Send + Sync>(mut self, service: T) -> Self {
        self.services.insert(TypeId::of::<T>(), Arc::new(service));
        self
    }

    pub fn with_command(mut self, command: CommandDecl<R>) -> Self {
        self.commands.insert(command.name, command.callback);
        self
    }

    pub fn build(self) -> App<R> {
        App {
            fs: self.fs,
            app_handle: self.app_handle,
            workbench: self.workbench,
            commands: self.commands,
            preferences: self.preferences,
            defaults: self.defaults,
            services: self.services,
            // event_listeners: Default::default(),
            // pending_events: Default::default(),
        }
    }
}
impl<R: TauriRuntime> App<R> {
    pub fn workbench(&self) -> &Workbench<R> {
        &self.workbench
    }

    pub fn preferences(&self) -> &AppPreferences {
        &self.preferences
    }

    pub fn defaults(&self) -> &AppDefaults {
        &self.defaults
    }

    pub fn service<T: Service>(&self) -> &T {
        let type_id = TypeId::of::<T>();
        let service = self.services.get(&type_id).expect(&format!(
            "Service {} must be registered before it can be used",
            std::any::type_name::<T>()
        ));

        service.downcast_ref::<T>().expect(&format!(
            "Service {} is registered with the wrong type type id",
            std::any::type_name::<T>()
        ))
    }

    pub fn command(&self, id: &ReadOnlyStr) -> Option<CommandCallback<R>> {
        self.commands.get(id).map(|cmd| Arc::clone(cmd))
    }

    // pub fn subscribe<T: Event, F>(&mut self, listener: F) -> Subscription
    // where
    //     F: Fn(&T) -> bool + Send + Sync + 'static,
    // {
    //     let type_id = TypeId::of::<T>();

    //     let erased_listener: Listener =
    //         Arc::new(
    //             move |event_any: &dyn Any| match event_any.downcast_ref::<T>() {
    //                 Some(event) => listener(event),
    //                 None => {
    //                     eprintln!(
    //                         "Type mismatch in event listener for {}",
    //                         std::any::type_name::<T>()
    //                     );
    //                     false
    //                 }
    //             },
    //         );

    //     let mut event_listeners = self.event_listeners.write().unwrap();
    //     let subscriber_id = event_listeners.len();

    //     event_listeners
    //         .entry(type_id)
    //         .or_default()
    //         .push((subscriber_id, erased_listener));

    //     let event_listeners_clone = self.event_listeners.clone();
    //     Subscription {
    //         subscriber_id,
    //         unsubscribe: Arc::new(move || {
    //             let mut event_listeners = event_listeners_clone.write().unwrap();
    //             event_listeners
    //                 .get_mut(&type_id)
    //                 .unwrap()
    //                 .remove(subscriber_id);
    //         }),
    //     }
    // }

    // pub fn emit<T: Event>(&self, event: T) {
    //     let type_id = TypeId::of::<T>();
    //     let mut pending_events = self.pending_events.write().unwrap();
    //     pending_events.push((type_id, Box::new(event)));
    // }

    // pub fn notify(&self) {
    //     let mut pending_events = self.pending_events.write().unwrap();
    //     while let Some((type_id, event)) = pending_events.pop() {
    //         if let Some(listeners) = self.event_listeners.read().unwrap().get(&type_id) {
    //             for (_, listener) in listeners {
    //                 listener(&*event as &dyn Any);
    //             }
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestEvent {
        pub value: String,
    }

    #[test]
    fn test_emitter() {
        let emitter: Emitter<TestEvent> = Emitter::new();

        let subscription = emitter.subscribe(|event: &TestEvent| {
            println!("Received event: {}", event.value);
        });

        emitter.fire(TestEvent {
            value: "Hello, world!".to_string(),
        });

        drop(subscription);

        emitter.fire(TestEvent {
            value: "2nd event!".to_string(),
        });
    }
}

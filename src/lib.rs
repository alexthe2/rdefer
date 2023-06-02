//! A Rust crate providing defer functionality for both synchronous and asynchronous code.
//!
//! This crate provides a defer functionality which is similar to Go's `defer` statement.
//! Furthermore it also allows for asynchronous defers. This is done by using a counter
//! which is decremented every time a defer is executed. When the counter reaches 0, the
//! provided function is executed.

/// The Defer struct provides defer functionality for synchronous code.
/// It takes a function which is run when the Defer struct is dropped.
pub struct Defer<F: FnOnce()> {
    f: Option<F>,
}

impl<F: FnOnce()> Defer<F> {
    /// Creates a new Defer instance with the provided function.
    pub fn new(f: F) -> Defer<F> {
        Defer { f: Some(f) }
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    /// Runs the stored function when the Defer struct is dropped.
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f()
        }
    }
}

#[cfg(feature = "async")]
mod async_defer {
    use std::future::Future;
    use std::sync::{Arc, Mutex};
    use tokio::runtime::Runtime;

    /// The AsyncDefer struct provides defer functionality for asynchronous code.
    /// It takes a function which is run when the counter reaches 0.
    pub struct AsyncDefer<F: Future<Output = ()> + Send + 'static> {
        f: Option<F>,
        rt: Runtime,
        counter: Arc<Mutex<usize>>,
    }

    impl<F: Future<Output = ()> + Send + 'static> AsyncDefer<F> {
        /// Creates a new AsyncDefer instance with the provided function and counter.
        pub fn new(counter: usize, f: F) -> Arc<Mutex<Self>> {
            let counter = Arc::new(Mutex::new(counter));
            let defer = AsyncDefer {
                f: Some(f),
                rt: Runtime::new().unwrap(),
                counter: counter.clone(),
            };
            Arc::new(Mutex::new(defer))
        }

        /// Executes a function and decrements the counter.
        /// When the counter reaches 0, the deferred function is run.
        pub fn exec(&mut self, action: impl FnOnce() + Send + 'static) {
            let counter = self.counter.lock().unwrap().clone();
            self.rt.spawn(async move {
                action();
                let mut counter = counter.lock().unwrap();
                *counter -= 1;
            });
        }
    }
}

/// A macro for creating a Defer instance.
/// This macro takes a block of code to be deferred.
#[macro_export]
macro_rules! defer {
    ($($t:tt)*) => {
        $crate::Defer::new(|| { $($t)* })
    };
}

#[cfg(feature = "async")]
/// A macro for creating an AsyncDefer instance.
/// This macro takes a count and a block of async code to be deferred.
#[macro_export]
macro_rules! async_defer {
    ($count:expr, $f:expr) => {
        $crate::async_defer::AsyncDefer::new($count, $f)
    };
}

#[cfg(feature = "async")]
/// A macro for executing code before the async defer.
/// This macro takes an AsyncDefer instance and a block of code to execute.
#[macro_export]
macro_rules! exec_before_defer {
    ($defer:expr, $action:expr) => {
        $defer.lock().unwrap().exec($action)
    };
}

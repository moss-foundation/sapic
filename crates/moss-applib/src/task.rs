use std::{
    future::Future,
    pin::Pin,
    task::{Context as TaskContext, Poll},
    time::Duration,
};

use tokio::{sync::oneshot, time::Instant};

pub enum TaskError<E> {
    Err(E),
    Timeout,
    Cancelled,
}

pub struct Task<T, E> {
    inner: Pin<Box<dyn Future<Output = Result<T, TaskError<E>>> + Send>>,
    cancel: Option<oneshot::Sender<()>>,
}

impl<T: Send + 'static, E: Send + 'static> Task<T, E> {
    pub fn new<F>(future: F, timeout: Option<Duration>) -> Self
    where
        F: Future<Output = Result<T, E>> + Send + 'static,
    {
        let (tx, mut rx) = oneshot::channel::<()>();
        let fut = async move {
            match timeout {
                Some(duration) => {
                    let deadline = Instant::now() + duration;
                    tokio::select! {
                        res = future => match res {
                            Ok(val) => Ok(val),
                            Err(e) => Err(TaskError::Err(e)),
                        },
                        _ = &mut rx => Err(TaskError::Cancelled),
                        _ = tokio::time::sleep_until(deadline) => Err(TaskError::Timeout),
                    }
                }
                None => {
                    tokio::select! {
                        res = future => match res {
                            Ok(val) => Ok(val),
                            Err(e) => Err(TaskError::Err(e)),
                        },
                        _ = &mut rx => Err(TaskError::Cancelled),
                    }
                }
            }
        };

        Task {
            inner: Box::pin(fut),
            cancel: Some(tx),
        }
    }

    pub fn cancel(&mut self) {
        if let Some(tx) = self.cancel.take() {
            let _ = tx.send(());
        }
    }
}

impl<T: Send + 'static, E: Send + 'static> Future for Task<T, E> {
    type Output = Result<T, TaskError<E>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        self.inner.as_mut().poll(cx)
    }
}

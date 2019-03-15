use futures::task;
use futures::task::Task;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct PoolState<T> {
  active: bool,
  queue: VecDeque<T>,
  task: Option<Task>,
}

impl<T> Default for PoolState<T> {
  #[inline]
  fn default() -> Self {
    Self {
      active: false,
      queue: VecDeque::new(),
      task: None,
    }
  }
}

impl<T> PoolState<T> {
  #[inline]
  pub fn notify(&mut self) {
    if let Some(task) = self.task.take() {
      task.notify();
    }
  }

  #[inline]
  pub fn push(&mut self, event: T) {
    self.queue.push_back(event);

    if self.task.is_some() {
      self.notify();
    }
  }

  #[inline]
  pub fn pop(&mut self) -> Option<T> {
    self.queue.pop_front()
  }

  #[inline]
  pub fn set_task(&mut self) {
    let _ = self.task.replace(task::current());
  }

  #[inline]
  pub fn is_active(&self) -> bool {
    self.active
  }

  #[inline]
  pub fn activate(&mut self) {
    self.active = true;
  }
}

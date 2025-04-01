use parking_lot::RwLock;

/// 一个全局资源持有器，用于方便的写入和读取全局资源
pub struct GlobalHolder<T>(RwLock<Option<T>>);

impl<T> GlobalHolder<T> {
    pub const  fn new() -> Self {
        GlobalHolder(RwLock::new(None))
    }

    pub fn set(&self, value: T) {
        let write = &mut *self.0.write();
        write.replace(value);
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        let read = &*self.0.read();
        read.clone()
    }
}
pub struct Defer<T, F: FnOnce(T)> {
    data: Option<T>,
    defer: Option<F>,
}

impl<T, F> Defer<T, F>
where
    F: FnOnce(T),
{
    pub fn new(data: T, defer: F) -> Defer<T, F> {
        Defer {
            data: Some(data),
            defer: Some(defer),
        }
    }

    /// 因为只能通过[new](Self::new)构建，所以可以保证一定存在数据
    pub fn data_ref(&self) -> &T {
        self.data.as_ref().unwrap()
    }

    /// 因为只能通过[new](Self::new)构建，所以可以保证一定存在数据
    pub fn data_mut(&mut self) -> &mut T {
        self.data.as_mut().unwrap()
    }

    /// 放弃扫尾操作，直接返回数据，因为只能通过[new](Self::new)构建，并且，本接口会消费掉自己，所以可以保证一定存在数据
    /// 应该只在完全确认必要的情况下才调用本函数放弃后续的扫尾逻辑
    pub fn give_up(mut self) -> T {
        self.defer.take();
        let data = self.data.take().unwrap();
        data
    }
}

/// 在Drop中执行扫尾函数（如果存在)
impl<T, F> Drop for Defer<T, F>
where
    F: FnOnce(T),
{
    fn drop(&mut self) {
        if let Some(data) = self.data.take() {
            let f = self.defer.take();
            f.map(|f| (f)(data));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::defer::Defer;

    #[test]
    fn test() {
        struct A<F>
        where
            F: FnOnce(u32),
        {
            defer: Defer<u32, F>,
        }
        let defer: Defer<u32, _> = Defer::new(10, |data| {
            data;
        });
    }
}

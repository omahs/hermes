use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use crate::runtime::task::spawn;
use crate::std_prelude::*;
use crate::types::cell::Cell;
use crate::utils::future::{new_future, pin_future};

pub struct ReceiverOnce<T> {
    cell: Cell<Option<T>>,
}

pub struct SenderOnce<T> {
    cell: Cell<Option<T>>,
}

pub fn new_channel_once<T>() -> (SenderOnce<T>, ReceiverOnce<T>) {
    let cell = Cell::new(None);
    let sender = SenderOnce { cell: cell.clone() };
    let receiver = ReceiverOnce { cell };
    (sender, receiver)
}

impl<T> Future for ReceiverOnce<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let has_val = self.cell.borrow().is_some();

        if has_val {
            let val = self.cell.borrow_mut().take().unwrap();
            Poll::Ready(val)
        } else {
            Poll::Pending
        }
    }
}

// impl<T> ReceiverOnce<T> {
//     pub async fn recv(self) -> T {
//         let cell = self.cell;
//         new_future(move || {
//             let has_val = cell.borrow().is_some();

//             if has_val {
//                 cell.borrow_mut().take()
//             } else {
//                 None
//             }
//         })
//         .await
//     }
// }

impl<T: Send + Sync + 'static> SenderOnce<T> {
    pub fn send(self, val: T) {
        let cell = self.cell;
        spawn(pin_future(async move { *cell.borrow_mut() = Some(val) }));
    }
}

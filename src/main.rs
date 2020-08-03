#[macro_use]
extern crate derive_deref;
extern crate deno_core;
extern crate pin_project;

use deno_core::CoreIsolate;
use deno_core::CoreIsolateState;
use deno_core::Op;
use deno_core::ResourceTable;
use deno_core::Script;
use deno_core::StartupData;
use deno_core::ZeroCopyBuf;

use std::cell::RefCell;
use std::rc::Rc;
use std::future::{ Future };
use std::error::Error;
use std::str;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use pin_project::pin_project;

use std::sync::mpsc::channel;


#[tokio::main]
async fn main() {
    assert!(process("Test String".to_owned()).await == "Test String");
}

async fn process(test_string: String) -> String {

    let (tx, rx) = channel();

    let mut iso: Isolate = Isolate::new();

    iso.register_sync_op("return_string", move |_, zero_copy_bufs: &mut [ZeroCopyBuf]| {
        dbg!(zero_copy_bufs.len());
        let buf = zero_copy_bufs[0].clone();
        let result = str::from_utf8(&*buf).unwrap().to_owned();
        tx.send(result).unwrap();
        "".to_owned()
    });

    iso.register_sync_op("get_string", move |_, zero_copy_bufs: &mut [ZeroCopyBuf]| {
        test_string.clone()
    });


    iso.core_isolate.execute("AfterPrelude", include_str!("../res/testfile.js")).unwrap();

    rx.recv().unwrap()
    // iso.await.unwrap();
}

#[pin_project]
struct Isolate {
    #[pin]
    core_isolate: CoreIsolate,
    state: State,
}

#[derive(Clone, Default, Deref)]
struct State(Rc<RefCell<StateInner>>);

#[derive(Default)]
struct StateInner {
    resource_table: ResourceTable,
}

impl Future for Isolate {
    type Output = <CoreIsolate as Future>::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();
        this.core_isolate.poll(cx)
    }
}

impl Isolate {
    pub fn new() -> Self {
        let startup_data = StartupData::Script(Script {
            source: include_str!("../res/prelude.js"),
            filename: "testfile.js",
        });

        let isolate = Self {
            core_isolate: CoreIsolate::new(startup_data, false),
            state: Default::default(),
        };

        isolate
    }

    fn register_sync_op<F>(&mut self, name: &'static str, handler: F)
    where
    F: 'static + Fn(State, &mut [ZeroCopyBuf]) -> String //Result<u32, Box<dyn Error>>,
    {
        let state = self.state.clone();
        let core_handler = move |_isolate_state: &mut CoreIsolateState,
        mut zero_copy_bufs: &mut [ZeroCopyBuf]|
        -> Op {
            // assert!(!zero_copy_bufs.is_empty());
            let state = state.clone();

            let result: String = handler(state, &mut zero_copy_bufs);

            // let buf: &[u8] = if zero_copy_bufs.len() > 0 { &zero_copy_bufs[0] } else { &[] };
            let buf = result.as_bytes();

            Op::Sync(Box::from(buf))
        };

        self.core_isolate.register_op(name, core_handler);
    }

    fn register_op<F>(
        &mut self,
        name: &'static str,
        handler: impl Fn(State, &mut Box::<[u8]>) -> F + Copy + 'static,
    ) where
    F: Future::<Output = i32>,
    {
        let state = self.state.clone();
        let core_handler = move |_isolate_state: &mut CoreIsolateState,
        zero_copy_bufs: &mut [ZeroCopyBuf]|
        -> Op {
            assert!(!zero_copy_bufs.is_empty());
            let state = state.clone();
            let mut buf: Box<[u8]> = Box::from(&zero_copy_bufs[0] as &[u8]);

            let fut = async move {
                let _op = handler(state, &mut buf).await;
                buf
            };

            Op::Async(Box::pin(fut))
        };

        self.core_isolate.register_op(name, core_handler);
    }
}

use neon::prelude::*;
use std::sync::Arc;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("with_event_queue_new", with_event_queue_new)?;
    cx.export_function("with_event_queue_name", with_event_queue_name)?;
    cx.export_function("without_event_queue_new", without_event_queue_new)?;
    cx.export_function("without_event_queue_name", without_event_queue_name)?;
    Ok(())
}

pub struct WithEventQueue {
    queue: EventQueue,
    name: String,
}

impl Finalize for WithEventQueue {}

impl WithEventQueue {
    fn new(queue: EventQueue, name: String) -> Arc<Self> {
        Arc::new(Self { queue, name })
    }
}

pub fn with_event_queue_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<WithEventQueue>>> {
    let name = cx.argument::<JsString>(0)?;
    let name = name.value(&mut cx);
    let queue = cx.queue();
    let wrapper = WithEventQueue::new(queue, name);

    Ok(cx.boxed(wrapper))
}

pub fn with_event_queue_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let wrapper = Arc::clone(
        &&cx.this()
            .downcast_or_throw::<JsBox<Arc<WithEventQueue>>, FunctionContext>(&mut cx)?,
    );
    let name = wrapper.name.clone();

    Ok(cx.string(&name))
}

pub struct WithoutEventQueue {
    name: String,
}

impl Finalize for WithoutEventQueue {}

impl WithoutEventQueue {
    fn new(name: String) -> Arc<Self> {
        Arc::new(Self { name })
    }
}

pub fn without_event_queue_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<WithoutEventQueue>>> {
    let name = cx.argument::<JsString>(0)?;
    let name = name.value(&mut cx);
    let wrapper = WithoutEventQueue::new(name);

    Ok(cx.boxed(wrapper))
}

pub fn without_event_queue_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let wrapper = Arc::clone(
        &&cx.this()
            .downcast_or_throw::<JsBox<Arc<WithoutEventQueue>>, FunctionContext>(&mut cx)?,
    );
    let name = wrapper.name.clone();

    Ok(cx.string(&name))
}

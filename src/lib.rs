use neon::prelude::*;
use std::sync::Arc;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("with_channel_new", with_channel_new)?;
    cx.export_function("with_channel_name", with_channel_name)?;
    cx.export_function("without_channel_new", without_channel_new)?;
    cx.export_function("without_channel_name", without_channel_name)?;
    Ok(())
}

pub struct WithChannel {
    channel: Channel,
    name: String,
}

impl Finalize for WithChannel {}

impl WithChannel {
    fn new(channel: Channel, name: String) -> Arc<Self> {
        Arc::new(Self { channel, name })
    }
}

pub fn with_channel_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<WithChannel>>> {
    let name = cx.argument::<JsString>(0)?;
    let name = name.value(&mut cx);
    let channel = cx.channel();
    let wrapper = WithChannel::new(channel, name);

    Ok(cx.boxed(wrapper))
}

pub fn with_channel_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let wrapper = Arc::clone(
        &&cx.this()
            .downcast_or_throw::<JsBox<Arc<WithChannel>>, FunctionContext>(&mut cx)?,
    );
    let name = wrapper.name.clone();

    Ok(cx.string(&name))
}

pub struct WithoutChannel {
    name: String,
}

impl Finalize for WithoutChannel {}

impl WithoutChannel {
    fn new(name: String) -> Arc<Self> {
        Arc::new(Self { name })
    }
}

pub fn without_channel_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<WithoutChannel>>> {
    let name = cx.argument::<JsString>(0)?;
    let name = name.value(&mut cx);
    let wrapper = WithoutChannel::new(name);

    Ok(cx.boxed(wrapper))
}

pub fn without_channel_name(mut cx: FunctionContext) -> JsResult<JsString> {
    let wrapper = Arc::clone(
        &&cx.this()
            .downcast_or_throw::<JsBox<Arc<WithoutChannel>>, FunctionContext>(&mut cx)?,
    );
    let name = wrapper.name.clone();

    Ok(cx.string(&name))
}

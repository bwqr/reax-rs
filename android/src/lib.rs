#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JObject, JValue, JString};
use jni::signature::{JavaType, Primitive};

struct HandlerWrapper<'a> {
    env: JNIEnv<'a>,
    handler: JObject<'a>
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store_startEventLoop<'a>(env: JNIEnv<'a>, _: JObject, handler: JObject<'a>) {
    let wrapper = HandlerWrapper { env, handler };
    let wrapper = unsafe { std::mem::transmute::<HandlerWrapper<'a>, HandlerWrapper<'static>>(wrapper) };

    store::start_event_loop(move |sub_id| {
        let wrapper = &wrapper;
        let object_class = wrapper.env.get_object_class(wrapper.handler).unwrap();
        let method_id = wrapper.env.get_method_id(object_class, "sendEmptyMessage", "(I)Z").unwrap();
        wrapper.env.call_method_unchecked(wrapper.handler, method_id, JavaType::Primitive(Primitive::Boolean), &[JValue::Int(sub_id)]).unwrap();
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store_call(env: JNIEnv, _: JObject, j_callback: JObject) {
    let object_class = env.get_object_class(j_callback).unwrap();
    let method_id = env.get_method_id(object_class, "invoke", "(Ljava/lang/String;)Ljava/lang/Object;").unwrap();
    env.call_method_unchecked(j_callback, method_id, JavaType::Object("Function0".to_string()), &[JValue::Object(*env.new_string("Hola").unwrap())]).unwrap();
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store_register(env: JNIEnv, _: JObject, sub_id: JString) {
    let sub_id: String = env.get_string(sub_id).expect("Couldn't get java string!").into();

    store::register_event(sub_id.to_string());
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store_unregister(env: JNIEnv, _: JObject, sub_id: JString) {
    let sub_id: String = env.get_string(sub_id).expect("Couldn't get java string!").into();

    store::unregister_event(sub_id.to_string());
}

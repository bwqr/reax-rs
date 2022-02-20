#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::{JavaType, Primitive};
use jni::sys::jint;

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1initRuntime(_: JNIEnv, _: JObject) {
    store::init_runtime();
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1initHandler<'a>(env: JNIEnv<'a>, _: JObject, handler: JObject<'a>) {
    store::init_handler(|subs, ser_val| {
        let array_len = ser_val.len().try_into().unwrap();
        let byte_array = env.new_byte_array(array_len).unwrap();
        env.set_byte_array_region(byte_array, 0, ser_val.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

        let message_class = env.find_class("android/os/Message").unwrap();
        let obtain_id = env.get_static_method_id(message_class, "obtain", "(Landroid/os/Handler;ILjava/lang/Object;)Landroid/os/Message;").unwrap();

        for sub in subs {
            let message = env.call_static_method_unchecked(message_class, obtain_id, JavaType::Object("android/os/Message".to_string()), &[JValue::Object(handler), JValue::Int(sub.try_into().unwrap()), JValue::Object(byte_array.into())]).unwrap();

            let handler_class = env.get_object_class(handler).unwrap();
            let send_message_id = env.get_method_id(handler_class, "sendMessage", "(Landroid/os/Message;)Z").unwrap();
            env.call_method_unchecked(handler, send_message_id, JavaType::Primitive(Primitive::Boolean), &[message]).unwrap();
        }
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1initStore(_: JNIEnv, _: JObject) {
    store::init_store();
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1user(_: JNIEnv, _: JObject) -> jint {
    store::user()
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1fetchUser(_: JNIEnv, _: JObject) {
    store::fetch_user()
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store__1unsubscribe(_: JNIEnv, _: JObject, sub: jint) {
    store::unsubscribe(sub);
}

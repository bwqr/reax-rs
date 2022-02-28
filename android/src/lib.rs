#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use jni::signature::JavaType;
use jni::sys::jint;

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1initRuntime(_: JNIEnv, _: JObject) {
    store::init_runtime();
}

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1initHandler(env: JNIEnv, _: JObject, callback: JObject) {
    store::init_handler(|subs, ser_val| {
        let subs_array = env.new_int_array(subs.len().try_into().unwrap()).unwrap();
        env.set_int_array_region(subs_array, 0, subs.into_iter().collect::<Vec<i32>>().as_slice()).unwrap();

        let bytes_array = env.new_byte_array(ser_val.len().try_into().unwrap()).unwrap();
        env.set_byte_array_region(bytes_array, 0, ser_val.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

        let callback_class = env.get_object_class(callback).unwrap();
        let store_handler_id = env.get_method_id(callback_class, "invoke", "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;").unwrap();
        if let Err(e) = env.call_method_unchecked(callback, store_handler_id, JavaType::Object("Ljava/lang/Object;".to_string()), &[JValue::Object(subs_array.into()), JValue::Object(bytes_array.into())]) {
            eprintln!("failed to call storeHandler, {:?}", e);
        }
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1initStore(_: JNIEnv, _: JObject) {
    store::init_store();
}

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1user(_: JNIEnv, _: JObject) -> jint {
    store::user()
}

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1fetchUser(_: JNIEnv, _: JObject) {
    store::fetch_user()
}

#[no_mangle]
pub extern fn Java_com_bwqr_reaxdemo_Store__1unsubscribe(_: JNIEnv, _: JObject, sub: jint) {
    store::unsubscribe(sub);
}

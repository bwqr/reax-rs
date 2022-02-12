#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JObject, JValue, JString};
use jni::signature::{JavaType, Primitive};

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Store_startEventLoop<'a>(env: JNIEnv<'a>, _: JObject, handler: JObject<'a>) {
    store::start_event_loop(move |sub_id| {
        let object_class = env.get_object_class(handler).unwrap();
        let method_id = env.get_method_id(object_class, "sendEmptyMessage", "(I)Z").unwrap();
        env.call_method_unchecked(handler, method_id, JavaType::Primitive(Primitive::Boolean), &[JValue::Int(sub_id)]).unwrap();
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

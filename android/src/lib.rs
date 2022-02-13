#![allow(non_snake_case)]

use jni::JNIEnv;
use jni::objects::{JObject, JValue, JString};
use jni::signature::{JavaType, Primitive};

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Subscriptions_startEventLoop<'a>(env: JNIEnv<'a>, _: JObject, handler: JObject<'a>) {
    store::start_event_loop(move |sub_id, users| {
        let vec = env.new_object_array(users.len() as i32, "[B", env.new_byte_array(0).unwrap()).unwrap();
        for (index, user) in users.iter().enumerate() {
            let serialized_user = bincode::serialize(&user).unwrap();
            let array_len = serialized_user.len().try_into().unwrap();
            let byte_array = env.new_byte_array(array_len).unwrap();
            env.set_byte_array_region(byte_array, 0, serialized_user.iter().map(|byte| *byte as i8).collect::<Vec<i8>>().as_slice()).unwrap();

            env.set_object_array_element(vec, index as i32, byte_array).unwrap();
        }

        let message_class = env.find_class("android/os/Message").unwrap();
        let obtain_id = env.get_static_method_id(message_class, "obtain", "(Landroid/os/Handler;ILjava/lang/Object;)Landroid/os/Message;").unwrap();
        let message = env.call_static_method_unchecked(message_class, obtain_id, JavaType::Object("android/os/Message".to_string()), &[JValue::Object(handler), JValue::Int(sub_id), JValue::Object(vec.into())]).unwrap();

        let handler_class = env.get_object_class(handler).unwrap();
        let send_message_id = env.get_method_id(handler_class, "sendMessage", "(Landroid/os/Message;)Z").unwrap();
        env.call_method_unchecked(handler, send_message_id, JavaType::Primitive(Primitive::Boolean), &[message]).unwrap();
    });
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Subscriptions_call(env: JNIEnv, _: JObject, j_callback: JObject) {
    let object_class = env.get_object_class(j_callback).unwrap();
    let method_id = env.get_method_id(object_class, "invoke", "(Ljava/lang/String;)Ljava/lang/Object;").unwrap();
    env.call_method_unchecked(j_callback, method_id, JavaType::Object("Function0".to_string()), &[JValue::Object(*env.new_string("Hola").unwrap())]).unwrap();
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Subscriptions_register(env: JNIEnv, _: JObject, sub_id: JString) {
    let sub_id: String = env.get_string(sub_id).expect("Couldn't get java string!").into();

    store::register_event(sub_id.to_string());
}

#[no_mangle]
pub extern fn Java_com_bwqr_rustgreetings_Subscriptions_unregister(env: JNIEnv, _: JObject, sub_id: JString) {
    let sub_id: String = env.get_string(sub_id).expect("Couldn't get java string!").into();

    store::unregister_event(sub_id.to_string());
}

#![cfg(target_os = "android")]
#![allow(non_snake_case)]
extern crate jni;

use jni::objects::{JClass, JObject, JValue, JString};
use jni::JNIEnv;

use jni::sys::jstring;

#[no_mangle]
pub extern "C" fn Java_org_wesj_mobilerust_JNI_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let s = super::say_hello();
    let response = env.new_string(&s).expect("Couldn't create java string!");

    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/String;)V",
        &[JValue::from(JObject::from(response))],
    )
    .unwrap();
}

#[no_mangle]
pub extern "C" fn Java_org_wesj_mobilerust_JNI_getATCFInfoJNI(
    env: JNIEnv,
    _class: JClass,
    atcfId: JString
) -> jstring {
    let input: String = env.get_string(atcfId).expect("Couldn't get java string!").into();
    let res = match crate::atcf::get_atcf_info(&input) {
        Ok(message) => format!("{} {} ({} mph)", message.system_type, message.system_name, message.system_intensity_mph),
        Err(e) => format!("{:?}", e)
    };
    let output = env.new_string(res).expect("Couldn't create java string!");

    output.into_inner()
}
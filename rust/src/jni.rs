extern crate jni;

use jni::JNIEnv;
use jni::objects::{JClass, JObject, JValue};

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_org_wesj_mobilerust_JNI_invokeCallbackViaJNI(
    env: JNIEnv,
    _class: JClass,
    callback: JObject
) {
    let s = super::say_hello();
    let response = env.new_string(&s)
        .expect("Couldn't create java string!");

    env.call_method(callback, "callback", "(Ljava/lang/String;)V",
                    &[JValue::from(JObject::from(response))]).unwrap();
}
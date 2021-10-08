#![cfg(target_os = "android")]
#![allow(non_snake_case)]
extern crate jni;
extern crate android_logger;

use std::convert::{TryFrom, TryInto};
use jni::objects::{JClass, JObject, JValue, JString};
use jni::JNIEnv;
use jni::sys::jstring;
use log::Level;
use crate::atcf::{CycloneMessage, CycloneMessageRequest};
use self::android_logger::Config;
use self::jni::errors::Error;
use self::jni::sys::jobject;

pub trait TryIntoJObject {
    fn try_into_jni<'a>(self, env: &'a JNIEnv) ->  Result<JObject<'a>, jni::errors::Error>;
}

pub trait TryFromJObject: Sized {
    fn try_from_jni(env: &JNIEnv, obj: JObject) -> Result<Self, Error>;
}

impl TryIntoJObject for CycloneMessage {
    fn try_into_jni<'a>(self, env: &'a JNIEnv) -> Result<JObject<'a>, Error> {
        env.new_object(
            "org/wesj/mobilerust/CycloneMessage",
            "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            &[
                JValue::from(JObject::from(env.new_string(self.system_type)?)),
                JValue::from(JObject::from(env.new_string(self.system_name)?)),
                JValue::from(JObject::from(env.new_string(self.system_intensity_mph)?)),
            ]
        )
    }
}

impl TryFromJObject for CycloneMessageRequest {
    fn try_from_jni(env: &JNIEnv, obj: JObject) -> Result<Self, Error> {
        let b = env.get_field(obj, "basin", "Ljava/lang/String;")?.l()?;
        let year = env.get_field(obj, "year", "I")?.i()?;
        let number = env.get_field(obj, "number", "I")?.i()?;

        Ok(Self {
            basin: env.get_string(b.into())?.into(),
            year: year.into(),
            number: number.into()
        })
    }
}

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
    request: JObject
) -> jobject {
    /*let input: String = env.get_string(atcfId).expect("Couldn't get java string!").into();
    let res = match crate::atcf::get_atcf_info(&input) {
        Ok(message) => format!("{} {} ({} mph)", message.system_type, message.system_name, message.system_intensity_mph),
        Err(e) => format!("{:?}", e)
    };*
    let output = env.new_string(res).expect("Couldn't create java string!");
*/

    let input = match CycloneMessageRequest::try_from_jni(&env, request) {
        Ok(req) => req,
        Err(jni) => {
            error!("{:?}", jni);
            panic!();
        }
    };

    let res = crate::atcf::get_atcf_info(input).expect("Error getting ATCF info");
    let output = res.try_into_jni(&env).expect("Couldn't convert into JNI");
    output.into_inner()
}

#[no_mangle]
pub extern "C" fn Java_org_wesj_mobilerust_JNI_setupLogging(
    _env: JNIEnv,
    _class: JClass
) {
    android_logger::init_once(Config::default().with_min_level(Level::Trace));
    info!("Logging start!");
}
use eyre::Report;
use jni::{jni_sig, jni_str, Env};
use jni::objects::{JObject};
use jni::sys::jlong;

pub trait JavaPointers<T> {

    fn into_jlong_pointer(self) -> jlong;

    unsafe fn from_jlong_pointer<'a>(pointer: jlong) -> &'a mut T {
        &mut *(pointer as *mut T)
    }
}

pub fn get_pointer_from_field(env: &mut Env, object: &JObject) -> Result<jlong, Report> {
    let field = env.get_field(object, jni_str!("pointer"), jni_sig!("J"))?;
    let pointer = field.j()?;

    if pointer == 0 {
        eyre::bail!("native object is closed or uninitialized");
    }

    Ok(pointer)
}

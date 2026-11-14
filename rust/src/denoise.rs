use jni::objects::{JClass, JFloatArray, JObject, JValue};
use jni::{jni_sig, jni_str, Env, EnvUnowned};
use jni::errors::ThrowRuntimeExAndDefault;
use jni::sys::{jfloat, jlong};
use crate::denoise_container::{frame_count, DenoiseContainer};
use crate::util::exception::{JavaException, JavaExceptions};
use crate::util::into_exception::ErrIntoException;
use crate::util::pointer::{get_pointer_from_field, JavaPointers};

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_createNative<'local>(
    mut env: EnvUnowned<'local>,
    _class: JClass<'local>
) -> jlong {
    env.with_env(|env| create_denoise_state().or_throw(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_processNative<'local>(
    mut env: EnvUnowned<'local>,
    denoise: JObject<'local>,
    input: JFloatArray<'local>,
    output: JFloatArray<'local>
) -> jfloat {
    env.with_env(|env| denoise_process(env, denoise, input, output).or_throw(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_resetNative<'local>(
    mut env: EnvUnowned<'local>,
    denoise: JObject<'local>
) {
    env.with_env(|env| denoise_reset(env, denoise).or_throw(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
pub extern "system" fn Java_com_plasmoverse_rnnoise_Denoise_closeNative<'local>(
    mut env: EnvUnowned<'local>,
    denoise: JObject<'local>
) {
    env.with_env(|env| denoise_close(env, denoise).or_throw(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}


fn create_denoise_state() -> Result<jlong, JavaException> {
    Ok(DenoiseContainer::default().into_jlong_pointer())
}

fn get_denoise<'a>(
    env: &mut Env,
    denoise: &JObject
) -> Result<&'a mut DenoiseContainer, JavaException> {
    let pointer = get_pointer_from_field(env, denoise)
        .err_into_denoise_exception("Failed to get a pointer from the java object".into())?;

    Ok(unsafe { DenoiseContainer::from_jlong_pointer(pointer) })
}

fn denoise_close(
    env: &mut Env,
    denoise: JObject
) -> Result<(), JavaException> {
    let pointer = get_pointer_from_field(env, &denoise)
        .err_into_denoise_exception("Failed to get a pointer from the java object".into())?;

    let _container = unsafe { Box::from_raw(pointer as *mut DenoiseContainer) };
    env.set_field(&denoise, jni_str!("pointer"), jni_sig!("J"), JValue::from(0 as jlong))
        .err_into_denoise_exception("Failed to reset pointer".into())?;

    Ok(())
}

fn denoise_reset(
    env: &mut Env,
    denoise: JObject
) -> Result<(), JavaException> {
    get_denoise(env, &denoise)?.reset();

    Ok(())
}

fn denoise_process(
    env: &mut Env,
    denoise: JObject,
    input: JFloatArray,
    output: JFloatArray
) -> Result<jfloat, JavaException> {
    let container = get_denoise(env, &denoise)?;

    let input_length = input.len(env)
        .err_into_denoise_exception("Failed to get input array length".into())?;
    let output_length = if output.is_null() {
        None
    } else {
        Some(output.len(env).err_into_denoise_exception("Failed to get output array length".into())?)
    };
    let frames = frame_count(input_length, output_length).map_err(JavaException::new_denoise)?;

    let (probability, processed) = container.process(frames, |samples| {
        input.get_region(env, 0, samples)
            .err_into_denoise_exception("Failed to copy input array to rust vec".into())
    })?;

    if !output.is_null() {
        output.set_region(env, 0, processed)
            .err_into_denoise_exception("Failed to copy processed samples into output array".into())?;
    }

    Ok(probability)
}

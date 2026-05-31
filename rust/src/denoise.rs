use jni::objects::{JClass, JFloatArray, JObject, JValue};
use jni::{jni_sig, jni_str, Env, EnvUnowned};
use jni::errors::ThrowRuntimeExAndDefault;
use jni::sys::{jfloat, jlong};
use nnnoiseless::DenoiseState;
use crate::denoise_container::DenoiseContainer;
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
    samples: JFloatArray<'local>
) -> JFloatArray<'local> {
    env.with_env(|env| denoise_process(env, denoise, samples).or_throw(env))
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
    let denoise = DenoiseState::new();

    let denoise_container = DenoiseContainer {
        denoise,
        first: false
    };

    Ok(denoise_container.into_jlong_pointer())
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
        .err_into_denoise_exception("Failed set reset pointer".into())?;

    Ok(())
}

fn denoise_process<'local>(
    env: &mut Env<'local>,
    denoise: JObject<'local>,
    samples: JFloatArray<'local>
) -> Result<JFloatArray<'local>, JavaException> {
    let container = get_denoise(env, &denoise)?;

    let samples_length = samples.len(env)
        .err_into_denoise_exception("Failed to get samples array length".into())?;

    let mut samples_vec = vec![0f64 as jfloat; samples_length];

    samples.get_region(env, 0, &mut samples_vec)
        .err_into_denoise_exception("Failed to copy samples to rust vec".into())?;

    let mut output = Vec::new();
    let mut out_buf = [0.0; DenoiseState::FRAME_SIZE];

    for chunk in samples_vec.chunks_exact(DenoiseState::FRAME_SIZE) {
        container.denoise.process_frame(&mut out_buf[..], chunk);

        if !container.first {
            output.extend_from_slice(&out_buf[..]);
        }
        container.first = false;
    }

    let output_java = env.new_float_array(output.len())
        .err_into_denoise_exception("Failed to create java float array".into())?;

    output_java.set_region(env, 0, &output)
        .err_into_denoise_exception("Failed to copy float vec into java float array".into())?;

    Ok(output_java)
}

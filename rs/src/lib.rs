use std::path::Path;
use tch::{Tensor, Kind, Device, nn, Cuda};

use rust_sentence_transformers::SentenceTransformer;

use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::{jlong, jint, jstring, jobject, jobjectArray};
use serde::{Deserialize, Serialize};
use serde_clj::{to_object, Encoder, Decoder, from_object};


#[no_mangle]
pub unsafe extern "system" fn Java_ClojureRust_getSentenceTransformerRust(env: JNIEnv,
                                                                          _class: JString,
                                                                          model_path: JString,
                                                                          device: JString) -> jlong {
    let model_path: String = env
        .get_string(model_path)
        .expect("Couldn't get model_path java string")
        .into();
    let device: String = env
        .get_string(device)
        .expect("Couldn't get device java string")
        .into();

    let device = match device.as_str() {
        "cpu" => Some(Device::Cpu),
        "cuda" => Some(Device::cuda_if_available()),
        _ => None.expect("\"cpu\" or \"cuda\" expected"),
    };

    let embedder = SentenceTransformer::new(
        Path::new(&model_path),
        device.unwrap());

    Box::into_raw(Box::new(embedder.unwrap())) as jlong
}

#[no_mangle]
pub unsafe extern "system" fn Java_ClojureRust_encodeRust(env: JNIEnv,
                                                              _class: JString,
                                                               model_ptr: jlong,
                                                               documents: JObject,
                                                               batch_size: jint) -> jobject
{
    let embedder = &*(model_ptr as *mut SentenceTransformer);

    env.ensure_local_capacity(64)
        .expect("failed increasing capacity");

    let dec = Decoder::new(env.clone()).unwrap();
    let input: Vec<String> = from_object(&dec, documents).expect("Object deserialisation failed");
    let input: Vec<&str> = input.iter().map(|s| &**s).collect();

    let embeddings = embedder.encode(input, batch_size as i32);
    let enc = Encoder::new(env).unwrap();
    let output = to_object(&enc, &embeddings)
        .expect("serialisation failed!");
    output.into_inner()
}

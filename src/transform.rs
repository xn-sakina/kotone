use anyhow::Result;
use kotone_transform::transform_to_utf8;
use napi::{bindgen_prelude::AsyncTask, Env, Task};

pub struct Transform {
    buffer: Box<[u8]>,
    encoding: Option<String>,
}

impl Transform {
    pub fn transform(&self) -> Result<String> {
        transform_to_utf8(&self.buffer, self.encoding.as_deref())
    }
}

pub struct TransformTask {
    task: Transform,
}

impl Task for TransformTask {
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        Ok(self.task.transform()?)
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(output)
    }
}

#[napi]
pub fn transform_sync(buffer: &[u8], encoding: Option<&str>) -> Result<String> {
    transform_to_utf8(buffer, encoding)
}

#[napi(ts_return_type = "Promise<string>")]
pub fn transform(buffer: &[u8], encoding: Option<&str>) -> AsyncTask<TransformTask> {
    AsyncTask::new(TransformTask {
        task: Transform {
            buffer: buffer.into(),
            encoding: encoding.map(|s| s.to_string()),
        },
    })
}

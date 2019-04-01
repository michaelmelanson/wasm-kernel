use log::{debug, error, info};
use alloc::prelude::*;
use wasmi::{ImportsBuilder, Module, ModuleInstance};



#[no_mangle]
pub extern "C" fn fmodf() {
    panic!("Call to fmodf");
}

#[no_mangle]
pub extern "C" fn fmod() {
    panic!("Call to fmod");
}

pub fn exec_init() {
    let mut runtime = WasmRuntime::new();

    debug!("Creating WASM module for init process...");
    let init_binary: &[u8] = include_bytes!("../extern/init/target/wasm32-unknown-unknown/release/init.wasm");
    let module = Module::from_buffer(init_binary).expect("loading init WASM module");

    debug!("Initializing 'init' process...");
    let main = ModuleInstance::new(&module, &ImportsBuilder::new().with_resolver("env", &runtime))
        .expect("Failed to instantiate module")
        .run_start(&mut runtime)
        .expect("Failed to run start function in module");

    if let Some(wasmi::ExternVal::Memory(memory)) = main.export_by_name("memory") {
        runtime.memory = Some(memory);
    }

    debug!("Running main function...");
    let result = main.invoke_export("main", &[], &mut runtime);

    debug!("Main finished with result: {:?}", result);
}

struct WasmRuntime {
    memory: Option<wasmi::MemoryRef>,
}


const CONSOLE_LOG_INDEX: usize = 1;

impl WasmRuntime {
    fn new() -> Self {
        WasmRuntime {
            memory: None
        }
    }

    fn check_signature(&self, index: usize, signature: &wasmi::Signature) -> bool {
        let (params, ret_ty): (&[wasmi::ValueType], Option<wasmi::ValueType>) = match index {
            CONSOLE_LOG_INDEX => (&[wasmi::ValueType::I32, wasmi::ValueType::I32], None),

            _ => return false,
        };

        signature.params() == params && signature.return_type() == ret_ty
    }
}

impl wasmi::ModuleImportResolver for WasmRuntime {
    fn resolve_func(&self, field_name: &str, signature: &wasmi::Signature) -> Result<wasmi::FuncRef, wasmi::Error> {
        let index = match field_name {
            "console_log" => CONSOLE_LOG_INDEX,
            _ => {
                error!("Export {} not found", field_name);
                return Err(wasmi::Error::Instantiation("Export not found".to_owned()));
            }
        };

        if !self.check_signature(index, signature) {
            error!("Invalid signature type for {:?}: {:?}", field_name, signature);
            return Err(wasmi::Error::Instantiation("Export doesnt match expected type".to_owned()));
        }

        Ok(wasmi::FuncInstance::alloc_host(signature.clone(), index))
    }

    fn resolve_memory(
        &self,
        field_name: &str,
        _memory_type: &wasmi::MemoryDescriptor,
    ) -> Result<wasmi::MemoryRef, wasmi::Error> {
        error!("Export {} not found", field_name);
        Err(wasmi::Error::Instantiation("Export not found".to_owned()))
    }
}

impl wasmi::Externals for WasmRuntime {
    fn invoke_index(
        &mut self,
        index: usize,
        args: wasmi::RuntimeArgs,
    ) -> Result<Option<wasmi::RuntimeValue>, wasmi::Trap> {
        match index {
            CONSOLE_LOG_INDEX => {
                let ptr: u32 = args.nth(0);
                let len: u32 = args.nth(1);

                let memory = self.memory.as_ref()
                    .expect("Function 'get_mem' expects attached memory");

                let mut buf = Vec::new();
                buf.resize(len as usize, 0);
                memory.get_into(ptr, &mut buf).unwrap();

                let string = alloc::str::from_utf8(&buf).expect("failed to parse string");
                info!("{}", string);

                Ok(None)
            }

            _ => panic!("env doesn't provide function at index {}", index),
        }
    }
}
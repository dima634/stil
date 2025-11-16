use std::sync::{LazyLock, Mutex};

mod cpu {
    use super::system;

    pub fn get_usage() -> ffi::CpuUsage {
        let mut system = system();
        system.refresh_cpu_usage();

        let mut usage = ffi::CpuUsage {
            cores: [0.0; _],
            num_cores: system.cpus().len(),
            total: system.global_cpu_usage(),
        };

        for (i, cpu) in system.cpus().iter().enumerate() {
            usage.cores[i] = cpu.cpu_usage();
        }

        usage
    }

    pub fn get_brand() -> String {
        system()
            .cpus()
            .first()
            .map_or("Unknown".to_string(), |cpu| cpu.brand().to_string())
    }

    #[cxx::bridge(namespace = "core::cpu")]
    mod ffi {
        pub struct CpuUsage {
            pub cores: [f32; 64], // avoid allocation
            pub num_cores: usize,
            pub total: f32,
        }

        extern "Rust" {
            fn get_usage() -> CpuUsage;
            fn get_brand() -> String;
        }
    }
}

static SYSTEM: LazyLock<Mutex<sysinfo::System>> = LazyLock::new(|| Mutex::new(sysinfo::System::new()));

#[inline]
fn system() -> std::sync::MutexGuard<'static, sysinfo::System> {
    SYSTEM.lock().expect("should not be poisoned")
}

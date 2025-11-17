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
        let mut system = system();
        system.refresh_cpu_usage();
        system
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

mod memory {
    use super::system;

    pub fn get_memory_usage() -> ffi::MemoryUsage {
        let mut system = system();
        system.refresh_memory();

        ffi::MemoryUsage {
            totalRam: system.total_memory(),
            usedRam: system.used_memory(),
            freeRam: system.free_memory(),
            availableRam: system.available_memory(),
            totalSwap: system.total_swap(),
            usedSwap: system.used_swap(),
            freeSwap: system.free_swap(),
        }
    }

    #[cxx::bridge(namespace = "core::memory")]
    mod ffi {
        pub struct MemoryUsage {
            pub totalRam: u64,
            pub usedRam: u64,
            pub freeRam: u64,
            pub availableRam: u64,
            pub totalSwap: u64,
            pub usedSwap: u64,
            pub freeSwap: u64,
        }

        extern "Rust" {
            fn get_memory_usage() -> MemoryUsage;
        }
    }
}

static SYSTEM: LazyLock<Mutex<sysinfo::System>> = LazyLock::new(|| Mutex::new(sysinfo::System::new()));

#[inline]
fn system() -> std::sync::MutexGuard<'static, sysinfo::System> {
    SYSTEM.lock().expect("should not be poisoned")
}

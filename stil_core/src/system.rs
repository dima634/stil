mod dbus {
    use crate::service_locator::ServiceLocator;

    #[zbus::proxy(
        default_service = "org.freedesktop.login1",
        default_path = "/org/freedesktop/login1",
        interface = "org.freedesktop.login1.Manager"
    )]
    trait Login1Manager {
        fn power_off(&self, interactive: bool) -> zbus::Result<()>;
        fn reboot(&self, interactive: bool) -> zbus::Result<()>;
    }

    pub fn power_off() -> bool {
        Login1ManagerProxyBlocking::new(&ServiceLocator::system_dbus())
            .and_then(|proxy| proxy.power_off(true))
            .is_ok()
    }

    pub fn reboot() -> bool {
        Login1ManagerProxyBlocking::new(&ServiceLocator::system_dbus())
            .and_then(|proxy| proxy.reboot(true))
            .is_ok()
    }

    #[cxx::bridge(namespace = "core::system")]
    mod ffi {
        extern "Rust" {
            fn power_off() -> bool;
            fn reboot() -> bool;
        }
    }
}

mod cpu {
    use crate::service_locator::ServiceLocator;

    pub fn get_usage() -> ffi::CpuUsage {
        let mut system = ServiceLocator::system_info();
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

    pub fn get_temp() -> f32 {
        ServiceLocator::components_info()
            .list_mut()
            .into_iter()
            .find(|comp| comp.label() == "k10temp Tctl")
            .map(|comp| {
                comp.refresh();
                comp.temperature()
            })
            .flatten()
            .unwrap_or(0.0)
    }

    pub fn get_brand() -> String {
        let mut system = ServiceLocator::system_info();
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
            fn get_temp() -> f32;
        }
    }
}

mod memory {
    use crate::service_locator::ServiceLocator;

    pub fn get_memory_usage() -> ffi::MemoryUsage {
        let mut system = ServiceLocator::system_info();
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

// mod dbus {
//     #[zbus::proxy(
//         default_service = "org.freedesktop.login1",
//         default_path = "/org/freedesktop/login1",
//         interface = "org.freedesktop.login1.Manager"
//     )]
//     trait Login1Manager {
//         fn power_off(&self, interactive: bool) -> zbus::Result<()>;
//         fn reboot(&self, interactive: bool) -> zbus::Result<()>;
//     }

//     pub fn power_off() -> bool {
//         Login1ManagerProxyBlocking::new(&ServiceLocator::system_dbus())
//             .and_then(|proxy| proxy.power_off(true))
//             .is_ok()
//     }

//     pub fn reboot() -> bool {
//         Login1ManagerProxyBlocking::new(&ServiceLocator::system_dbus())
//             .and_then(|proxy| proxy.reboot(true))
//             .is_ok()
//     }
// }

// mod cpu {
//     pub struct CpuUsage {
//         pub cores: [f32; 64], // avoid allocation
//         pub num_cores: usize,
//         pub total: f32,
//     }
//     pub fn get_usage() -> CpuUsage {
//         let mut system = ServiceLocator::system_info();
//         system.refresh_cpu_usage();

//         let mut usage = CpuUsage {
//             cores: [0.0; _],
//             num_cores: system.cpus().len(),
//             total: system.global_cpu_usage(),
//         };

//         for (i, cpu) in system.cpus().iter().enumerate() {
//             usage.cores[i] = cpu.cpu_usage();
//         }

//         usage
//     }

//     pub fn get_temp() -> f32 {
//         ServiceLocator::components_info()
//             .list_mut()
//             .into_iter()
//             .find(|comp| comp.label() == "k10temp Tctl")
//             .map(|comp| {
//                 comp.refresh();
//                 comp.temperature()
//             })
//             .flatten()
//             .unwrap_or(0.0)
//     }

//     pub fn get_brand() -> String {
//         let mut system = ServiceLocator::system_info();
//         system.refresh_cpu_usage();
//         system
//             .cpus()
//             .first()
//             .map_or("Unknown".to_string(), |cpu| cpu.brand().to_string())
//     }
// }

// mod memory {
//     use crate::service_locator::ServiceLocator;

//     pub struct MemoryUsage {
//         pub total_ram: u64,
//         pub used_ram: u64,
//         pub free_ram: u64,
//         pub available_ram: u64,
//         pub total_swap: u64,
//         pub used_swap: u64,
//         pub free_swap: u64,
//     }

//     pub fn get_memory_usage() -> MemoryUsage {
//         let mut system = ServiceLocator::system_info();
//         system.refresh_memory();

//         MemoryUsage {
//             total_ram: system.total_memory(),
//             used_ram: system.used_memory(),
//             free_ram: system.free_memory(),
//             available_ram: system.available_memory(),
//             total_swap: system.total_swap(),
//             used_swap: system.used_swap(),
//             free_swap: system.free_swap(),
//         }
//     }
// }

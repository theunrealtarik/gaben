extern crate windows;

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::path::PathBuf;

use sysinfo::{Pid, System};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use windows::Win32::System::ProcessStatus::{GetModuleBaseNameA, GetModuleFileNameExA};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

use super::prelude::*;

#[derive(Debug)]
pub struct Memory {
    pub process_id: Pid,
    pub process_handle: HANDLE,
    pub snap_handle: HANDLE,
    pub base_module: Module,
    pub modules: HashMap<String, Module>,
}

#[allow(dead_code)]
impl Memory {
    pub fn new(process_name: &str) -> Result<Self, String> {
        let mut modules: HashMap<String, Module> = HashMap::new();
        let mut system = System::new();
        system.refresh_all();

        let Some(process_id) = system
            .processes()
            .iter()
            .find(|(_, process)| process.name().to_lowercase() == process_name.to_lowercase())
            .map(|(_, p)| p.pid())
        else {
            return Err("process was not found".to_string());
        };

        let Ok(snap_handle) = (unsafe {
            CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id.as_u32())
        }) else {
            return Err("failed to create a snapshot for modules".to_string());
        };

        let Ok(process_handle) =
            (unsafe { OpenProcess(PROCESS_ALL_ACCESS, false, process_id.as_u32()) })
        else {
            return Err("failed to open process".to_string());
        };

        let mut entry = MODULEENTRY32::default();
        entry.dwSize = std::mem::size_of_val(&entry) as u32;

        let base_module = match unsafe { Module32First(snap_handle, &mut entry) } {
            Ok(_) => Module::from(entry),
            Err(_) => return Err(String::from("failed to retrieve process base module")),
        };

        loop {
            let module_handle = entry.hModule;

            let mut filename: [u8; u8::MAX as usize] = [0u8; u8::MAX as usize];
            let mut basename: [u8; u8::MAX as usize] = [0u8; u8::MAX as usize];

            unsafe {
                GetModuleFileNameExA(process_handle, module_handle, &mut filename);
                GetModuleBaseNameA(process_handle, module_handle, &mut basename);
            }

            let mut module = Module::from(entry);
            module.path = PathBuf::from(
                String::from_utf8_lossy(&filename)
                    .trim_end_matches(char::from(0))
                    .to_string(),
            );
            module.name = String::from_utf8_lossy(&basename)
                .trim_end_matches(char::from(0))
                .to_string()
                .to_lowercase();
            module.id = entry.th32ModuleID;
            modules.insert(module.name.clone(), module);

            if unsafe { Module32Next(snap_handle, &mut entry).is_err() } {
                break;
            }
        }

        unsafe { CloseHandle(snap_handle).unwrap() };
        Ok(Self {
            process_id,
            process_handle,
            snap_handle,
            base_module,
            modules,
        })
    }

    pub fn read<T>(&self, address: LPBYTE) -> Result<T, windows::core::Error> {
        let mut buffer: MaybeUninit<T> = MaybeUninit::uninit();

        match unsafe {
            ReadProcessMemory(
                self.process_handle,
                address.0 as *const c_void,
                buffer.as_mut_ptr() as *mut c_void,
                std::mem::size_of::<T>(),
                None,
            )
        } {
            Ok(_) => Ok(unsafe { buffer.assume_init() }),
            Err(err) => Err(err),
        }
    }

    pub fn write<T>(&self, address: u32, data: T) -> Result<usize, ()> {
        let mut bytes: usize = 0;
        match unsafe {
            WriteProcessMemory(
                self.process_handle,
                address as *const c_void,
                &data as *const T as *const c_void,
                std::mem::size_of_val(&data),
                Some(&mut bytes),
            )
        } {
            Ok(_) => Ok(bytes),
            Err(_) => Err(()),
        }
    }

    pub fn close_process_handle(&self) -> Result<(), windows::core::Error> {
        unsafe { CloseHandle(self.process_handle) }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        self.close_process_handle().unwrap();
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub address: LPBYTE,
    pub size: u32,
    pub id: u32,
    entry: MODULEENTRY32,
}

impl Module {
    pub fn entry(&self) -> MODULEENTRY32 {
        self.entry
    }
}

impl From<MODULEENTRY32> for Module {
    fn from(entry: MODULEENTRY32) -> Self {
        Self {
            name: stringify_bytes_u8(entry.szModule.iter().map(|&i| i as u8).collect::<Vec<u8>>()),
            path: PathBuf::from(stringify_bytes_u8(
                entry
                    .szExePath
                    .iter()
                    .map(|&i| i as u8)
                    .collect::<Vec<u8>>(),
            )),
            address: LPBYTE(entry.modBaseAddr),
            size: entry.dwSize,
            id: entry.th32ModuleID,
            entry,
        }
    }
}

impl std::fmt::Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field("path", &self.path)
            .field("address", &self.address)
            .field("size", &self.size)
            .field("id", &self.id)
            .finish()
    }
}

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

        let mut entry = MODULEENTRY32 {
            dwSize: std::mem::size_of::<MODULEENTRY32>() as u32,
            ..Default::default()
        };

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

    pub fn read<T>(&self, address: usize) -> Result<T, windows::core::Error> {
        let mut buffer: MaybeUninit<T> = MaybeUninit::uninit();

        match unsafe {
            ReadProcessMemory(
                self.process_handle,
                address as LPCVOID,
                buffer.as_mut_ptr() as LPVOID,
                std::mem::size_of::<T>(),
                None,
            )
        } {
            Ok(_) => Ok(unsafe { buffer.assume_init() }),
            Err(err) => Err(err),
        }
    }

    pub fn write<T>(&self, address: usize, data: T) -> Result<usize, windows::core::Error> {
        let mut bytes: usize = 0;
        match unsafe {
            WriteProcessMemory(
                self.process_handle,
                address as LPCVOID,
                &data as *const T as LPCVOID,
                std::mem::size_of_val(&data),
                Some(&mut bytes),
            )
        } {
            Ok(_) => Ok(bytes),
            Err(err) => Err(err),
        }
    }

    pub fn calculate_pointer(
        &self,
        base_address: usize,
        offsets: &[usize],
    ) -> Result<usize, windows::core::Error> {
        let mut current_address = base_address;

        if offsets.is_empty() {
            Ok(current_address)
        } else {
            for &offset in offsets.iter() {
                match self.read::<usize>(current_address) {
                    Ok(addr) => {
                        current_address = addr + offset;
                    }
                    Err(err) => return Err(err),
                }
            }

            Ok(current_address)
        }
    }

    pub fn read_pointer<T>(
        &self,
        address: usize,
        offsets: Option<&[usize]>,
    ) -> Result<T, windows::core::Error> {
        let offsets = match offsets {
            Some(o) => o,
            None => &[],
        };

        match self.calculate_pointer(address, offsets) {
            Ok(address) => self.read::<T>(address),
            Err(err) => Err(err),
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
#[derive(Default, Debug)]
pub struct Module {
    pub name: String,
    pub path: PathBuf,
    pub address: usize,
    pub size: u32,
    pub id: u32,
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
            address: entry.modBaseAddr as usize,
            size: entry.dwSize,
            id: entry.th32ModuleID,
        }
    }
}

//! Procedural Macros for #[derive(XSCommand)] Implementation
//! 

extern crate proc_macro;
extern crate quote;
extern crate syn;
// extern crate proc_macro2;
// extern crate core;

use  proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(XSCommand)]
pub fn xscommand_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let gen = quote! {
        impl<'a> XSCommand<'a, DefaultErr> for #name<'a> {
            fn set_exe(path: &str) -> Self {
                let exe  = Command::new(path);
                let args = Vec::new();
                Self {
                    exe,
                    args,
                    work_dir: None,
                }
            }

            fn set_args(&mut self, args: Vec<&'a str>) -> Result<(), DefaultErr> {
                // TODO: check the rationality of args
                // Consider create a list to store available argements
                self.args = args;
                Ok(())
            }

            fn get_args(&self) -> Vec<&str> {
                // let mut args = Vec::new();
                // for arg in &self.args {
                //     args.push(*arg);
                // }
                // I can write beautiful code like this now!
                let args: Vec<&str> = self.args.iter().map(|a| *a).collect();
                args
            }
            
            fn set_workdir(&mut self, work_dir: Option<&'a str>) -> Result<(), DefaultErr> {
                // TODO: check the rationality of workdir
                // Consider checking if the workdir readable and writable
                self.work_dir = work_dir;
                Ok(())
            }

            fn excute(&mut self, stdout: Option<&str>, stderr: Option<&str>) -> Result<i32, DefaultErr> {
                for arg in &self.args {
                    self.exe.arg(arg);
                }
                let workload = if let Some(dir) = self.work_dir { dir } else { "./" };
                log::info!("{} excute args: {:?} in workload: {}", stringify!(#name).to_ascii_lowercase(), self.args, workload);
                // TODO: use clouse here to reduce code lines
                if let Some(stdout_path) = stdout {
                    let stdout_fd = match File::create(stdout_path) {
                        Ok(fd) => {
                            fd.into_raw_fd()
                        },
                        Err(_) => {
                            return Err(DefaultErr::ExcuteErr(3));
                        }
                    };
                    // let stdout_fd = File::create(stdout_path).unwrap().into_raw_fd();
                    let std_out = unsafe { Stdio::from_raw_fd(stdout_fd) };
                    self.exe.stdout(std_out);
                }
                if let Some(stderr_path) = stderr {
                    let stderr_fd = match File::create(stderr_path) {
                        Ok(fd) => {
                            fd.into_raw_fd()
                        },
                        Err(_) => {
                            return Err(DefaultErr::ExcuteErr(4));
                        }
                    };
                    let err_out = unsafe { Stdio::from_raw_fd(stderr_fd) };
                    self.exe.stderr(err_out);
                }
                if let Some(dir) = self.work_dir {
                    self.exe.current_dir(dir);
                }
                // Block here until command return
                let res = self.exe.status();
                match res {
                    Ok(exit_status) => {
                        if let Some(exit_code) = exit_status.code() {
                            log::info!("{} excute with exit code: {}", stringify!(#name).to_ascii_lowercase(), exit_code);
                            Ok(exit_code)
                        } else {
                            return Err(DefaultErr::ExcuteErr(5));
                        }
                    },
                    Err(_) => {
                        return Err(DefaultErr::ExcuteErr(6));
                    }
                }
            }

            fn to_string(&self) -> String {
                let mut name = stringify!(#name).to_lowercase();
                for arg in &self.args {
                    name.push_str(" ");
                    name.push_str(*arg);
                }
                name
            }
        }
    };
    gen.into()
}




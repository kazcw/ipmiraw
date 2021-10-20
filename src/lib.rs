pub mod si {
    use libc::{c_int, O_NONBLOCK};
    use std::convert::TryInto;
    use std::fs::{File, OpenOptions};
    use std::io;
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::io::AsRawFd;
    use std::os::unix::io::RawFd;
    use std::path::Path;

    #[link(name = "ipmiraw")]
    extern "C" {
        fn si_cmd(fd: RawFd, netfn: u8, cmd: u8, data: *const u8, data_len: u16) -> c_int;
    }

    pub struct Ipmi {
        f: File,
    }

    impl Ipmi {
        #[inline]
        pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
            Ok(Ipmi {
                f: OpenOptions::new()
                    .write(true)
                    .custom_flags(O_NONBLOCK)
                    .open(path)?,
            })
        }

        #[inline]
        pub fn cmd(&self, n: u8, c: u8, d: &[u8]) -> io::Result<()> {
            let result = unsafe {
                si_cmd(
                    self.f.as_raw_fd(),
                    n,
                    c,
                    d.as_ptr(),
                    d.len()
                        .try_into()
                        .expect("IPMI message must not exceed 64KiB"),
                )
            };
            match result {
                0 => Ok(()),
                e => Err(io::Error::from_raw_os_error(e as i32)),
            }
        }
    }
}

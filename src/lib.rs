pub mod si {
    #[link(name = "ipmiraw")]
    extern "C" {
        fn si_cmd(fd: RawFd, netfn: u8, cmd: u8, data: *mut u8, data_len: u16) -> c_int;
    }

    use libc::{c_int, O_NONBLOCK};
    use std::convert::TryInto;
    use std::fs::{File, OpenOptions};
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::io::AsRawFd;
    use std::os::unix::io::RawFd;
    use std::path::Path;

    pub struct Ipmi {
        f: File,
    }

    impl Ipmi {
        #[inline]
        pub fn open(path: impl AsRef<Path>) -> Result<Self, ()> {
            Ok(Ipmi {
                f: OpenOptions::new()
                    .write(true)
                    .custom_flags(O_NONBLOCK)
                    .open(path)
                    .unwrap(),
            })
        }

        #[inline]
        pub fn cmd(&self, n: u8, c: u8, d: &mut [u8]) -> u8 {
            debug_assert!(d.len() > 0);
            unsafe {
                si_cmd(
                    self.f.as_raw_fd(),
                    n,
                    c,
                    d.as_mut_ptr(),
                    d.len().try_into().unwrap(),
                );
            }
            d[0]
        }
    }
}

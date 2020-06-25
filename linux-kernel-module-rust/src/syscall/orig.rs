use {
    super::*,
}
mod cshim {
    extern "C" {
        pub fn read(fd : u32, buf : *mut u8, count : usize);

        pub fn write(fd : u32, buf : *const u8, count : usize);

        pub fn openat(dfd : i32, filename : *const u8, flags : i32, mode : u16);

        pub fn close(fd : u32);

        pub fn fstat(fd : u32, statbuf : * const u8);

        pub fn newfstatat(dfd : i32, filename : *const u8, statbuf : * const u8, flag : i32);

        pub fn lseek(fd : u32, offset : i64, whence : u32);

        pub fn ioctl(fd : u32, cmd : u32, arg : u64);

        pub fn pread64(fd : u32, buf : *mut u8, count : usize, pos : i64);

        pub fn pwrite64(fd : u32, buf : *const u8, count : usize, pos : i64);

        pub fn readv(fd : u64, vec : * const u8, vlen : u64);

        pub fn writev(fd : u64, vec : * const u8, vlen : u64);

        pub fn sendfile(out_fd : i32, in_fd : i32, offset : *mut i64, count : usize);

        pub fn fcntl(fd : u32, cmd : u32, arg : u64);

        pub fn flock(fd : u32, cmd : u32);

        pub fn fsync(fd : u32);

        pub fn fdatasync(fd : u32);

        pub fn truncate(path : *const u8, length : i64);

        pub fn ftruncate(fd : u32, length : u64);

        pub fn getdents64(fd : u32, dirent : * const u8, count : u32);

        pub fn getcwd(buf : *mut u8, size : u64);

        pub fn chdir(filename : *const u8);

        pub fn renameat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8);

        pub fn mkdirat(dfd : i32, pathname : *const u8, mode : u16);

        pub fn linkat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8, flags : i32);

        pub fn unlinkat(dfd : i32, pathname : *const u8, flag : i32);

        pub fn symlinkat(oldname : *const u8, newdfd : i32, newname : *const u8);

        pub fn readlinkat(dfd : i32, path : *const u8, buf : *mut u8, bufsiz : i32);

        pub fn fchmod(fd : u32, mode : u16);

        pub fn fchmodat(dfd : i32, filename : *const u8, mode : u16);

        pub fn fchown(fd : u32, user : u32, group : u32);

        pub fn fchownat(dfd : i32, filename : *const u8, user : u32, group : u32, flag : i32);

        pub fn faccessat(dfd : i32, filename : *const u8, mode : i32);

        pub fn dup3(oldfd : u32, newfd : u32, flags : i32);

        pub fn utimensat(dfd : i32, filename : *const u8, utimes : * const u8, flags : i32);

        pub fn copy_file_range(fd_in : i32, off_in : *mut i64, fd_out : i32, off_out : *mut i64, len : usize, flags : u32);

        pub fn statfs(path : *const u8, buf : * const u8);

        pub fn fstatfs(fd : u32, buf : * const u8);

        pub fn sync();

        pub fn mount(dev_name : *mut u8, dir_name : *mut u8, type : *mut u8, flags : u64, data : *mut u8);

        pub fn umount2( : u64,  : u64,  : u64,  : u64,  : u64,  : u64);

        pub fn brk(brk : u64);

        pub fn mmap( : u64,  : u64,  : u64,  : u64,  : u64,  : u64);

        pub fn mprotect(start : u64, len : usize, prot : u64);

        pub fn munmap(addr : u64, len : usize);

        pub fn madvise(start : u64, len : usize, behavior : i32);

        pub fn rt_sigaction(int : i32,  : * const u8,  : * const u8, size_t : usoze);

        pub fn rt_sigprocmask(how : i32, set : u64, oset : u64, sigsetsize : usize);

        pub fn sigaltstack(uss : * const u8, uoss : * const u8);

        pub fn clone(long : u64, long : u64,  : *mut i32,  : *mut i32, long : u64);

        pub fn execve(filename : *const u8, argv : *const *const u8, envp : *const *const u8);

        pub fn exit(error_code : i32);

        pub fn exit_group(error_code : i32);

        pub fn wait4(pid : i32, stat_addr : *mut i32, options : i32, ru : * const u8);

        pub fn set_tid_address(tidptr : *mut i32);

        pub fn futex(uaddr : *mut u32, op : i32, val : u32, utime : * const u8, uaddr2 : *mut u32, val3 : u32);

        pub fn tkill(pid : i32, sig : i32);

        pub fn setitimer(which : i32, value : * const u8, ovalue : * const u8);

        pub fn clock_gettime(which_clock : i32, tp : * const u8);

        pub fn getpid();

        pub fn gettid();

        pub fn uname( : * const u8);

        pub fn umask(mask : i32);

        pub fn getuid();

        pub fn getgid();

        pub fn setuid(uid : u32);

        pub fn geteuid();

        pub fn getegid();

        pub fn setpgid(pid : i32, pgid : i32);

        pub fn getppid();

        pub fn setsid();

        pub fn getpgid(pid : i32);

        pub fn getgroups(gidsetsize : i32, grouplist : *mut u32);

        pub fn setgroups(gidsetsize : i32, grouplist : *mut u32);

        pub fn prctl(option : i32, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64);

        pub fn membarrier(cmd : i32, flags : i32);

        pub fn rt_sigqueueinfo(pid : i32, sig : i32, uinfo : * const u8);

        pub fn finit_module(fd : i32, uargs : *const u8, flags : i32);

        pub fn open(filename : *const u8, flags : i32, mode : u16);

        pub fn stat(filename : *const u8, statbuf : * const u8);

        pub fn lstat(filename : *const u8, statbuf : * const u8);

        pub fn access(filename : *const u8, mode : i32);

        pub fn dup2(oldfd : u32, newfd : u32);

        pub fn fork();

        pub fn vfork();

        pub fn rename(oldname : *const u8, newname : *const u8);

        pub fn mkdir(pathname : *const u8, mode : u16);

        pub fn rmdir(pathname : *const u8);

        pub fn link(oldname : *const u8, newname : *const u8);

        pub fn unlink(pathname : *const u8);

        pub fn readlink(path : *const u8, buf : *mut u8, bufsiz : i32);

        pub fn arch_prctl( : u64,  : u64,  : u64,  : u64,  : u64,  : u64);

    }
}

use {
    super::*,
};

pub mod user {
    extern "C" {
        pub fn read(fd : u32, buf : *mut u8, count : usize) -> i64;

        pub fn write(fd : u32, buf : *const u8, count : usize) -> i64;

        pub fn openat(dfd : i32, filename : *const u8, flags : i32, mode : u16) -> i64;

        pub fn close(fd : u32) -> i64;

        pub fn fstat(fd : u32, statbuf : * const u8) -> i64;

        pub fn newfstatat(dfd : i32, filename : *const u8, statbuf : * const u8, flag : i32) -> i64;

        pub fn lseek(fd : u32, offset : i64, whence : u32) -> i64;

        pub fn ioctl(fd : u32, cmd : u32, arg : u64) -> i64;

        pub fn pread64(fd : u32, buf : *mut u8, count : usize, pos : i64) -> i64;

        pub fn pwrite64(fd : u32, buf : *const u8, count : usize, pos : i64) -> i64;

        pub fn readv(fd : u64, vec : * const u8, vlen : u64) -> i64;

        pub fn writev(fd : u64, vec : * const u8, vlen : u64) -> i64;

        pub fn sendfile(out_fd : i32, in_fd : i32, offset : *mut i64, count : usize) -> i64;

        pub fn fcntl(fd : u32, cmd : u32, arg : u64) -> i64;

        pub fn flock(fd : u32, cmd : u32) -> i64;

        pub fn fsync(fd : u32) -> i64;

        pub fn fdatasync(fd : u32) -> i64;

        pub fn truncate(path : *const u8, length : i64) -> i64;

        pub fn ftruncate(fd : u32, length : u64) -> i64;

        pub fn getdents64(fd : u32, dirent : * const u8, count : u32) -> i64;

        pub fn getcwd(buf : *mut u8, size : u64) -> i64;

        pub fn chdir(filename : *const u8) -> i64;

        pub fn renameat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8) -> i64;

        pub fn mkdirat(dfd : i32, pathname : *const u8, mode : u16) -> i64;

        pub fn linkat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8, flags : i32) -> i64;

        pub fn unlinkat(dfd : i32, pathname : *const u8, flag : i32) -> i64;

        pub fn symlinkat(oldname : *const u8, newdfd : i32, newname : *const u8) -> i64;

        pub fn readlinkat(dfd : i32, path : *const u8, buf : *mut u8, bufsiz : i32) -> i64;

        pub fn fchmod(fd : u32, mode : u16) -> i64;

        pub fn fchmodat(dfd : i32, filename : *const u8, mode : u16) -> i64;

        pub fn fchown(fd : u32, user : u32, group : u32) -> i64;

        pub fn fchownat(dfd : i32, filename : *const u8, user : u32, group : u32, flag : i32) -> i64;

        pub fn faccessat(dfd : i32, filename : *const u8, mode : i32) -> i64;

        pub fn dup3(oldfd : u32, newfd : u32, flags : i32) -> i64;

        pub fn utimensat(dfd : i32, filename : *const u8, utimes : * const u8, flags : i32) -> i64;

        pub fn copy_file_range(fd_in : i32, off_in : *mut i64, fd_out : i32, off_out : *mut i64, len : usize, flags : u32) -> i64;

        pub fn statfs(path : *const u8, buf : * const u8) -> i64;

        pub fn fstatfs(fd : u32, buf : * const u8) -> i64;

        pub fn sync() -> i64;

        pub fn mount(dev_name : *mut u8, dir_name : *mut u8, ty : *mut u8, flags : u64, data : *mut u8) -> i64;

        pub fn umount2(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

        pub fn brk(brk : u64) -> i64;

        pub fn mmap(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

        pub fn mprotect(start : u64, len : usize, prot : u64) -> i64;

        pub fn munmap(addr : u64, len : usize) -> i64;

        pub fn madvise(start : u64, len : usize, behavior : i32) -> i64;

        pub fn rt_sigaction(arg1 : i32, arg2 : * const u8, arg3 : * const u8, size_t : usize) -> i64;

        pub fn rt_sigprocmask(how : i32, set : u64, oset : u64, sigsetsize : usize) -> i64;

        pub fn sigaltstack(uss : * const u8, uoss : * const u8) -> i64;

        pub fn clone(arg1 : * const u8, arg2 : * const u8, arg3 : *mut i32, arg4 : *mut i32, arg5 : * const u8) -> i64;

        pub fn execve(filename : *const u8, argv : *const *const u8, envp : *const *const u8) -> i64;

        pub fn exit(error_code : i32) -> i64;

        pub fn exit_group(error_code : i32) -> i64;

        pub fn wait4(pid : i32, stat_addr : *mut i32, options : i32, ru : * const u8) -> i64;

        pub fn set_tid_address(tidptr : *mut i32) -> i64;

        pub fn futex(uaddr : *mut u32, op : i32, val : u32, utime : * const u8, uaddr2 : *mut u32, val3 : u32) -> i64;

        pub fn tkill(pid : i32, sig : i32) -> i64;

        pub fn setitimer(which : i32, value : * const u8, ovalue : * const u8) -> i64;

        pub fn clock_gettime(which_clock : i32, tp : * const u8) -> i64;

        pub fn getpid() -> i64;

        pub fn gettid() -> i64;

        pub fn uname(arg1 : * const u8) -> i64;

        pub fn umask(mask : i32) -> i64;

        pub fn getuid() -> i64;

        pub fn getgid() -> i64;

        pub fn setuid(uid : u32) -> i64;

        pub fn geteuid() -> i64;

        pub fn getegid() -> i64;

        pub fn setpgid(pid : i32, pgid : i32) -> i64;

        pub fn getppid() -> i64;

        pub fn setsid() -> i64;

        pub fn getpgid(pid : i32) -> i64;

        pub fn getgroups(gidsetsize : i32, grouplist : *mut u32) -> i64;

        pub fn setgroups(gidsetsize : i32, grouplist : *mut u32) -> i64;

        pub fn prctl(option : i32, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64) -> i64;

        pub fn membarrier(cmd : i32, flags : i32) -> i64;

        pub fn rt_sigqueueinfo(pid : i32, sig : i32, uinfo : * const u8) -> i64;

        pub fn finit_module(fd : i32, uargs : *const u8, flags : i32) -> i64;

        pub fn open(filename : *const u8, flags : i32, mode : u16) -> i64;

        pub fn stat(filename : *const u8, statbuf : * const u8) -> i64;

        pub fn lstat(filename : *const u8, statbuf : * const u8) -> i64;

        pub fn access(filename : *const u8, mode : i32) -> i64;

        pub fn dup2(oldfd : u32, newfd : u32) -> i64;

        pub fn fork() -> i64;

        pub fn vfork() -> i64;

        pub fn rename(oldname : *const u8, newname : *const u8) -> i64;

        pub fn mkdir(pathname : *const u8, mode : u16) -> i64;

        pub fn rmdir(pathname : *const u8) -> i64;

        pub fn link(oldname : *const u8, newname : *const u8) -> i64;

        pub fn unlink(pathname : *const u8) -> i64;

        pub fn readlink(path : *const u8, buf : *mut u8, bufsiz : i32) -> i64;

        pub fn arch_prctl(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

    }
}

pub mod kern {
    #[inline]
    pub fn read(fd : u32, buf : *mut u8, count : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::read(fd, buf, count) }
    }

    #[inline]
    pub fn write(fd : u32, buf : *const u8, count : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::write(fd, buf, count) }
    }

    #[inline]
    pub fn openat(dfd : i32, filename : *const u8, flags : i32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::openat(dfd, filename, flags, mode) }
    }

    #[inline]
    pub fn close(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::close(fd) }
    }

    #[inline]
    pub fn fstat(fd : u32, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fstat(fd, statbuf) }
    }

    #[inline]
    pub fn newfstatat(dfd : i32, filename : *const u8, statbuf : * const u8, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::newfstatat(dfd, filename, statbuf, flag) }
    }

    #[inline]
    pub fn lseek(fd : u32, offset : i64, whence : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::lseek(fd, offset, whence) }
    }

    #[inline]
    pub fn ioctl(fd : u32, cmd : u32, arg : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::ioctl(fd, cmd, arg) }
    }

    #[inline]
    pub fn pread64(fd : u32, buf : *mut u8, count : usize, pos : i64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::pread64(fd, buf, count, pos) }
    }

    #[inline]
    pub fn pwrite64(fd : u32, buf : *const u8, count : usize, pos : i64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::pwrite64(fd, buf, count, pos) }
    }

    #[inline]
    pub fn readv(fd : u64, vec : * const u8, vlen : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::readv(fd, vec, vlen) }
    }

    #[inline]
    pub fn writev(fd : u64, vec : * const u8, vlen : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::writev(fd, vec, vlen) }
    }

    #[inline]
    pub fn sendfile(out_fd : i32, in_fd : i32, offset : *mut i64, count : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::sendfile(out_fd, in_fd, offset, count) }
    }

    #[inline]
    pub fn fcntl(fd : u32, cmd : u32, arg : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fcntl(fd, cmd, arg) }
    }

    #[inline]
    pub fn flock(fd : u32, cmd : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::flock(fd, cmd) }
    }

    #[inline]
    pub fn fsync(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fsync(fd) }
    }

    #[inline]
    pub fn fdatasync(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fdatasync(fd) }
    }

    #[inline]
    pub fn truncate(path : *const u8, length : i64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::truncate(path, length) }
    }

    #[inline]
    pub fn ftruncate(fd : u32, length : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::ftruncate(fd, length) }
    }

    #[inline]
    pub fn getdents64(fd : u32, dirent : * const u8, count : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getdents64(fd, dirent, count) }
    }

    #[inline]
    pub fn getcwd(buf : *mut u8, size : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getcwd(buf, size) }
    }

    #[inline]
    pub fn chdir(filename : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::chdir(filename) }
    }

    #[inline]
    pub fn renameat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::renameat(olddfd, oldname, newdfd, newname) }
    }

    #[inline]
    pub fn mkdirat(dfd : i32, pathname : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::mkdirat(dfd, pathname, mode) }
    }

    #[inline]
    pub fn linkat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::linkat(olddfd, oldname, newdfd, newname, flags) }
    }

    #[inline]
    pub fn unlinkat(dfd : i32, pathname : *const u8, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::unlinkat(dfd, pathname, flag) }
    }

    #[inline]
    pub fn symlinkat(oldname : *const u8, newdfd : i32, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::symlinkat(oldname, newdfd, newname) }
    }

    #[inline]
    pub fn readlinkat(dfd : i32, path : *const u8, buf : *mut u8, bufsiz : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::readlinkat(dfd, path, buf, bufsiz) }
    }

    #[inline]
    pub fn fchmod(fd : u32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fchmod(fd, mode) }
    }

    #[inline]
    pub fn fchmodat(dfd : i32, filename : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fchmodat(dfd, filename, mode) }
    }

    #[inline]
    pub fn fchown(fd : u32, user : u32, group : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fchown(fd, user, group) }
    }

    #[inline]
    pub fn fchownat(dfd : i32, filename : *const u8, user : u32, group : u32, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fchownat(dfd, filename, user, group, flag) }
    }

    #[inline]
    pub fn faccessat(dfd : i32, filename : *const u8, mode : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::faccessat(dfd, filename, mode) }
    }

    #[inline]
    pub fn dup3(oldfd : u32, newfd : u32, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::dup3(oldfd, newfd, flags) }
    }

    #[inline]
    pub fn utimensat(dfd : i32, filename : *const u8, utimes : * const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::utimensat(dfd, filename, utimes, flags) }
    }

    #[inline]
    pub fn copy_file_range(fd_in : i32, off_in : *mut i64, fd_out : i32, off_out : *mut i64, len : usize, flags : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::copy_file_range(fd_in, off_in, fd_out, off_out, len, flags) }
    }

    #[inline]
    pub fn statfs(path : *const u8, buf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::statfs(path, buf) }
    }

    #[inline]
    pub fn fstatfs(fd : u32, buf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fstatfs(fd, buf) }
    }

    #[inline]
    pub fn sync() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::sync() }
    }

    #[inline]
    pub fn mount(dev_name : *mut u8, dir_name : *mut u8, ty : *mut u8, flags : u64, data : *mut u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::mount(dev_name, dir_name, ty, flags, data) }
    }

    #[inline]
    pub fn umount2(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::umount2(arg1, arg2, arg3, arg4, arg5, arg6) }
    }

    #[inline]
    pub fn brk(brk : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::brk(brk) }
    }

    #[inline]
    pub fn mmap(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::mmap(arg1, arg2, arg3, arg4, arg5, arg6) }
    }

    #[inline]
    pub fn mprotect(start : u64, len : usize, prot : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::mprotect(start, len, prot) }
    }

    #[inline]
    pub fn munmap(addr : u64, len : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::munmap(addr, len) }
    }

    #[inline]
    pub fn madvise(start : u64, len : usize, behavior : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::madvise(start, len, behavior) }
    }

    #[inline]
    pub fn rt_sigaction(arg1 : i32, arg2 : * const u8, arg3 : * const u8, size_t : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::rt_sigaction(arg1, arg2, arg3, size_t) }
    }

    #[inline]
    pub fn rt_sigprocmask(how : i32, set : u64, oset : u64, sigsetsize : usize) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::rt_sigprocmask(how, set, oset, sigsetsize) }
    }

    #[inline]
    pub fn sigaltstack(uss : * const u8, uoss : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::sigaltstack(uss, uoss) }
    }

    #[inline]
    pub fn clone(arg1 : * const u8, arg2 : * const u8, arg3 : *mut i32, arg4 : *mut i32, arg5 : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::clone(arg1, arg2, arg3, arg4, arg5) }
    }

    #[inline]
    pub fn execve(filename : *const u8, argv : *const *const u8, envp : *const *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::execve(filename, argv, envp) }
    }

    #[inline]
    pub fn exit(error_code : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::exit(error_code) }
    }

    #[inline]
    pub fn exit_group(error_code : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::exit_group(error_code) }
    }

    #[inline]
    pub fn wait4(pid : i32, stat_addr : *mut i32, options : i32, ru : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::wait4(pid, stat_addr, options, ru) }
    }

    #[inline]
    pub fn set_tid_address(tidptr : *mut i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::set_tid_address(tidptr) }
    }

    #[inline]
    pub fn futex(uaddr : *mut u32, op : i32, val : u32, utime : * const u8, uaddr2 : *mut u32, val3 : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::futex(uaddr, op, val, utime, uaddr2, val3) }
    }

    #[inline]
    pub fn tkill(pid : i32, sig : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::tkill(pid, sig) }
    }

    #[inline]
    pub fn setitimer(which : i32, value : * const u8, ovalue : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::setitimer(which, value, ovalue) }
    }

    #[inline]
    pub fn clock_gettime(which_clock : i32, tp : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::clock_gettime(which_clock, tp) }
    }

    #[inline]
    pub fn getpid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getpid() }
    }

    #[inline]
    pub fn gettid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::gettid() }
    }

    #[inline]
    pub fn uname(arg1 : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::uname(arg1) }
    }

    #[inline]
    pub fn umask(mask : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::umask(mask) }
    }

    #[inline]
    pub fn getuid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getuid() }
    }

    #[inline]
    pub fn getgid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getgid() }
    }

    #[inline]
    pub fn setuid(uid : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::setuid(uid) }
    }

    #[inline]
    pub fn geteuid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::geteuid() }
    }

    #[inline]
    pub fn getegid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getegid() }
    }

    #[inline]
    pub fn setpgid(pid : i32, pgid : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::setpgid(pid, pgid) }
    }

    #[inline]
    pub fn getppid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getppid() }
    }

    #[inline]
    pub fn setsid() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::setsid() }
    }

    #[inline]
    pub fn getpgid(pid : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getpgid(pid) }
    }

    #[inline]
    pub fn getgroups(gidsetsize : i32, grouplist : *mut u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::getgroups(gidsetsize, grouplist) }
    }

    #[inline]
    pub fn setgroups(gidsetsize : i32, grouplist : *mut u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::setgroups(gidsetsize, grouplist) }
    }

    #[inline]
    pub fn prctl(option : i32, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::prctl(option, arg2, arg3, arg4, arg5) }
    }

    #[inline]
    pub fn membarrier(cmd : i32, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::membarrier(cmd, flags) }
    }

    #[inline]
    pub fn rt_sigqueueinfo(pid : i32, sig : i32, uinfo : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::rt_sigqueueinfo(pid, sig, uinfo) }
    }

    #[inline]
    pub fn finit_module(fd : i32, uargs : *const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::finit_module(fd, uargs, flags) }
    }

    #[inline]
    pub fn open(filename : *const u8, flags : i32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::open(filename, flags, mode) }
    }

    #[inline]
    pub fn stat(filename : *const u8, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::stat(filename, statbuf) }
    }

    #[inline]
    pub fn lstat(filename : *const u8, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::lstat(filename, statbuf) }
    }

    #[inline]
    pub fn access(filename : *const u8, mode : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::access(filename, mode) }
    }

    #[inline]
    pub fn dup2(oldfd : u32, newfd : u32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::dup2(oldfd, newfd) }
    }

    #[inline]
    pub fn fork() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::fork() }
    }

    #[inline]
    pub fn vfork() -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::vfork() }
    }

    #[inline]
    pub fn rename(oldname : *const u8, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::rename(oldname, newname) }
    }

    #[inline]
    pub fn mkdir(pathname : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::mkdir(pathname, mode) }
    }

    #[inline]
    pub fn rmdir(pathname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::rmdir(pathname) }
    }

    #[inline]
    pub fn link(oldname : *const u8, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::link(oldname, newname) }
    }

    #[inline]
    pub fn unlink(pathname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::unlink(pathname) }
    }

    #[inline]
    pub fn readlink(path : *const u8, buf : *mut u8, bufsiz : i32) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::readlink(path, buf, bufsiz) }
    }

    #[inline]
    pub fn arch_prctl(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        unsafe{ user::arch_prctl(arg1, arg2, arg3, arg4, arg5, arg6) }
    }

}

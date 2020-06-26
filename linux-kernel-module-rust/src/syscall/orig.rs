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
    use super::*;
    #[inline]
    pub unsafe fn read(fd : u32, buf : *mut u8, count : usize) -> i64 {
        let fs = ProtFs::prot();
        user::read(fd, buf, count)
    }

    #[inline]
    pub unsafe fn write(fd : u32, buf : *const u8, count : usize) -> i64 {
        let fs = ProtFs::prot();
        user::write(fd, buf, count)
    }

    #[inline]
    pub unsafe fn openat(dfd : i32, filename : *const u8, flags : i32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::openat(dfd, filename, flags, mode)
    }

    #[inline]
    pub unsafe fn close(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        user::close(fd)
    }

    #[inline]
    pub unsafe fn fstat(fd : u32, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::fstat(fd, statbuf)
    }

    #[inline]
    pub unsafe fn newfstatat(dfd : i32, filename : *const u8, statbuf : * const u8, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        user::newfstatat(dfd, filename, statbuf, flag)
    }

    #[inline]
    pub unsafe fn lseek(fd : u32, offset : i64, whence : u32) -> i64 {
        let fs = ProtFs::prot();
        user::lseek(fd, offset, whence)
    }

    #[inline]
    pub unsafe fn ioctl(fd : u32, cmd : u32, arg : u64) -> i64 {
        let fs = ProtFs::prot();
        user::ioctl(fd, cmd, arg)
    }

    #[inline]
    pub unsafe fn pread64(fd : u32, buf : *mut u8, count : usize, pos : i64) -> i64 {
        let fs = ProtFs::prot();
        user::pread64(fd, buf, count, pos)
    }

    #[inline]
    pub unsafe fn pwrite64(fd : u32, buf : *const u8, count : usize, pos : i64) -> i64 {
        let fs = ProtFs::prot();
        user::pwrite64(fd, buf, count, pos)
    }

    #[inline]
    pub unsafe fn readv(fd : u64, vec : * const u8, vlen : u64) -> i64 {
        let fs = ProtFs::prot();
        user::readv(fd, vec, vlen)
    }

    #[inline]
    pub unsafe fn writev(fd : u64, vec : * const u8, vlen : u64) -> i64 {
        let fs = ProtFs::prot();
        user::writev(fd, vec, vlen)
    }

    #[inline]
    pub unsafe fn sendfile(out_fd : i32, in_fd : i32, offset : *mut i64, count : usize) -> i64 {
        let fs = ProtFs::prot();
        user::sendfile(out_fd, in_fd, offset, count)
    }

    #[inline]
    pub unsafe fn fcntl(fd : u32, cmd : u32, arg : u64) -> i64 {
        let fs = ProtFs::prot();
        user::fcntl(fd, cmd, arg)
    }

    #[inline]
    pub unsafe fn flock(fd : u32, cmd : u32) -> i64 {
        let fs = ProtFs::prot();
        user::flock(fd, cmd)
    }

    #[inline]
    pub unsafe fn fsync(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        user::fsync(fd)
    }

    #[inline]
    pub unsafe fn fdatasync(fd : u32) -> i64 {
        let fs = ProtFs::prot();
        user::fdatasync(fd)
    }

    #[inline]
    pub unsafe fn truncate(path : *const u8, length : i64) -> i64 {
        let fs = ProtFs::prot();
        user::truncate(path, length)
    }

    #[inline]
    pub unsafe fn ftruncate(fd : u32, length : u64) -> i64 {
        let fs = ProtFs::prot();
        user::ftruncate(fd, length)
    }

    #[inline]
    pub unsafe fn getdents64(fd : u32, dirent : * const u8, count : u32) -> i64 {
        let fs = ProtFs::prot();
        user::getdents64(fd, dirent, count)
    }

    #[inline]
    pub unsafe fn getcwd(buf : *mut u8, size : u64) -> i64 {
        let fs = ProtFs::prot();
        user::getcwd(buf, size)
    }

    #[inline]
    pub unsafe fn chdir(filename : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::chdir(filename)
    }

    #[inline]
    pub unsafe fn renameat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::renameat(olddfd, oldname, newdfd, newname)
    }

    #[inline]
    pub unsafe fn mkdirat(dfd : i32, pathname : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::mkdirat(dfd, pathname, mode)
    }

    #[inline]
    pub unsafe fn linkat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        user::linkat(olddfd, oldname, newdfd, newname, flags)
    }

    #[inline]
    pub unsafe fn unlinkat(dfd : i32, pathname : *const u8, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        user::unlinkat(dfd, pathname, flag)
    }

    #[inline]
    pub unsafe fn symlinkat(oldname : *const u8, newdfd : i32, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::symlinkat(oldname, newdfd, newname)
    }

    #[inline]
    pub unsafe fn readlinkat(dfd : i32, path : *const u8, buf : *mut u8, bufsiz : i32) -> i64 {
        let fs = ProtFs::prot();
        user::readlinkat(dfd, path, buf, bufsiz)
    }

    #[inline]
    pub unsafe fn fchmod(fd : u32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::fchmod(fd, mode)
    }

    #[inline]
    pub unsafe fn fchmodat(dfd : i32, filename : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::fchmodat(dfd, filename, mode)
    }

    #[inline]
    pub unsafe fn fchown(fd : u32, user : u32, group : u32) -> i64 {
        let fs = ProtFs::prot();
        user::fchown(fd, user, group)
    }

    #[inline]
    pub unsafe fn fchownat(dfd : i32, filename : *const u8, user : u32, group : u32, flag : i32) -> i64 {
        let fs = ProtFs::prot();
        user::fchownat(dfd, filename, user, group, flag)
    }

    #[inline]
    pub unsafe fn faccessat(dfd : i32, filename : *const u8, mode : i32) -> i64 {
        let fs = ProtFs::prot();
        user::faccessat(dfd, filename, mode)
    }

    #[inline]
    pub unsafe fn dup3(oldfd : u32, newfd : u32, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        user::dup3(oldfd, newfd, flags)
    }

    #[inline]
    pub unsafe fn utimensat(dfd : i32, filename : *const u8, utimes : * const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        user::utimensat(dfd, filename, utimes, flags)
    }

    #[inline]
    pub unsafe fn copy_file_range(fd_in : i32, off_in : *mut i64, fd_out : i32, off_out : *mut i64, len : usize, flags : u32) -> i64 {
        let fs = ProtFs::prot();
        user::copy_file_range(fd_in, off_in, fd_out, off_out, len, flags)
    }

    #[inline]
    pub unsafe fn statfs(path : *const u8, buf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::statfs(path, buf)
    }

    #[inline]
    pub unsafe fn fstatfs(fd : u32, buf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::fstatfs(fd, buf)
    }

    #[inline]
    pub unsafe fn sync() -> i64 {
        let fs = ProtFs::prot();
        user::sync()
    }

    #[inline]
    pub unsafe fn mount(dev_name : *mut u8, dir_name : *mut u8, ty : *mut u8, flags : u64, data : *mut u8) -> i64 {
        let fs = ProtFs::prot();
        user::mount(dev_name, dir_name, ty, flags, data)
    }

    #[inline]
    pub unsafe fn umount2(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        user::umount2(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline]
    pub unsafe fn brk(brk : u64) -> i64 {
        let fs = ProtFs::prot();
        user::brk(brk)
    }

    #[inline]
    pub unsafe fn mmap(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        user::mmap(arg1, arg2, arg3, arg4, arg5, arg6)
    }

    #[inline]
    pub unsafe fn mprotect(start : u64, len : usize, prot : u64) -> i64 {
        let fs = ProtFs::prot();
        user::mprotect(start, len, prot)
    }

    #[inline]
    pub unsafe fn munmap(addr : u64, len : usize) -> i64 {
        let fs = ProtFs::prot();
        user::munmap(addr, len)
    }

    #[inline]
    pub unsafe fn madvise(start : u64, len : usize, behavior : i32) -> i64 {
        let fs = ProtFs::prot();
        user::madvise(start, len, behavior)
    }

    #[inline]
    pub unsafe fn rt_sigaction(arg1 : i32, arg2 : * const u8, arg3 : * const u8, size_t : usize) -> i64 {
        let fs = ProtFs::prot();
        user::rt_sigaction(arg1, arg2, arg3, size_t)
    }

    #[inline]
    pub unsafe fn rt_sigprocmask(how : i32, set : u64, oset : u64, sigsetsize : usize) -> i64 {
        let fs = ProtFs::prot();
        user::rt_sigprocmask(how, set, oset, sigsetsize)
    }

    #[inline]
    pub unsafe fn sigaltstack(uss : * const u8, uoss : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::sigaltstack(uss, uoss)
    }

    #[inline]
    pub unsafe fn clone(arg1 : * const u8, arg2 : * const u8, arg3 : *mut i32, arg4 : *mut i32, arg5 : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::clone(arg1, arg2, arg3, arg4, arg5)
    }

    #[inline]
    pub unsafe fn execve(filename : *const u8, argv : *const *const u8, envp : *const *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::execve(filename, argv, envp)
    }

    #[inline]
    pub unsafe fn exit(error_code : i32) -> i64 {
        let fs = ProtFs::prot();
        user::exit(error_code)
    }

    #[inline]
    pub unsafe fn exit_group(error_code : i32) -> i64 {
        let fs = ProtFs::prot();
        user::exit_group(error_code)
    }

    #[inline]
    pub unsafe fn wait4(pid : i32, stat_addr : *mut i32, options : i32, ru : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::wait4(pid, stat_addr, options, ru)
    }

    #[inline]
    pub unsafe fn set_tid_address(tidptr : *mut i32) -> i64 {
        let fs = ProtFs::prot();
        user::set_tid_address(tidptr)
    }

    #[inline]
    pub unsafe fn futex(uaddr : *mut u32, op : i32, val : u32, utime : * const u8, uaddr2 : *mut u32, val3 : u32) -> i64 {
        let fs = ProtFs::prot();
        user::futex(uaddr, op, val, utime, uaddr2, val3)
    }

    #[inline]
    pub unsafe fn tkill(pid : i32, sig : i32) -> i64 {
        let fs = ProtFs::prot();
        user::tkill(pid, sig)
    }

    #[inline]
    pub unsafe fn setitimer(which : i32, value : * const u8, ovalue : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::setitimer(which, value, ovalue)
    }

    #[inline]
    pub unsafe fn clock_gettime(which_clock : i32, tp : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::clock_gettime(which_clock, tp)
    }

    #[inline]
    pub unsafe fn getpid() -> i64 {
        let fs = ProtFs::prot();
        user::getpid()
    }

    #[inline]
    pub unsafe fn gettid() -> i64 {
        let fs = ProtFs::prot();
        user::gettid()
    }

    #[inline]
    pub unsafe fn uname(arg1 : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::uname(arg1)
    }

    #[inline]
    pub unsafe fn umask(mask : i32) -> i64 {
        let fs = ProtFs::prot();
        user::umask(mask)
    }

    #[inline]
    pub unsafe fn getuid() -> i64 {
        let fs = ProtFs::prot();
        user::getuid()
    }

    #[inline]
    pub unsafe fn getgid() -> i64 {
        let fs = ProtFs::prot();
        user::getgid()
    }

    #[inline]
    pub unsafe fn setuid(uid : u32) -> i64 {
        let fs = ProtFs::prot();
        user::setuid(uid)
    }

    #[inline]
    pub unsafe fn geteuid() -> i64 {
        let fs = ProtFs::prot();
        user::geteuid()
    }

    #[inline]
    pub unsafe fn getegid() -> i64 {
        let fs = ProtFs::prot();
        user::getegid()
    }

    #[inline]
    pub unsafe fn setpgid(pid : i32, pgid : i32) -> i64 {
        let fs = ProtFs::prot();
        user::setpgid(pid, pgid)
    }

    #[inline]
    pub unsafe fn getppid() -> i64 {
        let fs = ProtFs::prot();
        user::getppid()
    }

    #[inline]
    pub unsafe fn setsid() -> i64 {
        let fs = ProtFs::prot();
        user::setsid()
    }

    #[inline]
    pub unsafe fn getpgid(pid : i32) -> i64 {
        let fs = ProtFs::prot();
        user::getpgid(pid)
    }

    #[inline]
    pub unsafe fn getgroups(gidsetsize : i32, grouplist : *mut u32) -> i64 {
        let fs = ProtFs::prot();
        user::getgroups(gidsetsize, grouplist)
    }

    #[inline]
    pub unsafe fn setgroups(gidsetsize : i32, grouplist : *mut u32) -> i64 {
        let fs = ProtFs::prot();
        user::setgroups(gidsetsize, grouplist)
    }

    #[inline]
    pub unsafe fn prctl(option : i32, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64) -> i64 {
        let fs = ProtFs::prot();
        user::prctl(option, arg2, arg3, arg4, arg5)
    }

    #[inline]
    pub unsafe fn membarrier(cmd : i32, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        user::membarrier(cmd, flags)
    }

    #[inline]
    pub unsafe fn rt_sigqueueinfo(pid : i32, sig : i32, uinfo : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::rt_sigqueueinfo(pid, sig, uinfo)
    }

    #[inline]
    pub unsafe fn finit_module(fd : i32, uargs : *const u8, flags : i32) -> i64 {
        let fs = ProtFs::prot();
        user::finit_module(fd, uargs, flags)
    }

    #[inline]
    pub unsafe fn open(filename : *const u8, flags : i32, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::open(filename, flags, mode)
    }

    #[inline]
    pub unsafe fn stat(filename : *const u8, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::stat(filename, statbuf)
    }

    #[inline]
    pub unsafe fn lstat(filename : *const u8, statbuf : * const u8) -> i64 {
        let fs = ProtFs::prot();
        user::lstat(filename, statbuf)
    }

    #[inline]
    pub unsafe fn access(filename : *const u8, mode : i32) -> i64 {
        let fs = ProtFs::prot();
        user::access(filename, mode)
    }

    #[inline]
    pub unsafe fn dup2(oldfd : u32, newfd : u32) -> i64 {
        let fs = ProtFs::prot();
        user::dup2(oldfd, newfd)
    }

    #[inline]
    pub unsafe fn fork() -> i64 {
        let fs = ProtFs::prot();
        user::fork()
    }

    #[inline]
    pub unsafe fn vfork() -> i64 {
        let fs = ProtFs::prot();
        user::vfork()
    }

    #[inline]
    pub unsafe fn rename(oldname : *const u8, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::rename(oldname, newname)
    }

    #[inline]
    pub unsafe fn mkdir(pathname : *const u8, mode : u16) -> i64 {
        let fs = ProtFs::prot();
        user::mkdir(pathname, mode)
    }

    #[inline]
    pub unsafe fn rmdir(pathname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::rmdir(pathname)
    }

    #[inline]
    pub unsafe fn link(oldname : *const u8, newname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::link(oldname, newname)
    }

    #[inline]
    pub unsafe fn unlink(pathname : *const u8) -> i64 {
        let fs = ProtFs::prot();
        user::unlink(pathname)
    }

    #[inline]
    pub unsafe fn readlink(path : *const u8, buf : *mut u8, bufsiz : i32) -> i64 {
        let fs = ProtFs::prot();
        user::readlink(path, buf, bufsiz)
    }

    #[inline]
    pub unsafe fn arch_prctl(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64 {
        let fs = ProtFs::prot();
        user::arch_prctl(arg1, arg2, arg3, arg4, arg5, arg6)
    }

}

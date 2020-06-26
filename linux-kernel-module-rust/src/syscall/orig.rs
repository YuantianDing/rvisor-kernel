use {
    super::*,
};

pub mod user {
    extern "C" {
        fn orig_read(fd : u32, buf : *mut u8, count : usize) -> i64;

        fn orig_write(fd : u32, buf : *const u8, count : usize) -> i64;

        fn orig_openat(dfd : i32, filename : *const u8, flags : i32, mode : u16) -> i64;

        fn orig_close(fd : u32) -> i64;

        fn orig_fstat(fd : u32, statbuf : * const u8) -> i64;

        fn orig_newfstatat(dfd : i32, filename : *const u8, statbuf : * const u8, flag : i32) -> i64;

        fn orig_lseek(fd : u32, offset : i64, whence : u32) -> i64;

        fn orig_ioctl(fd : u32, cmd : u32, arg : u64) -> i64;

        fn orig_pread64(fd : u32, buf : *mut u8, count : usize, pos : i64) -> i64;

        fn orig_pwrite64(fd : u32, buf : *const u8, count : usize, pos : i64) -> i64;

        fn orig_readv(fd : u64, vec : * const u8, vlen : u64) -> i64;

        fn orig_writev(fd : u64, vec : * const u8, vlen : u64) -> i64;

        fn orig_sendfile(out_fd : i32, in_fd : i32, offset : *mut i64, count : usize) -> i64;

        fn orig_fcntl(fd : u32, cmd : u32, arg : u64) -> i64;

        fn orig_flock(fd : u32, cmd : u32) -> i64;

        fn orig_fsync(fd : u32) -> i64;

        fn orig_fdatasync(fd : u32) -> i64;

        fn orig_truncate(path : *const u8, length : i64) -> i64;

        fn orig_ftruncate(fd : u32, length : u64) -> i64;

        fn orig_getdents64(fd : u32, dirent : * const u8, count : u32) -> i64;

        fn orig_getcwd(buf : *mut u8, size : u64) -> i64;

        fn orig_chdir(filename : *const u8) -> i64;

        fn orig_renameat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8) -> i64;

        fn orig_mkdirat(dfd : i32, pathname : *const u8, mode : u16) -> i64;

        fn orig_linkat(olddfd : i32, oldname : *const u8, newdfd : i32, newname : *const u8, flags : i32) -> i64;

        fn orig_unlinkat(dfd : i32, pathname : *const u8, flag : i32) -> i64;

        fn orig_symlinkat(oldname : *const u8, newdfd : i32, newname : *const u8) -> i64;

        fn orig_readlinkat(dfd : i32, path : *const u8, buf : *mut u8, bufsiz : i32) -> i64;

        fn orig_fchmod(fd : u32, mode : u16) -> i64;

        fn orig_fchmodat(dfd : i32, filename : *const u8, mode : u16) -> i64;

        fn orig_fchown(fd : u32, user : u32, group : u32) -> i64;

        fn orig_fchownat(dfd : i32, filename : *const u8, user : u32, group : u32, flag : i32) -> i64;

        fn orig_faccessat(dfd : i32, filename : *const u8, mode : i32) -> i64;

        fn orig_dup3(oldfd : u32, newfd : u32, flags : i32) -> i64;

        fn orig_utimensat(dfd : i32, filename : *const u8, utimes : * const u8, flags : i32) -> i64;

        fn orig_copy_file_range(fd_in : i32, off_in : *mut i64, fd_out : i32, off_out : *mut i64, len : usize, flags : u32) -> i64;

        fn orig_statfs(path : *const u8, buf : * const u8) -> i64;

        fn orig_fstatfs(fd : u32, buf : * const u8) -> i64;

        fn orig_sync() -> i64;

        fn orig_mount(dev_name : *mut u8, dir_name : *mut u8, ty : *mut u8, flags : u64, data : *const u8) -> i64;

        fn orig_umount2(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

        fn orig_brk(brk : u64) -> i64;

        fn orig_mmap(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

        fn orig_mprotect(start : u64, len : usize, prot : u64) -> i64;

        fn orig_munmap(addr : u64, len : usize) -> i64;

        fn orig_madvise(start : u64, len : usize, behavior : i32) -> i64;

        fn orig_rt_sigaction(arg1 : i32, arg2 : * const u8, arg3 : * const u8, size_t : usize) -> i64;

        fn orig_rt_sigprocmask(how : i32, set : u64, oset : u64, sigsetsize : usize) -> i64;

        fn orig_sigaltstack(uss : * const u8, uoss : * const u8) -> i64;

        fn orig_clone(arg1 : * const u8, arg2 : * const u8, arg3 : *mut i32, arg4 : *mut i32, arg5 : * const u8) -> i64;

        fn orig_execve(filename : *const u8, argv : *const *const u8, envp : *const *const u8) -> i64;

        fn orig_exit(error_code : i32) -> i64;

        fn orig_exit_group(error_code : i32) -> i64;

        fn orig_wait4(pid : i32, stat_addr : *mut i32, options : i32, ru : * const u8) -> i64;

        fn orig_set_tid_address(tidptr : *mut i32) -> i64;

        fn orig_futex(uaddr : *mut u32, op : i32, val : u32, utime : * const u8, uaddr2 : *mut u32, val3 : u32) -> i64;

        fn orig_tkill(pid : i32, sig : i32) -> i64;

        fn orig_setitimer(which : i32, value : * const u8, ovalue : * const u8) -> i64;

        fn orig_clock_gettime(which_clock : i32, tp : *const u8) -> i64;

        fn orig_getpid() -> i64;

        fn orig_gettid() -> i64;

        fn orig_uname(arg1 : * const u8) -> i64;

        fn orig_umask(mask : i32) -> i64;

        fn orig_getuid() -> i64;

        fn orig_getgid() -> i64;

        fn orig_setuid(uid : u32) -> i64;

        fn orig_geteuid() -> i64;

        fn orig_getegid() -> i64;

        fn orig_setpgid(pid : i32, pgid : i32) -> i64;

        fn orig_getppid() -> i64;

        fn orig_setsid() -> i64;

        fn orig_getpgid(pid : i32) -> i64;

        fn orig_getgroups(gidsetsize : i32, grouplist : *mut u32) -> i64;

        fn orig_setgroups(gidsetsize : i32, grouplist : *mut u32) -> i64;

        fn orig_prctl(option : i32, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64) -> i64;

        fn orig_membarrier(cmd : i32, flags : i32) -> i64;

        fn orig_rt_sigqueueinfo(pid : i32, sig : i32, uinfo : * const u8) -> i64;

        fn orig_finit_module(fd : i32, uargs : *const u8, flags : i32) -> i64;

        fn orig_open(filename : *const u8, flags : i32, mode : u16) -> i64;

        fn orig_stat(filename : *const u8, statbuf : * const u8) -> i64;

        fn orig_lstat(filename : *const u8, statbuf : * const u8) -> i64;

        fn orig_access(filename : *const u8, mode : i32) -> i64;

        fn orig_dup2(oldfd : u32, newfd : u32) -> i64;

        fn orig_fork() -> i64;

        fn orig_vfork() -> i64;

        fn orig_rename(oldname : *const u8, newname : *const u8) -> i64;

        fn orig_mkdir(pathname : *const u8, mode : u16) -> i64;

        fn orig_rmdir(pathname : *const u8) -> i64;

        fn orig_link(oldname : *const u8, newname : *const u8) -> i64;

        fn orig_unlink(pathname : *const u8) -> i64;

        fn orig_readlink(path : *const u8, buf : *mut u8, bufsiz : i32) -> i64;

        fn orig_arch_prctl(arg1 : u64, arg2 : u64, arg3 : u64, arg4 : u64, arg5 : u64, arg6 : u64) -> i64;

    }
        pub use orig_read as read;

        pub use orig_write as write;

        pub use orig_openat as openat;

        pub use orig_close as close;

        pub use orig_fstat as fstat;

        pub use orig_newfstatat as newfstatat;

        pub use orig_lseek as lseek;

        pub use orig_ioctl as ioctl;

        pub use orig_pread64 as pread64;

        pub use orig_pwrite64 as pwrite64;

        pub use orig_readv as readv;

        pub use orig_writev as writev;

        pub use orig_sendfile as sendfile;

        pub use orig_fcntl as fcntl;

        pub use orig_flock as flock;

        pub use orig_fsync as fsync;

        pub use orig_fdatasync as fdatasync;

        pub use orig_truncate as truncate;

        pub use orig_ftruncate as ftruncate;

        pub use orig_getdents64 as getdents64;

        pub use orig_getcwd as getcwd;

        pub use orig_chdir as chdir;

        pub use orig_renameat as renameat;

        pub use orig_mkdirat as mkdirat;

        pub use orig_linkat as linkat;

        pub use orig_unlinkat as unlinkat;

        pub use orig_symlinkat as symlinkat;

        pub use orig_readlinkat as readlinkat;

        pub use orig_fchmod as fchmod;

        pub use orig_fchmodat as fchmodat;

        pub use orig_fchown as fchown;

        pub use orig_fchownat as fchownat;

        pub use orig_faccessat as faccessat;

        pub use orig_dup3 as dup3;

        pub use orig_utimensat as utimensat;

        pub use orig_copy_file_range as copy_file_range;

        pub use orig_statfs as statfs;

        pub use orig_fstatfs as fstatfs;

        pub use orig_sync as sync;

        pub use orig_mount as mount;

        pub use orig_umount2 as umount2;

        pub use orig_brk as brk;

        pub use orig_mmap as mmap;

        pub use orig_mprotect as mprotect;

        pub use orig_munmap as munmap;

        pub use orig_madvise as madvise;

        pub use orig_rt_sigaction as rt_sigaction;

        pub use orig_rt_sigprocmask as rt_sigprocmask;

        pub use orig_sigaltstack as sigaltstack;

        pub use orig_clone as clone;

        pub use orig_execve as execve;

        pub use orig_exit as exit;

        pub use orig_exit_group as exit_group;

        pub use orig_wait4 as wait4;

        pub use orig_set_tid_address as set_tid_address;

        pub use orig_futex as futex;

        pub use orig_tkill as tkill;

        pub use orig_setitimer as setitimer;

        pub use orig_clock_gettime as clock_gettime;

        pub use orig_getpid as getpid;

        pub use orig_gettid as gettid;

        pub use orig_uname as uname;

        pub use orig_umask as umask;

        pub use orig_getuid as getuid;

        pub use orig_getgid as getgid;

        pub use orig_setuid as setuid;

        pub use orig_geteuid as geteuid;

        pub use orig_getegid as getegid;

        pub use orig_setpgid as setpgid;

        pub use orig_getppid as getppid;

        pub use orig_setsid as setsid;

        pub use orig_getpgid as getpgid;

        pub use orig_getgroups as getgroups;

        pub use orig_setgroups as setgroups;

        pub use orig_prctl as prctl;

        pub use orig_membarrier as membarrier;

        pub use orig_rt_sigqueueinfo as rt_sigqueueinfo;

        pub use orig_finit_module as finit_module;

        pub use orig_open as open;

        pub use orig_stat as stat;

        pub use orig_lstat as lstat;

        pub use orig_access as access;

        pub use orig_dup2 as dup2;

        pub use orig_fork as fork;

        pub use orig_vfork as vfork;

        pub use orig_rename as rename;

        pub use orig_mkdir as mkdir;

        pub use orig_rmdir as rmdir;

        pub use orig_link as link;

        pub use orig_unlink as unlink;

        pub use orig_readlink as readlink;

        pub use orig_arch_prctl as arch_prctl;

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
    pub unsafe fn mount(dev_name : *mut u8, dir_name : *mut u8, ty : *mut u8, flags : u64, data : *const u8) -> i64 {
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
    pub unsafe fn clock_gettime(which_clock : i32, tp : *const u8) -> i64 {
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

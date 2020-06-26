#[cfg(feature="bindgen")]
use bindgen;

use cc;
use shlex;

use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use glob::glob;

const INCLUDED_TYPES: &[&str] = &[
    "file_system_type",
    "mode_t",
    "umode_t", 
    "ctl_table",
    "mm_segment_t",
    "pid_t",
    "dentry",
    "path",
    "file",
];
const INCLUDED_FUNCTIONS: &[&str] = &[
    "cdev_add",
    "cdev_init",
    "cdev_del",
    "register_filesystem",
    "unregister_filesystem",
    "krealloc",
    "kfree",
    "mount_nodev",
    "kill_litter_super",
    "register_sysctl",
    "unregister_sysctl_table",
    "access_ok",
    "_copy_to_user",
    "_copy_from_user",
    "alloc_chrdev_region",
    "unregister_chrdev_region",
    "wait_for_random_bytes",
    "get_random_bytes",
    "rng_is_initialized",
    "strncpy_from_user",
];
const INCLUDED_VARS: &[&str] = &[
    "EINVAL",
    "ENOMEM",
    "ESPIPE",
    "EFAULT",
    "EAGAIN",
    "__this_module",
    "FS_REQUIRES_DEV",
    "FS_BINARY_MOUNTDATA",
    "FS_HAS_SUBTYPE",
    "FS_USERNS_MOUNT",
    "FS_RENAME_DOES_D_MOVE",
    "BINDINGS_GFP_KERNEL",
    "KERN_INFO",
    "VERIFY_WRITE",
    "LINUX_VERSION_CODE",
    "SEEK_SET",
    "SEEK_CUR",
    "SEEK_END",
    "__NR_read",
    "__NR_write",
    "__NR_open",
    "__NR_close",
    "__NR_stat",
    "__NR_fstat",
    "__NR_lstat",
    "__NR_poll",
    "__NR_lseek",
    "__NR_mmap",
    "__NR_mprotect",
    "__NR_munmap",
    "__NR_brk",
    "__NR_rt_sigaction",
    "__NR_rt_sigprocmask",
    "__NR_rt_sigreturn",
    "__NR_ioctl",
    "__NR_pread64",
    "__NR_pwrite64",
    "__NR_readv",
    "__NR_writev",
    "__NR_access",
    "__NR_pipe",
    "__NR_select",
    "__NR_sched_yield",
    "__NR_mremap",
    "__NR_msync",
    "__NR_mincore",
    "__NR_madvise",
    "__NR_shmget",
    "__NR_shmat",
    "__NR_shmctl",
    "__NR_dup",
    "__NR_dup2",
    "__NR_pause",
    "__NR_nanosleep",
    "__NR_getitimer",
    "__NR_alarm",
    "__NR_setitimer",
    "__NR_getpid",
    "__NR_sendfile",
    "__NR_socket",
    "__NR_connect",
    "__NR_accept",
    "__NR_sendto",
    "__NR_recvfrom",
    "__NR_sendmsg",
    "__NR_recvmsg",
    "__NR_shutdown",
    "__NR_bind",
    "__NR_listen",
    "__NR_getsockname",
    "__NR_getpeername",
    "__NR_socketpair",
    "__NR_setsockopt",
    "__NR_getsockopt",
    "__NR_clone",
    "__NR_fork",
    "__NR_vfork",
    "__NR_execve",
    "__NR_exit",
    "__NR_wait4",
    "__NR_kill",
    "__NR_uname",
    "__NR_semget",
    "__NR_semop",
    "__NR_semctl",
    "__NR_shmdt",
    "__NR_msgget",
    "__NR_msgsnd",
    "__NR_msgrcv",
    "__NR_msgctl",
    "__NR_fcntl",
    "__NR_flock",
    "__NR_fsync",
    "__NR_fdatasync",
    "__NR_truncate",
    "__NR_ftruncate",
    "__NR_getdents",
    "__NR_getcwd",
    "__NR_chdir",
    "__NR_fchdir",
    "__NR_rename",
    "__NR_mkdir",
    "__NR_rmdir",
    "__NR_creat",
    "__NR_link",
    "__NR_unlink",
    "__NR_symlink",
    "__NR_readlink",
    "__NR_chmod",
    "__NR_fchmod",
    "__NR_chown",
    "__NR_fchown",
    "__NR_lchown",
    "__NR_umask",
    "__NR_gettimeofday",
    "__NR_getrlimit",
    "__NR_getrusage",
    "__NR_sysinfo",
    "__NR_times",
    "__NR_ptrace",
    "__NR_getuid",
    "__NR_syslog",
    "__NR_getgid",
    "__NR_setuid",
    "__NR_setgid",
    "__NR_geteuid",
    "__NR_getegid",
    "__NR_setpgid",
    "__NR_getppid",
    "__NR_getpgrp",
    "__NR_setsid",
    "__NR_setreuid",
    "__NR_setregid",
    "__NR_getgroups",
    "__NR_setgroups",
    "__NR_setresuid",
    "__NR_getresuid",
    "__NR_setresgid",
    "__NR_getresgid",
    "__NR_getpgid",
    "__NR_setfsuid",
    "__NR_setfsgid",
    "__NR_getsid",
    "__NR_capget",
    "__NR_capset",
    "__NR_rt_sigpending",
    "__NR_rt_sigtimedwait",
    "__NR_rt_sigqueueinfo",
    "__NR_rt_sigsuspend",
    "__NR_sigaltstack",
    "__NR_utime",
    "__NR_mknod",
    "__NR_uselib",
    "__NR_personality",
    "__NR_ustat",
    "__NR_statfs",
    "__NR_fstatfs",
    "__NR_sysfs",
    "__NR_getpriority",
    "__NR_setpriority",
    "__NR_sched_setparam",
    "__NR_sched_getparam",
    "__NR_sched_setscheduler",
    "__NR_sched_getscheduler",
    "__NR_sched_get_priority_max",
    "__NR_sched_get_priority_min",
    "__NR_sched_rr_get_interval",
    "__NR_mlock",
    "__NR_munlock",
    "__NR_mlockall",
    "__NR_munlockall",
    "__NR_vhangup",
    "__NR_modify_ldt",
    "__NR_pivot_root",
    "__NR__sysctl",
    "__NR_prctl",
    "__NR_arch_prctl",
    "__NR_adjtimex",
    "__NR_setrlimit",
    "__NR_chroot",
    "__NR_sync",
    "__NR_acct",
    "__NR_settimeofday",
    "__NR_mount",
    "__NR_umount2",
    "__NR_swapon",
    "__NR_swapoff",
    "__NR_reboot",
    "__NR_sethostname",
    "__NR_setdomainname",
    "__NR_iopl",
    "__NR_ioperm",
    "__NR_create_module",
    "__NR_init_module",
    "__NR_delete_module",
    "__NR_get_kernel_syms",
    "__NR_query_module",
    "__NR_quotactl",
    "__NR_nfsservctl",
    "__NR_getpmsg",
    "__NR_putpmsg",
    "__NR_afs_syscall",
    "__NR_tuxcall",
    "__NR_security",
    "__NR_gettid",
    "__NR_readahead",
    "__NR_setxattr",
    "__NR_lsetxattr",
    "__NR_fsetxattr",
    "__NR_getxattr",
    "__NR_lgetxattr",
    "__NR_fgetxattr",
    "__NR_listxattr",
    "__NR_llistxattr",
    "__NR_flistxattr",
    "__NR_removexattr",
    "__NR_lremovexattr",
    "__NR_fremovexattr",
    "__NR_tkill",
    "__NR_time",
    "__NR_futex",
    "__NR_sched_setaffinity",
    "__NR_sched_getaffinity",
    "__NR_set_thread_area",
    "__NR_io_setup",
    "__NR_io_destroy",
    "__NR_io_getevents",
    "__NR_io_submit",
    "__NR_io_cancel",
    "__NR_get_thread_area",
    "__NR_lookup_dcookie",
    "__NR_epoll_create",
    "__NR_epoll_ctl_old",
    "__NR_epoll_wait_old",
    "__NR_remap_file_pages",
    "__NR_getdents64",
    "__NR_set_tid_address",
    "__NR_restart_syscall",
    "__NR_semtimedop",
    "__NR_fadvise64",
    "__NR_timer_create",
    "__NR_timer_settime",
    "__NR_timer_gettime",
    "__NR_timer_getoverrun",
    "__NR_timer_delete",
    "__NR_clock_settime",
    "__NR_clock_gettime",
    "__NR_clock_getres",
    "__NR_clock_nanosleep",
    "__NR_exit_group",
    "__NR_epoll_wait",
    "__NR_epoll_ctl",
    "__NR_tgkill",
    "__NR_utimes",
    "__NR_vserver",
    "__NR_mbind",
    "__NR_set_mempolicy",
    "__NR_get_mempolicy",
    "__NR_mq_open",
    "__NR_mq_unlink",
    "__NR_mq_timedsend",
    "__NR_mq_timedreceive",
    "__NR_mq_notify",
    "__NR_mq_getsetattr",
    "__NR_kexec_load",
    "__NR_waitid",
    "__NR_add_key",
    "__NR_request_key",
    "__NR_keyctl",
    "__NR_ioprio_set",
    "__NR_ioprio_get",
    "__NR_inotify_init",
    "__NR_inotify_add_watch",
    "__NR_inotify_rm_watch",
    "__NR_migrate_pages",
    "__NR_openat",
    "__NR_mkdirat",
    "__NR_mknodat",
    "__NR_fchownat",
    "__NR_futimesat",
    "__NR_newfstatat",
    "__NR_unlinkat",
    "__NR_renameat",
    "__NR_linkat",
    "__NR_symlinkat",
    "__NR_readlinkat",
    "__NR_fchmodat",
    "__NR_faccessat",
    "__NR_pselect6",
    "__NR_ppoll",
    "__NR_unshare",
    "__NR_set_robust_list",
    "__NR_get_robust_list",
    "__NR_splice",
    "__NR_tee",
    "__NR_sync_file_range",
    "__NR_vmsplice",
    "__NR_move_pages",
    "__NR_utimensat",
    "__NR_epoll_pwait",
    "__NR_signalfd",
    "__NR_timerfd_create",
    "__NR_eventfd",
    "__NR_fallocate",
    "__NR_timerfd_settime",
    "__NR_timerfd_gettime",
    "__NR_accept4",
    "__NR_signalfd4",
    "__NR_eventfd2",
    "__NR_epoll_create1",
    "__NR_dup3",
    "__NR_pipe2",
    "__NR_inotify_init1",
    "__NR_preadv",
    "__NR_pwritev",
    "__NR_rt_tgsigqueueinfo",
    "__NR_perf_event_open",
    "__NR_recvmmsg",
    "__NR_fanotify_init",
    "__NR_fanotify_mark",
    "__NR_prlimit64",
    "__NR_name_to_handle_at",
    "__NR_open_by_handle_at",
    "__NR_clock_adjtime",
    "__NR_syncfs",
    "__NR_sendmmsg",
    "__NR_setns",
    "__NR_getcpu",
    "__NR_process_vm_readv",
    "__NR_process_vm_writev",
    "__NR_kcmp",
    "__NR_finit_module",
    "__NR_sched_setattr",
    "__NR_sched_getattr",
    "__NR_renameat2",
    "__NR_seccomp",
    "__NR_getrandom",
    "__NR_memfd_create",
    "__NR_kexec_file_load",
    "__NR_bpf",
    "__NR_execveat",
    "__NR_userfaultfd",
    "__NR_membarrier",
    "__NR_mlock2",
    "__NR_copy_file_range",
    "__NR_preadv2",
    "__NR_pwritev2",
    "__NR_pkey_mprotect",
    "__NR_pkey_alloc",
    "__NR_pkey_free",
    "__NR_statx",
];
const OPAQUE_TYPES: &[&str] = &[
    // These need to be opaque because they're both packed and aligned, which rustc
    // doesn't support yet. See https://github.com/rust-lang/rust/issues/59154
    // and https://github.com/rust-lang/rust-bindgen/issues/1538
    "desc_struct",
    "xregs_state",
];

fn kernel_version_code(major: u8, minor: u8, patch: u8) -> u64 {
    ((major as u64) << 16) | ((minor as u64) << 8) | (patch as u64)
}

fn handle_kernel_version_cfg(bindings_path: &PathBuf) {
    let f = BufReader::new(fs::File::open(bindings_path).unwrap());
    let mut version = None;
    for line in f.lines() {
        let line = line.unwrap();
        if let Some(type_and_value) = line.split("pub const LINUX_VERSION_CODE").nth(1) {
            if let Some(value) = type_and_value.split("=").nth(1) {
                let raw_version = value.split(";").next().unwrap();
                version = Some(raw_version.trim().parse::<u64>().unwrap());
                break;
            }
        }
    }
    let version = version.expect("Couldn't find kernel version");
    if version >= kernel_version_code(4, 15, 0) {
        println!("cargo:rustc-cfg=kernel_4_15_0_or_greater")
    }
    if version >= kernel_version_code(4, 19, 0) {
        println!("cargo:rustc-cfg=kernel_4_19_0_or_greater")
    }
    if version >= kernel_version_code(4, 20, 0) {
        println!("cargo:rustc-cfg=kernel_4_20_0_or_greater")
    }
    if version >= kernel_version_code(5, 1, 0) {
        println!("cargo:rustc-cfg=kernel_5_1_0_or_greater")
    }
}

fn handle_kernel_symbols_cfg(symvers_path: &PathBuf) {
    let f = BufReader::new(fs::File::open(symvers_path).unwrap());
    for line in f.lines() {
        let line = line.unwrap();
        if let Some(symbol) = line.split_ascii_whitespace().nth(1) {
            if symbol == "setfl" {
                println!("cargo:rustc-cfg=kernel_aufs_setfl");
                break;
            }
        }
    }
}

fn add_env_if_present(cmd: &mut Command, var: &str) {
    if let Ok(val) = env::var(var) {
        cmd.env(var, val);
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=KDIR");
    let kdir = env::var("KDIR").unwrap_or(format!(
        "/lib/modules/{}/build",
        std::str::from_utf8(&(Command::new("uname").arg("-r").output().unwrap().stdout))
            .unwrap()
            .trim()
    ));

    println!("cargo:rerun-if-env-changed=CLANG");
    println!("cargo:rerun-if-changed=kernel-cflags-finder/Makefile");
    let mut cmd = Command::new("make");
    cmd.arg("-C")
        .arg("kernel-cflags-finder")
        .arg("-s")
        .env_clear();
    add_env_if_present(&mut cmd, "KDIR");
    add_env_if_present(&mut cmd, "CLANG");
    add_env_if_present(&mut cmd, "PATH");
    let output = cmd.output().unwrap();
    if !output.status.success() {
        eprintln!("kernel-cflags-finder did not succeed");
        eprintln!("stdout: {}", std::str::from_utf8(&output.stdout).unwrap());
        eprintln!("stderr: {}", std::str::from_utf8(&output.stderr).unwrap());
        std::process::exit(1);
    }

    let target = env::var("TARGET").unwrap();

    #[cfg(feature="bindgen")]
    {
        let mut builder = bindgen::Builder::default()
            .use_core()
            .ctypes_prefix("c_types")
            .derive_default(true)
            .rustfmt_bindings(true);

        builder = builder.clang_arg(format!("--target={}", target));
        for arg in shlex::split(std::str::from_utf8(&output.stdout).unwrap()).unwrap() {
            builder = builder.clang_arg(arg.to_string());
        }

        println!("cargo:rerun-if-changed=src/bindings_helper.h");
        builder = builder.header("src/bindings_helper.h");

        for t in INCLUDED_TYPES {
            builder = builder.whitelist_type(t);
        }
        for f in INCLUDED_FUNCTIONS {
            builder = builder.whitelist_function(f);
        }
        for v in INCLUDED_VARS {
            builder = builder.whitelist_var(v);
        }
        for t in OPAQUE_TYPES {
            builder = builder.opaque_type(t);
        }
        let bindings = builder.generate().expect("Unable to generate bindings");

        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
        out_path.join("bindings.rs")
    }

    handle_kernel_version_cfg(&out_path.join("bindings.rs"));
    handle_kernel_symbols_cfg(&PathBuf::from(&kdir).join("Module.symvers"));

    let mut builder = cc::Build::new();
    builder.compiler(env::var("CLANG").unwrap_or("clang".to_string()));
    builder.target(&target);
    builder.warnings(false);
    builder.file("src/helpers.c");

    for entry in glob("src/syscall/*.c").unwrap() {
        if let Ok(path) = entry {
            println!("cargo:rerun-if-env-changed={}", path.to_str().unwrap());
            builder.file(path);
        }
    }
    for arg in shlex::split(std::str::from_utf8(&output.stdout).unwrap()).unwrap() {
        builder.flag(&arg);
    }
    builder.flag("-mfentry");
    builder.compile("helpers");
} 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
 
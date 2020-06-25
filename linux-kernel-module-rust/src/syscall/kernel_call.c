
#include "hook_syscall.h"

#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
// #include <linux/syscalls.h>
#include <linux/delay.h>
#include <linux/sched.h>
#include <linux/version.h>
#include <linux/kallsyms.h>
#include <linux/semaphore.h>
#include <asm/cacheflush.h>
#include <linux/bitops.h>
#include <linux/sizes.h>
#include <linux/byteorder/generic.h>
#include <linux/preempt.h>

// 这里用宏来将系统调用指针封装成函数，对C宏编程，参考：https://blog.csdn.net/gkzscs/article/details/82934054
SYSCALL_EXPORT3(open, const char *, filename, int,flags, unsigned short,mode)

SYSCALL_EXPORT0(getpid)


SYSCALL_EXPORT4(openat, unsigned long, f, const char *, filename, int, flags, unsigned short, mode)

SYSCALL_EXPORT5(clone,
        unsigned long, clone_flags,
        unsigned long, newsp,
		int __user *, parent_tidptr,
		int __user *, child_tidptr,
		unsigned long, tls
)

SYSCALL_EXPORT0(fork)

SYSCALL_EXPORT0(vfork)


SYSCALL_EXPORT3(execve,
		const char __user *, filename,
		const char __user *const __user *, argv,
		const char __user *const __user *, envp)

SYSCALL_EXPORT5(execveat,
		int, fd, const char __user *, filename,
		const char __user *const __user *, argv,
		const char __user *const __user *, envp,
		int, flags)


SYSCALL_EXPORT1(chdir, const char __user *, filename);
SYSCALL_EXPORT2(getcwd, char __user *, buf, unsigned long, size);
  
SYSCALL_EXPORT4(mknodat, int, dfd, const char __user *, filename, umode_t, mode, unsigned int, dev);
SYSCALL_EXPORT3(mknod, const char __user *, filename, umode_t, mode, unsigned, dev);
SYSCALL_EXPORT3(mkdirat, int, dfd, const char __user *, pathname, umode_t, mode);
SYSCALL_EXPORT2(mkdir, const char __user *, pathname, umode_t, mode);
SYSCALL_EXPORT1(rmdir, const char __user *, pathname);
SYSCALL_EXPORT2(stat, const char __user *, filename, struct __old_kernel_stat __user *, statbuf);
SYSCALL_EXPORT2(lstat, const char __user *, filename, struct __old_kernel_stat __user *, statbuf);
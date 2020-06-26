#pragma once

#include <linux/module.h>
#include <linux/init.h>
#include <linux/types.h>
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
#include <linux/syscalls.h>

extern void **syscall_table;

#define SYSCALL_TABLE_LENGTH 300
extern void * saved_syscall_table[SYSCALL_TABLE_LENGTH];


// 系统调用名，返回值，系统调用参数表
#define SYSCALL_C_FUNC(name, ret, args...) \
    typedef ret (*name##_syscall_t)( args ); \
    ret orig_##name( args ) { \
        name##_syscall_t orig_##name##_ptr = saved_syscall_table[__NR_##name]; \
        return orig_##name##_ptr

#define SYSCALL_C_FUNC_END ;}


#define SYSCALL_EXPORT0(name) \
    SYSCALL_C_FUNC(name, long, void) \
        () \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT1(name, T1, V1) \
    SYSCALL_C_FUNC(name, long, T1 V1) \
        (V1) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT2(name, T1, V1, T2, V2) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2) \
        (V1, V2) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT3(name, T1, V1, T2, V2, T3, V3) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3) \
        (V1, V2, V3) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT4(name, T1, V1, T2, V2, T3, V3, T4, V4) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3, T4 V4) \
        (V1, V2, V3, V4) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT5(name, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3, T4 V4, T5 V5) \
        (V1, V2, V3, V4, V5) \
    SYSCALL_C_FUNC_END

#define SYSCALL_EXPORT6(name, T1, V1, T2, V2, T3, V3, T4, V4, T5, V5, T6, V6) \
    SYSCALL_C_FUNC(name, long, T1 V1, T2 V2, T3 V3, T4 V4, T5 V5, T6 V6) \
        (V1, V2, V3, V4, V5, V6) \
    SYSCALL_C_FUNC_END
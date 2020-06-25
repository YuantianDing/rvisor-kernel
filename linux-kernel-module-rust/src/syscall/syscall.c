
#include "hook_syscall.h"
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

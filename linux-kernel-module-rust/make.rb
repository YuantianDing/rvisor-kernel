#!/usr/bin/env ruby

require "ruby_make_script"
require "./syscall.rb"

c_headers = "
#include \"hook_syscall.h\"
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
"

def rust_pair(v)
    v.map { |name, ty|
        "#{name} : #{rust_type(ty)}"
    }
end

def c_pair(v)
    v.map { |name, ty|
        "#{ty}, #{name}"
    }
end

make do
    :orig_syscall .then do
        cd 'src/syscall'
        File.open('orig.rs', 'w') do |f|
            f.puts "use {"
            f.puts "    super::*,"
            f.puts "};"
            f.puts ""
            f.puts "mod lx_orig {"
            f.puts "    extern \"C\" {"
            $syscalls.each do |k, v|
                f.puts "        pub fn #{k}(#{rust_pair(v).join(', ')}) -> i64;"
                f.puts ""
            end
            f.puts "    }"
            f.puts "}"
            f.puts ""
            f.puts ""
            $syscalls.each do |k, v|
                f.puts "#[inline]"
                f.puts "pub fn #{k}(#{rust_pair(v).join(', ')}) -> i64 {"
                f.puts "    let fs = ProtFs::prot();"
                f.puts "    unsafe{ lx_orig::#{k}(#{v.map{ |n,t| n }.join(', ')}) }"
                f.puts "}"
                f.puts ""
            end
        end

        File.open('syscall.c', 'w') do |f|
            f.puts c_headers
            $syscalls.each do |k, v|
                f.puts "SYSCALL_EXPORT#{v.length}(#{k}, #{c_pair(v).join(', ')})"
            end
        end
        cd "../.."
    end
end
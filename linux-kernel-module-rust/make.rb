#!/usr/bin/env ruby

require "ruby_make_script"
require "./syscall.rb"

c_headers = "
#include \"hook_syscall.h\"
"

def rust_pair(v)
    v.map { |name, ty|
        "#{name} : #{rust_type(ty)}"
    }
end

def c_pair(v)
    v.map { |name, ty|
        if ty == "?"
            ty = "unsigned long"
        end
        "#{ty}, #{name}"
    }
end

make do
    :build .from "orig_syscall" do
        r "cargo build"
    end
    :orig_syscall .then do
        cd 'src/syscall'
        File.open('orig.rs', 'w') do |f|
            f.puts "use {"
            f.puts "    super::*,"
            f.puts "};"
            f.puts ""
            f.puts "pub mod user {"
            f.puts "    extern \"C\" {"
            $syscalls.each do |k, v|
                f.puts "        pub fn orig_#{k}(#{rust_pair(v).join(', ')}) -> i64;"
                f.puts ""
            end
            f.puts "    }"
            $syscalls.each do |k, v|
                f.puts "    pub use orig_#{k} as #{k};"
                f.puts ""
            end
            f.puts "}"
            f.puts ""
            f.puts "pub mod kern {"
            f.puts "    use super::*;"
            $syscalls.each do |k, v|
                f.puts "    #[inline]"
                f.puts "    pub unsafe fn #{k}(#{rust_pair(v).join(', ')}) -> i64 {"
                f.puts "        let fs = ProtFs::prot();"
                f.puts "        user::#{k}(#{v.map{ |n,t| n }.join(', ')})"
                f.puts "    }"
                f.puts ""
            end
            f.puts "}"
        end

        File.open('syscall.c', 'w') do |f|
            f.puts c_headers
            $syscalls.each do |k, v|
                f.puts "SYSCALL_EXPORT#{v.length}(#{[k, *c_pair(v)].join(', ')})"
            end
        end
        cd "../.."
    end
end
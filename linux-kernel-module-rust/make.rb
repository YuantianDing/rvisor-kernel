#!/usr/bin/env ruby

require "ruby_make_script"

syscall = [
    :open = [""
    :getpid = [""
    :openat = [""
    :clone = [""
    :fork = [""
    :vfork = [""
    :execve = [""
    :execveat = [""
    :chdir = [""
    :getcwd = [""
    :mknodat = [""
    :mknod = [""
    :mkdirat = [""
    :mkdir = [""
    :rmdir = [""
    :stat = [""
    :lstat = [""
]

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
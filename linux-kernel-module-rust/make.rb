#!/usr/bin/en

require "ruby_make_script"

make do
    :orig_syscall .then do
        r "ruby", "src/syscall"
    end
end
#!/usr/bin/env ruby

require "ruby_make_script"

make do
    :orig_syscall .then do
        r "./src/syscall/make.rb"
    end
end
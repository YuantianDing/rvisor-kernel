#!/usr/bin/env ruby

require "ruby_make_script"

syscall = {
    
}

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
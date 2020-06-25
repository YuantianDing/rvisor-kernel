#!/usr/bin/env ruby

require "ruby_make_script"
require 'rexml/document'

file = File.new("syscall_table.html")
doc = Document.new(file)

p doc == nil

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
#!/usr/bin/env ruby

require "ruby_make_script"
require 'rexml/document'

doc = Document.new(File.new("syscall_table.html"))

puts doc

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
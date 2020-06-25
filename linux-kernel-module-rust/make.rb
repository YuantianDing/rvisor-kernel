#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

file = File.new("syscall_table.html")
doc = Document.new(file)

p doc

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
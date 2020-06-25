#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
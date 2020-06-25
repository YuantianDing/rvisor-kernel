#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'
Line = Struct.new(sysname, )
File.readlines("../zCore/linux-syscall/src/lib.rs") do |line|
    line
end
syscall_list = 

doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }


syscalls = Hash[]
syscallxml = doc.xpath("//tr")
syscallxml.each { |xml|
    xml.xpath("//")
}


make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
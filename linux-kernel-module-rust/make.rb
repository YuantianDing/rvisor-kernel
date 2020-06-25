#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

syscall_list = Hash[]
File.readlines("../zCore/linux-syscall/src/lib.rs") do |line|
    syscall_list[line.match(/^\W*Sys::(\w+)/)[1].downcase] = []
end

doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }


syscalls = Hash[]
syscallxml = doc.xpath("//tr")
syscallxml.each { |xml|
    tds = xml.xpath("//")
    if syscall_list[tds[1]] != nil
        syscall_list[tds[1]] += []
    end
}


make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
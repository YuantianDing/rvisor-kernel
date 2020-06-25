#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

syscall_list = Hash[]
File.readlines("../zCore/linux-syscall/src/lib.rs") do |line|
    syscall_list[line.match(/^\W*Sys::(\w+)/)[1].downcase] = []
end

doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

def parse_type(str)
    str.match(/(\w*)$/)
end

syscalls = Hash[]
syscallxml = doc.xpath("//tr")
syscallxml.each { |xml|
    tds = xml.xpath("//")
    if syscall_list[tds[1]] != nil
        tds[4..-1].each { |td|
            syscall_list[tds[1]] += [parse_type(td.content)]
        }
    end
}


make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
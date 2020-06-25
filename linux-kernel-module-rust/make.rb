#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

syscall_list = ["read", ]

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
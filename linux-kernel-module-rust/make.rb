#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

syscalls = Hash[]
File.readlines("../zCore/linux-syscall/src/lib.rs") { |line|
    p line
    str = line.match(/^\W*Sys::(\w+)/)[1]
    syscalls[str.downcase] = []
}

doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

def parse_type(str)
    name = str.match(/(\w*)$/)[1]
    type = str[0..str.length - name-length]
    [name, type]
end

syscallxml = doc.xpath("//tr")
syscallxml.each { |xml|
    tds = xml.xpath("//td")
    if syscalls[tds[1]] != nil
        tds[4..-1].each { |td|
            syscalls[tds[1]] += [parse_type(td.content)]
        }
    end
}

puts syscalls

make do
    :orig_syscall .then do
        # using dir('src/syscall') do
            
        # end
    end
end
#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

syscalls = Hash[]

File.readlines("../zCore/linux-syscall/src/lib.rs").each do |line|
    m = line.match(/^\W*Sys::(\w+)/)
    if m
        syscalls[m[1].downcase] = []
    end
end


doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

def parse_type(str)
    name = str.match(/(\w*)$/)[1]
    type = str[0..<(str.length - name.length)]
    [name, type]
end

trs = doc.xpath("//tr")

trs.each { |tr|
    tds = tr.children
    p tds[1]
    if syscalls[tds[1].content] != nil
        tds[4..-1].each do |td|
            if td.content != "-"
                syscalls[tds[1].content] += [parse_type(td.content)]
            end
        end
    end
}
p syscalls

make do
    :orig_syscall .then do
        # using dir('src/syscall') do
            
        # end
    end
end
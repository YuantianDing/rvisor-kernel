#!/usr/bin/env ruby

require "ruby_make_script"
require 'nokogiri'

doc = File.open("blossom.xml") { |f| Nokogiri::XML(f) }

p doc

make do
    :orig_syscall .then do
        using dir('src/syscall') do
            
        end
    end
end
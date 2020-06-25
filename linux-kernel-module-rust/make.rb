#!/usr/bin/env ruby

require "ruby_make_script"
require "./syscall.rb"



make do
    :orig_syscall .then do
        using dir('src/syscall') do
            File.open('orig.rs', 'w') do |f|
                f.puts "use {"
                f.puts "    super::*,"
                f.puts "}"
                
                f.puts "mod cshim {"
                f.puts "    extern \"C\" {"
                $syscall.each do |k, v|
                    
                end
                f.puts "    }"
                f.puts "}"
            end
        end
    end
end
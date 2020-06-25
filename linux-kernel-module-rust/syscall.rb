require 'nokogiri'

$syscalls = Hash[]

$rust_type = Hash[
    "unsigned int "=> "u32",
    "char *"=>"*mut u8",
    "size_t "=> "usize",
    "const char *"=> "*const u8",
    "int "=>"i32",
    "umode_t "=>"u16",
    "off_t "=>"i64",
    "?"=>"u64",
    "unsigned long "=>"u64",
    "int"=>"i32",
    "size_t"=>"usize",
    "sigset_t *"=>"u64",
    "loff_t "=>"i64",
    "off_t *"=>"*mut i64",
    "unsigned "=>"u64",
    "int *"=>"*mut i32",
    "const char *const *"=>"*const *const u8",
    "pid_t "=>"i32",
    "long "=>"i64",
    "uid_t "=>"u32",
    "gid_t "=>"u32",
    "gid_t *"=>"*mut u32",
    "const char * "=>"*const u8",
    "void *"=>"*mut u8",
    "u32 *"=>"*mut u32",
    "u32 "=>"u32",
    "clockid_t "=>"i32",
    "loff_t *"=>"*mut i64"
]

File.readlines("../zCore/linux-syscall/src/lib.rs").each do |line|
    m = line.match(/^[\s\t]*Sys::(\w+)/)
    if m
        $syscalls[m[1].downcase] = []
    end
end


doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

$name_num
def parse_type(str)
    name = str.match(/(\w*)$/)[1]
    if ["int", "long", "short", "char", "byte"].include?(name)
        name = ""
    end
    type = str[0..(str.length - name.length - 1)]
    # $rust_type[type] = 'missing'
    $name_num += 1
    if name == ""
        name = "arg#{$name_num}"
    elsif name == "type"
        name = "ty"
    end
    [name, type]
end

trs = doc.xpath("//tr")

trs.each do |tr|
    tds = tr.children
    $name_num = 0
    puts "\"__NR_#{tds[1].content}\","
    if $syscalls[tds[1].content] != nil
        tds[4..-1].each do |td|
            if td.content != "-"
                $syscalls[tds[1].content] += [parse_type(td.content)]
            end
        end
    end
end

def rust_type(str)
    if $rust_type[str]
        return $rust_type[str]
    else
        return "* const u8"
    end
end


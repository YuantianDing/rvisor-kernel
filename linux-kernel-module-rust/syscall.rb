require 'nokogiri'

$syscalls = Hash[]

$rust_type = Hash[
    "unsigned int "=> "u32",
    "char *"=>"*mut u8",
    "size_t "=> "usize",
    "const char *"=> "*const u8",
    "int "=>"i32",
    "umode_t "=>"u16",
    "off_t "=>"u64",
    "?"=>"u64",
    "unsigned long "=>"u64",
    "int"=>"i32",
    "size_t"=>"usoze",
    "sigset_t *"=>"u64",
    "loff_t "=>"u64",
    "off_t *"=>"*mut u64",
    "unsigned "=>"u64",
    "int *"=>"*mut i32",
    "const char *const *"=>"*const *const u8",
    "pid_t "=>"i32",
    "long "=>"i64",
    "uid_t "=>"u32",
    "gid_t "=>"",
    "gid_t *"=>"missing",
    "siginfo_t *"=>"missing",
    "const struct sigaltstack *"=>"missing",
    "struct sigaltstack *"=>"missing",
    "const char * "=>"missing",
    "struct statfs *"=>"missing",
    "void *"=>"missing",
    "u32 *"=>"missing",
    "u32 "=>"missing",
    "struct timespec *"=>"missing",
    "struct linux_dirent64 *"=>"missing",
    "clockid_t "=>"missing",
    "struct __kernel_timespec *"=>"missing",
    "struct stat *"=>"missing",
    "loff_t *"=>"missing"
]

File.readlines("../zCore/linux-syscall/src/lib.rs").each do |line|
    m = line.match(/^[\s\t]*Sys::(\w+)/)
    if m
        $syscalls[m[1].downcase] = []
    end
end


doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

def parse_type(str)
    name = str.match(/(\w*)$/)[1]
    type = str[0..(str.length - name.length - 1)]
    $rust_type[type] = 'missing'
    [name, type]
end

trs = doc.xpath("//tr")

trs.each do |tr|
    tds = tr.children
    if $syscalls[tds[1].content] != nil
        p tds[1].content
        tds[4..-1].each do |td|
            if td.content != "-"
                $syscalls[tds[1].content] += [parse_type(td.content)]
            end
        end
    end
end



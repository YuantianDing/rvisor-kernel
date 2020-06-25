require 'nokogiri'

$syscalls = Hash[]

$rust_type = Hash[
    "unsigned int "=>"missing",
    "char *"=>"missing",
    "size_t "=>"missing",
    "const char *"=>"missing",
    "int "=>"missing",
    "umode_t "=>"missing",
    "struct __old_kernel_stat *"=>"missing",
    "off_t "=>"missing",
    "?"=>"missing",
    "unsigned long "=>"missing",
    "int"=>"missing",
    "const struct sigaction *"=>"missing",
    "struct sigaction *"=>"missing",
    "size_t"=>"missing",
    "sigset_t *"=>"missing",
    "loff_t "=>"missing",
    "const struct iovec *"=>"missing",
    "struct itimerval *"=>"missing",
    "off_t *"=>"missing",
    "unsigned "=>"missing",
    "int *"=>"missing",
    "const char *const *"=>"missing", "pid_t "=>"missing", "struct rusage *"=>"missing", "struct old_utsname *"=>"missing", "long "=>"missing", "uid_t "=>"missing", "gid_t "=>"missing", "gid_t *"=>"missing", "siginfo_t *"=>"missing", "const struct sigaltstack *"=>"missing", "struct sigaltstack *"=>"missing", "const char * "=>"missing", "struct statfs *"=>"missing", "void *"=>"missing", "u32 *"=>"missing", "u32 "=>"missing", "struct timespec *"=>"missing", "struct linux_dirent64 *"=>"missing", "clockid_t "=>"missing", "struct __kernel_timespec *"=>"missing", "struct stat *"=>"missing", "loff_t *"=>"missing"]

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



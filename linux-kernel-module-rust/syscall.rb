require 'nokogiri'

$syscalls = Hash[]

File.readlines("../zCore/linux-syscall/src/lib.rs").each do |line|
    m = line.match(/^\W*Sys::(\w+)/)
    if m
        $syscalls[m[1].downcase] = []
    end
end


doc = File.open("syscall_table.html") { |f| Nokogiri::XML(f) }

def parse_type(str)
    name = str.match(/(\w*)$/)[1]
    type = str[0..(str.length - name.length - 1)]
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

$rust_type = Hash [
    "unsigned int"
]

def rust_ty()

end
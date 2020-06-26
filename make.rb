require "ruby_make_script"

make do
    :update_raw do
        rm "-rf", "target/debug/build/lkm-*"
        use dir('linux-kernel-module-rust') do
            r "cargo "
        end
    end
end
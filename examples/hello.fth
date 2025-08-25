variable str "Hello world!\0"
variable port 1

: print_str 
    str             \ STACK[addr] 
    begin
        dup         \ STACK[addr; addr]
        @           \ STACK[addr; char] 
        dup         \ STACK[addr; char; char] 
        0 = not     \ STACK[addr; char; flag]
        if 
            port @ out    \ STACK[addr] 
            1 +         \ STACK[addr + 1]
        then
        dup @ 0 =   \ STACK[addr + 1; flag] 
    until            
    drop
    ret
;

print_str
halt


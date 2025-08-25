variable out_port 1 
variable in_port 2

: cat 
    begin 
        in_port @ in 
        dup
        out_port @ out
    0 = until 
    ret
;
cat 
halt
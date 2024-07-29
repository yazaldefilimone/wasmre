(module
  (func $factorial (param $n i32) (result i32)
    (local $result i32)
    (local.set $result (i32.const 1))
    (block $exit
      (loop $loop
        local.get $n
        i32.const 1
        i32.lt_s
        br_if $exit
        local.get $result
        local.get $n
        i32.mul
        local.set $result
        local.get $n
        i32.const 1
        i32.sub
        local.set $n
        br $loop
      )
    )
    local.get $result
  )
  (export "factorial" (func $factorial))
)

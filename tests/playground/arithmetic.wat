(module
  (func $arithmetic (param $x i32) (param $y i32) (result i32)
    (local $result i32)
    ;; addition
    local.get $x
    local.get $y
    i32.add
    local.set $result

    ;; subtraction
    local.get $result
    local.get $y
    i32.sub
    local.set $result

    ;; multiplication
    local.get $result
    local.get $x
    i32.mul
    local.set $result

    ;; division
    local.get $result
    local.get $y
    i32.div_s
    local.set $result

    local.get $result
  )
  (export "arithmetic" (func $arithmetic))
)

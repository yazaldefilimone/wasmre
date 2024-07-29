(module
  (import "env" "external_add" (func $external_add (param i32 i32) (result i32)))
  (func $use_external_add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    call $external_add
  )
  (export "use_external_add" (func $use_external_add))
)

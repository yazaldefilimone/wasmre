(module
  (memory $mem 1)
  (export "memory" (memory $mem))
  (func $store_and_load (param $value i32) (result i32)
    ;; store the value in memory
    local.get $value
    i32.const 0
    i32.store

    ;; load the value from memory
    i32.const 0
    i32.load
  )
  (export "store_and_load" (func $store_and_load))
)

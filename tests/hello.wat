(module
  (func $hello (import "" "hello"))
  (func (export "run") (call $hello))
  (func $get_version (result i32 i32 i32)
      i32.const 1
      i32.const 2
      i32.const 3
    )
    (export "get_version" (func $get_version))
)

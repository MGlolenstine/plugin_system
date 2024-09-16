(module
  (import "env" "send_data" (func $send_data (param i32 i32)))

  (func (export "run")
    (i32.const 8)
    (i32.const 4)
    (call $send_data)
  )

  (func $get_version (result i32 i32 i32)
    i32.const 1
    i32.const 2
    i32.const 3
  )
  (export "get_version" (func $get_version))

  (memory $mem 1) ;; declare a memory with 1 page (64 KiB)
  (export "memory" (memory $mem)) ;; export the memory for host access

  (func $get_bytes (export "get_bytes")
    (result i32 i32) ;; return two i32s (pointer and length)
    ;; Create some data (byte array)
    (i32.const 92) ;; pointer to the start of the byte array
    (i32.const 14) ;; length of the byte array (we'll use 4 bytes)
  )

  ;; Initial memory data (byte array at offset 8)
  (data (i32.const 8) "\01\03\03\07")

  ;; Version tuple
  (data (i32.const 64) "\00\00\01\00\00\00\02\00\00\00\03\00\00\00")

  ;; Hello world return
  (data (i32.const 92) "\01\00\48\65\6c\6c\6f\20\57\6f\72\6c\64\21")

  ;; A global to track the number of calls
  (global $call_counter (mut i32) (i32.const 0))

  ;; Function that checks the call counter and returns different arrays
  (func $get_array (export "get_array") (result i32 i32)
    ;; Load the current value of the global call counter
    (global.get $call_counter)
    
    ;; Check if call_counter == 0 (first call)
    (i32.const 0)
    (i32.eq)
    (if (result i32 i32)
      ;; If true, return the first array (offset 64)
      (then
        ;; Increment call counter
        (global.set $call_counter (i32.const 1))
        (i32.const 64) ;; Return offset of the first array
        (i32.const 14) ;; Return offset of the first array
      )
      ;; Else branch (call_counter != 0)
      (else
        ;; Check if call_counter == 1 (second call)
        (global.get $call_counter)
        (i32.const 1)
        (i32.eq)
        (if (result i32 i32)
          ;; If true, return the second array (offset 92)
          (then
            ;; Increment call counter
            (global.set $call_counter (i32.const 2))
            (i32.const 92) ;; Return offset of the second array
            (i32.const 14) ;; Return offset of the second array
          )
          ;; Else (third or more calls), return something else, for example 0
          (else
            ;; Perform an action after second call
            ;; (In this case, we'll return 0 to signal no array)
            (global.set $call_counter (i32.const 2)) ;; Keep counter at 2
            (i32.const 0) ;; Return 0 to signal no valid array
            (i32.const 0) ;; Return 0 to signal no valid array
          )
        )
      )
    )
  )
)

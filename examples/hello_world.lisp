(import liblisp_hello_world "../builder/lisp-builder-out/liblisp_hello_world.so")

(dylib.call liblisp_hello_world :lisp_hello_world)
(println "Hello, world!")
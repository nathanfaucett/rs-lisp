(def test (fn (x) (if x true false)))

(def a (test true))
(def b (test false))

(println a)
(println b)

(println [{:a a}, {:b b}])

(gc_allocator.collect default_gc_allocator)